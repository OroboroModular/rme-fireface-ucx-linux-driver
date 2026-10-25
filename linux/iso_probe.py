"""Probes the UCX's isochronous audio interface 0 on Linux (re/NOTES.md, "Isochronous streaming").

    python3 linux/iso_probe.py --info
    python3 linux/iso_probe.py [LOG] [--rate 44100] [--seconds 2] [--init] [--stream-start] [--raw FILE]
                               [--out silence|int|float|ramp|none] [--out-layout planar|interleaved|framed]
                               [--frame-word N|pos|pos1|pos5|hdr|hdr1] [--frame-offset N] [--ramp-after N]
                               [--out-header] [--out-slots N] [--sync implicit|nominal]
                               [--loopback 3,4] [--meters] [--trace-playback]

--info only reads (firmware, serial, status word). Otherwise: claims interface 0, optionally sends
the captured power-on writes (--init), selects the alt setting for the rate's speed, writes the
base rate, optionally sends the stream start/stop requests with the playback channel mask
(--stream-start; playback needs it), keeps iso IN transfers on 0x8f and (unless --out none) iso
OUT transfers on 0x0e in flight, and puts the starting base rate back at the end.

The format that plays: --out-layout framed --frame-word pos (each frame led by its position, the
unit's frame time to play it at, plus --frame-offset). The other layouts and lead words are the
ones that were ruled out. --sync implicit sizes each OUT packet from the frame count of an IN
packet, nominal from the rate alone.

OUT carries digital silence, or with --out int slot n carries the 24-bit value n + 1 (left-
justified, about -138 dBFS), tiny whatever the unit takes it for. --out float sends that level as
float32 (the format the unit reads), and --out ramp sends slot n = (n + 1) * 128 + frame position
mod 128 LSB as float32, at most -71 dBFS, optionally after --ramp-after frames of silence. Read
as 24-bit integers, small floats would be a DC step near -8 dBFS, so the float patterns are
inaudible only with the framed float layout. Use them with nothing loud connected to the outputs.

--loopback feeds those outputs back into the recording of the same channels (request 21, all off
again at the end): with --out ramp the analysis then shows, sample by sample, which frames played
and how long after their frame time. --meters reads the level meters (bulk 0x85 on interface 1)
while streaming, --trace-playback lists every playback meter packet.

Prints, and writes to LOG, what the IN packets look like: lengths, the sample rate each
header-size guess implies, the header fields (as a 60 MHz timestamp too) and per-channel levels
(4-byte header, then planar float32). --raw saves every IN packet as <seq u32, status i32,
length u16> followed by its bytes, little-endian.

Uses the system libusb-1.0 through ctypes. Needs no kernel driver on the unit and write access to
its device node (linux/70-fireface-ucx.rules). Writes no mixer registers.
"""

import argparse
import collections
import ctypes as C
import ctypes.util
import math
import os
import struct
import time

VID, PID = 0x2a39, 0x3fc9
AUDIO_IF, MIXER_IF = 0, 1
EP_OUT, EP_IN, EP_METERS = 0x0e, 0x8f, 0x85
CHANNELS = {1: 18, 2: 14, 4: 12}  # per direction, by speed
# alt setting -> (OUT, IN) bytes per microframe (wMaxPacketSize x mult), re/descriptors/2a39_3fc9.txt
MAX_PACKET = {1: (532, 508), 2: (780, 732), 3: (1304, 1208)}
POSITION_WRAP = 3072  # IN header bits 5-16 count frames modulo this
RAMP = 128  # period of the --out ramp pattern, frames
LOOPBACK, LOOPBACK_INDICES = 21, 30  # req 21 wValue 1/0 per index; index i = output i + 1 (AN 1/2 = 0, 1)
POWER_ON = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "crates", "ucx-protocol",
                        "tests", "fixtures", "power_on.txt")

# Vendor requests, as in crates/ucx-protocol/src/control.rs
CONTROL, STATUS, MUTE, UNMUTE, FIRMWARE, STREAM_29, SERIAL_LO, SERIAL_HI = 16, 17, 19, 20, 28, 29, 30, 31
# Status word top byte = (speed << 2) | base
RATE_INDEX = {32000: 0, 44100: 1, 48000: 2, 64000: 4, 88200: 5, 96000: 6, 128000: 8, 176400: 9, 192000: 10}
METER_TAGS = {0x11111111: "inputs", 0x55555555: "inputs (2nd)", 0x22222222: "playback",
              0x33333333: "outputs pre-fader", 0x66666666: "outputs"}

ERROR_ACCESS = -3
TRANSFER_ISO, TRANSFER_BULK = 1, 2
COMPLETED, CANCELLED = 0, 3
STATUS_NAMES = ["completed", "error", "timed out", "cancelled", "stall", "no device", "overflow"]


class DeviceDescriptor(C.Structure):
    _fields_ = [("bLength", C.c_uint8), ("bDescriptorType", C.c_uint8), ("bcdUSB", C.c_uint16),
                ("bDeviceClass", C.c_uint8), ("bDeviceSubClass", C.c_uint8), ("bDeviceProtocol", C.c_uint8),
                ("bMaxPacketSize0", C.c_uint8), ("idVendor", C.c_uint16), ("idProduct", C.c_uint16),
                ("bcdDevice", C.c_uint16), ("iManufacturer", C.c_uint8), ("iProduct", C.c_uint8),
                ("iSerialNumber", C.c_uint8), ("bNumConfigurations", C.c_uint8)]


class IsoPacket(C.Structure):
    _fields_ = [("length", C.c_uint), ("actual_length", C.c_uint), ("status", C.c_int)]


class Timeval(C.Structure):
    _fields_ = [("tv_sec", C.c_long), ("tv_usec", C.c_long)]


CALLBACK = C.CFUNCTYPE(None, C.c_void_p)


def transfer_struct(packets):
    """struct libusb_transfer with its flexible iso_packet_desc array sized to `packets`."""

    class Transfer(C.Structure):
        _fields_ = [("dev_handle", C.c_void_p), ("flags", C.c_uint8), ("endpoint", C.c_uint8),
                    ("type", C.c_uint8), ("timeout", C.c_uint), ("status", C.c_int), ("length", C.c_int),
                    ("actual_length", C.c_int), ("callback", CALLBACK), ("user_data", C.c_void_p),
                    ("buffer", C.c_void_p), ("num_iso_packets", C.c_int), ("iso_packet_desc", IsoPacket * packets)]

    return Transfer


class UsbError(Exception):
    pass


class Libusb:
    SIGNATURES = {
        "init": (C.c_int, [C.POINTER(C.c_void_p)]),
        "exit": (None, [C.c_void_p]),
        "error_name": (C.c_char_p, [C.c_int]),
        "get_device_list": (C.c_ssize_t, [C.c_void_p, C.POINTER(C.POINTER(C.c_void_p))]),
        "free_device_list": (None, [C.POINTER(C.c_void_p), C.c_int]),
        "get_device_descriptor": (C.c_int, [C.c_void_p, C.POINTER(DeviceDescriptor)]),
        "get_bus_number": (C.c_uint8, [C.c_void_p]),
        "get_device_address": (C.c_uint8, [C.c_void_p]),
        "open": (C.c_int, [C.c_void_p, C.POINTER(C.c_void_p)]),
        "close": (None, [C.c_void_p]),
        "kernel_driver_active": (C.c_int, [C.c_void_p, C.c_int]),
        "claim_interface": (C.c_int, [C.c_void_p, C.c_int]),
        "release_interface": (C.c_int, [C.c_void_p, C.c_int]),
        "set_interface_alt_setting": (C.c_int, [C.c_void_p, C.c_int, C.c_int]),
        "control_transfer": (C.c_int, [C.c_void_p, C.c_uint8, C.c_uint8, C.c_uint16, C.c_uint16, C.c_void_p,
                                       C.c_uint16, C.c_uint]),
        "alloc_transfer": (C.c_void_p, [C.c_int]),
        "free_transfer": (None, [C.c_void_p]),
        "submit_transfer": (C.c_int, [C.c_void_p]),
        "cancel_transfer": (C.c_int, [C.c_void_p]),
        "handle_events_timeout": (C.c_int, [C.c_void_p, C.POINTER(Timeval)]),
    }

    def __init__(self):
        lib = C.CDLL(ctypes.util.find_library("usb-1.0") or "libusb-1.0.so.0")
        for name, (res, args) in self.SIGNATURES.items():
            fn = getattr(lib, "libusb_" + name)
            fn.restype, fn.argtypes = res, args
            setattr(self, name, fn)

    def check(self, rc, what):
        if rc < 0:
            raise UsbError(f"{what}: {self.error_name(rc).decode()}")
        return rc


class Ucx:
    """The first 2a39:3fc9 on the bus, opened with libusb."""

    def __init__(self):
        self.usb = usb = Libusb()
        self.ctx = C.c_void_p()
        usb.check(usb.init(C.byref(self.ctx)), "libusb_init")
        devs = C.POINTER(C.c_void_p)()
        n = usb.check(usb.get_device_list(self.ctx, C.byref(devs)), "libusb_get_device_list")
        try:
            for dev in devs[:n]:
                d = DeviceDescriptor()
                if usb.get_device_descriptor(dev, C.byref(d)) == 0 and (d.idVendor, d.idProduct) == (VID, PID):
                    break
            else:
                raise SystemExit("no Fireface UCX (2a39:3fc9) on the bus")
            self.node = f"/dev/bus/usb/{usb.get_bus_number(dev):03}/{usb.get_device_address(dev):03}"
            self.handle = C.c_void_p()
            rc = usb.open(dev, C.byref(self.handle))
        finally:
            usb.free_device_list(devs, 1)
        if rc == ERROR_ACCESS:
            raise SystemExit(f"no write access to {self.node}: install linux/70-fireface-ucx.rules (see the file)")
        usb.check(rc, "libusb_open")
        self.claimed = []

    def read(self, request):
        buf = (C.c_uint8 * 4)()
        n = self.usb.check(self.usb.control_transfer(self.handle, 0xc0, request, 0, 0, buf, 4, 1000),
                           f"read request {request}")
        return bytes(buf[:n])

    def write(self, request, value, index):
        self.usb.check(self.usb.control_transfer(self.handle, 0x40, request, value, index, None, 0, 1000),
                       f"write request {request} {value:04x}/{index:04x}")

    def status(self):
        """Status word and the rate it reports."""
        word = int.from_bytes(self.read(STATUS), "little")
        return word, next((r for r, i in RATE_INDEX.items() if i == word >> 24), None)

    def claim(self, interface):
        if self.usb.kernel_driver_active(self.handle, interface) == 1:
            raise SystemExit(f"a kernel driver has interface {interface}; unbind it first")
        self.usb.check(self.usb.claim_interface(self.handle, interface), f"claim interface {interface}")
        self.claimed.append(interface)

    def set_alt(self, interface, alt):
        self.usb.check(self.usb.set_interface_alt_setting(self.handle, interface, alt),
                       f"interface {interface} alt {alt}")

    def events(self, seconds):
        tv = Timeval(0, int(seconds * 1e6))
        self.usb.handle_events_timeout(self.ctx, C.byref(tv))

    def close(self):
        for i in self.claimed:
            self.usb.release_interface(self.handle, i)
        self.usb.close(self.handle)
        self.usb.exit(self.ctx)


class Stream:
    """Keeps `depth` transfers in flight on one endpoint until cancel(): iso transfers of `packets`
    packets of up to `size` bytes each, or bulk transfers of `size` bytes (packets=0). OUT (iso):
    `produce(n)` gives the next n packet payloads. IN: each packet goes to `received(seq, status,
    data)`."""

    def __init__(self, ucx, endpoint, size, packets=32, depth=8, produce=None, received=None):
        self.ucx, self.endpoint, self.size, self.packets = ucx, endpoint, size, packets
        self.produce, self.received = produce, received
        self.callback = CALLBACK(self._done)
        self.transfers = {}  # libusb_transfer address -> (struct, buffer)
        self.pending = 0
        self.stopping = False
        self.error = None
        self.seq = 0
        Transfer = transfer_struct(packets)
        for _ in range(depth):
            ptr = ucx.usb.alloc_transfer(packets)
            if not ptr:
                raise MemoryError("libusb_alloc_transfer")
            buf = (C.c_uint8 * (size * max(packets, 1)))()
            t = C.cast(ptr, C.POINTER(Transfer)).contents
            t.dev_handle, t.endpoint, t.timeout = ucx.handle, endpoint, 1000
            t.type = TRANSFER_ISO if packets else TRANSFER_BULK
            t.callback, t.buffer, t.num_iso_packets = self.callback, C.addressof(buf), packets
            self.transfers[ptr] = (t, buf)

    def start(self):
        for ptr, (t, _) in self.transfers.items():
            self._submit(ptr, t)

    def _submit(self, ptr, t):
        if not self.packets:
            t.length = self.size
        elif self.produce:
            offset = 0
            for i, payload in enumerate(self.produce(self.packets)):
                payload = payload[:self.size]
                C.memmove(t.buffer + offset, payload, len(payload))
                t.iso_packet_desc[i].length = len(payload)
                offset += len(payload)
            t.length = offset
        else:
            for i in range(self.packets):
                t.iso_packet_desc[i].length = self.size
            t.length = self.size * self.packets
        rc = self.ucx.usb.submit_transfer(ptr)
        if rc < 0:
            self.error = f"submit on {self.endpoint:#04x}: {self.ucx.usb.error_name(rc).decode()}"
        else:
            self.pending += 1

    def _done(self, ptr):
        self.pending -= 1
        try:
            t, _ = self.transfers[ptr]
            if t.status != COMPLETED:
                if t.status != CANCELLED:
                    self.error = f"transfer on {self.endpoint:#04x}: {STATUS_NAMES[t.status]}"
                return
            if self.received and not self.packets:
                self.received(self.seq, t.status, C.string_at(t.buffer, t.actual_length))
                self.seq += 1
            elif self.received:
                offset = 0
                for i in range(self.packets):
                    d = t.iso_packet_desc[i]
                    self.received(self.seq, d.status, C.string_at(t.buffer + offset, d.actual_length))
                    offset += d.length
                    self.seq += 1
            if not self.stopping and self.error is None:
                self._submit(ptr, t)
        except Exception as e:  # ctypes would only print it
            self.error = f"callback on {self.endpoint:#04x}: {e!r}"

    def cancel(self):
        self.stopping = True
        for ptr in self.transfers:
            self.ucx.usb.cancel_transfer(ptr)

    def free(self):
        if not self.pending:
            for ptr in self.transfers:
                self.ucx.usb.free_transfer(ptr)


class OutPackets:
    """OUT packet payloads. Frame counts: those the unit sent on IN (implicit feedback), or the
    nominal rate where there are none yet. Each frame has `slots` 4-byte samples, laid out planar
    (all of slot 0's samples, then slot 1's, ...), interleaved, or framed (interleaved, each frame
    led by the word `frame_word`, the Fireface UC's macOS layout), after an optional 4-byte header."""

    def __init__(self, rate, slots, pattern, layout, header, implicit, frame_word=0, frame_offset=0, ramp_after=0):
        self.rate, self.slots, self.layout, self.header, self.implicit = rate, slots, layout, header, implicit
        self.frame_word, self.frame_offset = frame_word, frame_offset
        self.ramp_after = ramp_after  # frames of silence before the ramp starts
        # 'int': slot n = 24-bit n + 1, left-justified; 'float': the same level as float32
        self.samples = [{"int": (n + 1) << 8, "float": struct.unpack("<I", struct.pack("<f", (n + 1) / 2 ** 23))[0]}
                        .get(pattern, 0) for n in range(slots)]
        # 'ramp': float32, slot n = (n + 1) * 128 + frame position mod 128 LSB (at most -71 dBFS), so
        # a looped-back recording shows which frames arrived. One frame's words per position.
        self.ramp = None
        if pattern == "ramp":
            self.ramp = [[struct.unpack("<I", struct.pack("<f", ((n + 1) * RAMP + p) / 2 ** 23))[0]
                          for n in range(slots)] for p in range(RAMP)]
        self.acc = 0
        self.position = 0
        self.counts = collections.deque()
        self.nominal_used = 0

    def frames(self):
        if self.implicit and self.counts:
            return self.counts.popleft()
        self.acc += self.rate
        frames, self.acc = divmod(self.acc, 8000)
        self.nominal_used += 1
        return frames

    def lead_word(self, frames, i):
        """The word before frame i of a packet in the framed layout. `frame_word` is a number, or:
        'pos' the frame's position mod 3072, 'pos1' that on the first frame only, 'pos5' shifted
        left 5, 'hdr' the packet's IN-style header (frames | first position << 5) on every frame,
        'hdr1' that on the first frame only."""
        position = (self.position + i + self.frame_offset) % POSITION_WRAP
        header = frames | (self.position % POSITION_WRAP) << 5
        return {"pos": position, "pos1": position if i == 0 else 0, "pos5": position << 5, "hdr": header,
                "hdr1": header if i == 0 else 0}.get(self.frame_word, self.frame_word)

    def __call__(self, n):
        payloads = []
        for _ in range(n):
            frames = self.frames()
            if self.ramp:
                silent = [0] * self.slots
                rows = [self.ramp[(self.position + i) % RAMP] if self.position + i >= self.ramp_after else silent
                        for i in range(frames)]
            else:
                rows = [self.samples] * frames
            if self.layout == "planar":
                words = [row[s] for s in range(self.slots) for row in rows]
            elif self.layout == "framed":
                words = [w for i, row in enumerate(rows) for w in [self.lead_word(frames, i)] + row]
            else:
                words = [w for row in rows for w in row]
            if self.header:
                words.insert(0, frames | (self.position % POSITION_WRAP) << 5)
            self.position += frames
            payloads.append(struct.pack(f"<{len(words)}I", *words))
        return payloads


class Meters:
    """Collects the level meter packets from bulk 0x85 per tag. Each holds 20 u64 powers,
    512 x s24^2, and 18 u32 peaks, 16 x |s24| (re/NOTES.md, "Level meters")."""

    def __init__(self, trace=None, settle=0.5):
        self.sums = collections.defaultdict(lambda: [0] * 20)
        self.peaks = collections.defaultdict(lambda: [0] * 18)
        self.counts = collections.Counter()
        self.other = 0
        self.trace = trace  # tag whose packets are listed one by one
        self.traced = []
        self.t0 = time.monotonic()
        # The first reads return packets the unit queued before (an earlier run's levels), and a
        # new stream takes a moment to reach the DSP: only packets after `settle` s count.
        self.settle = settle

    def received(self, seq, status, data):
        if len(data) != 244:
            self.other += 1
            return
        tag = struct.unpack_from("<I", data, 240)[0]
        power = struct.unpack_from("<20Q", data)
        peak = struct.unpack_from("<18I", data, 160)
        if tag == self.trace:
            self.traced.append((time.monotonic() - self.t0, power, peak))
        if time.monotonic() - self.t0 < self.settle:
            return
        for i, p in enumerate(power):
            self.sums[tag][i] += p
        for i, p in enumerate(peak):
            self.peaks[tag][i] = max(self.peaks[tag][i], p)
        self.counts[tag] += 1

    def report(self, say):
        say(f"meters after the first {self.settle} s, mean rms / highest peak in 24-bit LSB per channel "
            "(0 = below range):")
        for tag, name in METER_TAGS.items():
            if self.counts[tag]:
                rms = [math.sqrt(s / self.counts[tag] / 512) for s in self.sums[tag][:18]]
                say(f"  {name:18} " + " ".join(f"{r:.0f}/{p / 16:.0f}" if r >= 0.5 or p else "0"
                                                for r, p in zip(rms, self.peaks[tag])))
        if self.other:
            say(f"  {self.other} packets that weren't 244 bytes")
        if self.traced:
            say(f"{METER_TAGS[self.trace]} meter packets, time, then rms/peak in LSB of channels 1-6:")
            for t, power, peak in self.traced:
                say(f"  {t:6.3f}  " + " ".join(f"{math.sqrt(power[c] / 512):.1f}/{peak[c] / 16:.0f}" for c in range(6)))


def power_on():
    with open(POWER_ON) as f:
        return [tuple(int(x, 16) for x in line.split()) for line in f if line.strip() and not line.startswith("#")]


def analyse(packets, rate, channels, say):
    frame = channels * 4
    total = len(packets)
    statuses = collections.Counter(s for _, s, _ in packets)
    lengths = collections.Counter(len(d) for _, _, d in packets)
    say(f"{total} IN packets ({total / 8000:.3f} s of microframes); status: "
        + ", ".join(f"{STATUS_NAMES[s] if 0 <= s < len(STATUS_NAMES) else s} x{n}" for s, n in statuses.items()))
    say("lengths: " + ", ".join(f"{n} B x{c}" for n, c in sorted(lengths.items())))
    data = [d for _, _, d in packets if d]
    if not data:
        say("no IN data: the unit isn't streaming on interface 0")
        return
    for extra in (0, 4, 8):
        fit = [len(d) - extra for d in data if len(d) >= extra and (len(d) - extra) % frame == 0]
        if fit:
            frames = sum(fit) // frame
            say(f"{extra} extra bytes per packet: {len(fit)}/{len(data)} packets are whole {channels}-slot frames, "
                f"{frames / (total / 8000):.0f} frames/s")
        else:
            say(f"{extra} extra bytes per packet: no packet fits")

    # The layout found at 44.1 kHz: a 4-byte header, then planar float32 (channel 0's samples, ...)
    fit = [d for d in data if (len(d) - 4) % frame == 0]
    if len(fit) < len(data):
        say(f"{len(data) - len(fit)} packets don't fit a 4-byte header: skipping the header and level checks")
        return
    hdr = [(struct.unpack_from("<I", d)[0], (len(d) - 4) // frame) for d in fit]
    say(f"header bits 0-4 = frames in the packet: {sum((h & 31) == f for h, f in hdr)}/{len(hdr)}")
    steps = [((h1 >> 5 & 0xfff) + f1) % POSITION_WRAP == h2 >> 5 & 0xfff for (h1, f1), (h2, _) in zip(hdr, hdr[1:])]
    say(f"header bits 5-16 = frame position mod {POSITION_WRAP}: {sum(steps)}/{len(steps)} steps continue it")
    say(f"header bits 17-20 non-zero in {sum(1 for h, _ in hdr if h >> 17 & 15)} packets")
    deltas = collections.defaultdict(list)
    for (h1, _), (h2, f2) in zip(hdr, hdr[1:]):
        deltas[f2].append(((h2 >> 21) - (h1 >> 21)) % 2048)
    say("header bits 21-31, step to a packet of n frames: "
        + ", ".join(f"n={n}: {min(v)}-{max(v)}" for n, v in sorted(deltas.items())))
    # Bits 21-31 as a 60 MHz timestamp mod 2048 of the end of the packet's frames, against the
    # microframe clock (7500 ticks): what's left over is the sample clock's drift.
    ticks, end, residual = 60e6 / rate, 0, []
    for seq, _, d in (p for p in packets if p[2]):
        f = (len(d) - 4) // frame
        end += f
        if f:
            r = ((struct.unpack_from("<I", d)[0] >> 21) - 7500 * seq + end * ticks) % 2048
            residual.append(r if not residual else residual[-1] + (r - residual[-1] + 1024) % 2048 - 1024)
    steps = [abs(b - a) for a, b in zip(residual, residual[1:])]
    if steps:
        drift = (residual[-1] - residual[0]) / ((len(residual) - 1) / 8000) / 60
        say(f"as a 60 MHz timestamp of the packet's end: jitter up to {max(steps):.1f} ticks, "
            f"sample clock {drift:+.1f} ppm against the USB clock")

    sums, peaks, count = [0.0] * channels, [0.0] * channels, 0
    for d in fit:
        n = (len(d) - 4) // frame
        samples = struct.unpack_from(f"<{n * channels}f", d, 4)
        for c in range(channels):
            for x in samples[c * n:(c + 1) * n]:
                sums[c] += x * x
                peaks[c] = max(peaks[c], abs(x))
        count += n
    say("channels as planar float32, rms in 24-bit LSB / peak dBFS:")
    say("  " + "  ".join(f"{c + 1}: {math.sqrt(sums[c] / count) * 2 ** 23:.0f}/"
                         f"{20 * math.log10(peaks[c]) if peaks[c] else -math.inf:.0f}" for c in range(channels)))


def analyse_ramp(packets, channels, ramp_after, say):
    """Find the --out ramp pattern in the recorded channels (with --loopback) and check that its
    frame positions follow each other. With silence first (ramp_after), the ramp's start after the
    silence gives the delay from a frame's position to when it's recorded."""
    frame = channels * 4
    recorded = [[] for _ in range(channels)]
    for _, _, d in packets:
        if len(d) > 4 and (len(d) - 4) % frame == 0:
            n = (len(d) - 4) // frame
            samples = struct.unpack_from(f"<{n * channels}f", d, 4)
            for c in range(channels):
                recorded[c] += [round(x * 2 ** 23) for x in samples[c * n:(c + 1) * n]]
    is_ramp = lambda v: RAMP <= v < (channels + 1) * RAMP  # noqa: E731
    for c, values in enumerate(recorded):
        ramp = [v for v in values if is_ramp(v)]
        sources = collections.Counter(v // RAMP for v in ramp)
        # Inputs settling after a speed change also reach ramp levels, but spread over every source.
        if len(ramp) < len(values) // 50 or sources.most_common(1)[0][1] < 0.9 * len(ramp):
            continue
        values = [v if is_ramp(v) or v == 0 else -1 for v in values]
        steps = collections.Counter((b % RAMP - a % RAMP) % RAMP for a, b in zip(values, values[1:])
                                    if a >= RAMP and b >= RAMP)
        zeros = sum(1 for v in values if v == 0)
        say(f"recorded channel {c + 1}: {len(ramp)}/{len(values)} samples carry the ramp, {zeros} are 0; "
            "from playback " + ", ".join(f"{s}: {n}" for s, n in sources.most_common(4))
            + "; position steps " + ", ".join(f"{s}: {n}" for s, n in steps.most_common(6)))
        first = next(i for i, v in enumerate(values) if v >= RAMP)
        say(f"  first ramp sample is recorded frame {first} and carries position {values[first] % RAMP} (mod {RAMP})")
        if ramp_after:
            # Before the silence the unit may replay what its buffer still holds from an earlier run,
            # so take the last start of the ramp after 64 silent frames.
            start = next((i for i in range(len(values) - 1, 63, -1) if values[i] >= RAMP and not any(values[i - 64:i])),
                         None)
            if start is None:
                say("  the ramp never starts after a stretch of silence")
            else:
                say(f"  after the silence the ramp starts at recorded frame {start} with position "
                    f"{values[start] % RAMP} (mod {RAMP}): frame {ramp_after} plays {start - ramp_after} frames "
                    "after its position")
                first = start
        say("  from there: " + " ".join(
            f"{v // RAMP}.{v % RAMP}" if v >= RAMP else str(v) for v in values[first:first + 36]))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("log", nargs="?", help="also write the summary here")
    ap.add_argument("--info", action="store_true", help="only read firmware, serial and status")
    ap.add_argument("--rate", type=int, default=44100, choices=sorted(RATE_INDEX))
    ap.add_argument("--seconds", type=float, default=2.0)
    ap.add_argument("--init", action="store_true", help="send the captured power-on writes first")
    ap.add_argument("--stream-start", action="store_true", help="send the Windows stream start/stop requests")
    ap.add_argument("--out", choices=["silence", "int", "float", "ramp", "none"], default="silence")
    ap.add_argument("--loopback", help="outputs to loop back into the recording, e.g. 3,4 (all off again at the end)")
    ap.add_argument("--out-layout", choices=["planar", "interleaved", "framed"], default="planar")
    ap.add_argument("--frame-word", type=lambda s: s if s in ("pos", "pos1", "pos5", "hdr", "hdr1") else int(s, 0),
                    default=0, help="with --out-layout framed: a number, pos, pos1, pos5, hdr or hdr1 (see OutPackets)")
    ap.add_argument("--frame-offset", type=int, default=0, help="added to the positions --frame-word pos sends")
    ap.add_argument("--ramp-after", type=int, default=0, help="--out ramp: send silence for this many frames first")
    ap.add_argument("--out-header", action="store_true", help="start each OUT packet with a 4-byte header")
    ap.add_argument("--out-slots", type=int, help="4-byte slots per OUT frame (default: the channel count)")
    ap.add_argument("--sync", choices=["implicit", "nominal"], default="implicit")
    ap.add_argument("--meters", action="store_true", help="read the level meters while streaming")
    ap.add_argument("--trace-playback", action="store_true", help="with --meters: list every playback meter packet")
    ap.add_argument("--raw", help="save every IN packet here")
    args = ap.parse_args()

    lines = []

    def say(s=""):
        print(s)
        lines.append(s)

    ucx = Ucx()
    try:
        word, start_rate = ucx.status()
        say(f"Fireface UCX at {ucx.node}: firmware {ucx.read(FIRMWARE).hex(' ')}, serial words "
            f"{ucx.read(SERIAL_LO).hex(' ')} / {ucx.read(SERIAL_HI).hex(' ')}, status {word:08x} ({start_rate} Hz)")
        if args.info:
            return

        index = RATE_INDEX[args.rate]
        speed, base = (1, 2, 4)[index >> 2], index & 3
        alt, channels = {1: 1, 2: 2, 4: 3}[speed], CHANNELS[speed]
        out_max, in_max = MAX_PACKET[alt]
        ucx.claim(AUDIO_IF)
        if args.init:
            writes = power_on()
            for request, value, windex in writes:
                ucx.write(request, value, windex)
            say(f"sent the {len(writes)} power-on writes")
        ucx.set_alt(AUDIO_IF, alt)
        ucx.write(CONTROL, base << 4, 0x0030)
        word, rate = ucx.status()
        say(f"interface 0 alt {alt}, base rate {base}: status {word:08x} ({rate} Hz)")
        # Requests 19/20 take a playback channel mask, wValue:wIndex, channel n at bit n + 2: the
        # Windows driver sent 0000/000c for a 2-channel stream and 000f/fffc for all 18.
        mask = ((1 << channels) - 1) << 2
        if args.stream_start:
            ucx.write(CONTROL, 0x8000, 0x8000)
            ucx.write(STREAM_29, 0, 0)
            ucx.write(UNMUTE, mask >> 16, mask & 0xffff)
            say(f"sent stream start (16 8000/8000, 29, 20 {mask >> 16:04x}/{mask & 0xffff:04x})")
        looped = [int(x) for x in args.loopback.split(",")] if args.loopback else []
        if looped:
            for i in range(LOOPBACK_INDICES):
                ucx.write(LOOPBACK, int(i + 1 in looped), i)
            say(f"loopback on for outputs {args.loopback}")

        out = OutPackets(args.rate, args.out_slots or channels, args.out, args.out_layout, args.out_header,
                         args.sync == "implicit", args.frame_word, args.frame_offset, args.ramp_after)
        packets = []

        def received(seq, status, data):
            packets.append((seq, status, data))
            out.counts.append(max(len(data) - 4, 0) // (channels * 4))

        streams = [Stream(ucx, EP_IN, in_max, received=received)]
        if args.out != "none":
            streams.append(Stream(ucx, EP_OUT, out_max, produce=out))
            say(f"OUT: {args.out}, {out.slots} slots per frame, {args.out_layout}, "
                f"{'4-byte header, ' if args.out_header else ''}{args.sync} packet sizes")
        meters = Meters(trace=0x22222222 if args.trace_playback else None)
        if args.meters:
            ucx.claim(MIXER_IF)
            streams.append(Stream(ucx, EP_METERS, 512, packets=0, depth=4, received=meters.received))
        for s in streams:
            s.start()
        deadline = time.monotonic() + args.seconds
        try:
            while time.monotonic() < deadline and not any(s.error for s in streams):
                ucx.events(0.05)
        except KeyboardInterrupt:
            say("interrupted")
        for s in streams:
            s.cancel()
        stop_by = time.monotonic() + 2
        while any(s.pending for s in streams) and time.monotonic() < stop_by:
            ucx.events(0.05)
        for s in streams:
            if s.error:
                say(f"error: {s.error}")
            s.free()
        if args.out != "none" and out.implicit:
            say(f"OUT packets sized from the nominal rate: {out.nominal_used} (no IN count yet)")

        if looped:
            for i in range(LOOPBACK_INDICES):
                ucx.write(LOOPBACK, 0, i)
            say("loopback off for all outputs (the power-on state)")
        if args.stream_start:
            ucx.write(MUTE, mask >> 16, mask & 0xffff)
        ucx.set_alt(AUDIO_IF, 0)
        if start_rate:
            ucx.write(CONTROL, (RATE_INDEX[start_rate] & 3) << 4, 0x0030)
        word, rate = ucx.status()
        say(f"stopped, back to alt 0 and the starting base rate: status {word:08x} ({rate} Hz)")

        analyse(packets, args.rate, channels, say)
        if args.out == "ramp":
            analyse_ramp(packets, channels, args.ramp_after, say)
        if args.meters:
            meters.report(say)
        if args.raw:
            with open(args.raw, "wb") as f:
                for seq, status, data in packets:
                    f.write(struct.pack("<IiH", seq, status, len(data)) + data)
            say(f"saved {len(packets)} IN packets to {args.raw}")
    finally:
        ucx.close()
        if args.log:
            with open(args.log, "w") as f:
                f.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
