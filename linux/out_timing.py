"""Measures how far ahead of the unit's own frame clock an OUT packet has to arrive
(linux/DRIVER_PLAN.md, open question 1).

    python3 linux/out_timing.py [LOG] [--rate 44100] [--runs 3] [--sweep=-48:176:8] [--segment-ms 50] [--asap]

Each run streams IN and OUT on interface 0 through usbfs (linux/usbfs.py), with loopback on for
outputs 3/4. IN starts first and OUT 50 ms later, both without ISO_ASAP unless --asap. The reap
times of the first 200 OUT URBs, against those of the IN URBs reaped meanwhile, give the
streams' offset s in microframes: OUT packet k goes out in the same microframe as IN packet
k + s, whose header gives the unit's frame position A there (the position just after its
frames). Each OUT packet's first lead word L is then set to A + D, with D stepping through
--sweep, one step per --segment-ms, and packet sizes following so that the lead words run on
without gaps. The offset is taken again per 100 URBs over the run, as a check.

Playback 3 and 4 carry each frame's position q as 128 + q mod 128 and 128 + (q >> 7) mod 128
LSB (below -90 dBFS, the other channels silent), so the recording shows frame by frame which
position played, and when. Afterwards every frame gets q - A from the unit's real position (not
the prediction the lead word came from) and counts as on time, a ring (128 frames) late or
early, or missing (never recorded); the report merges runs of offsets with one outcome. Whole
packets give the range of L - A that a driver can use, if it comes out the same in every run.

Writes no mixer registers. Puts the starting base rate back, turns loopback off for all outputs
and leaves interface 0 at alt 0.
"""

import argparse
import collections
import struct
import time

from iso_probe import (AUDIO_IF, CHANNELS, CONTROL, EP_IN, EP_OUT, FIRMWARE, LOOPBACK, LOOPBACK_INDICES,
                       MAX_PACKET, MUTE, POSITION_WRAP, RATE_INDEX, STATUS, STREAM_29, UNMUTE)
from usbfs import IsoUrb, Ucx

PACKETS = 8  # per URB: one ms
DEPTH = 16  # URBs in flight per direction
OUT_AFTER_MS = 50  # IN runs alone this long first
CALIBRATION_URBS = 200  # OUT URBs reaped before the offset is taken
TAIL_MS = 30  # silence after the sweep
UFRAME_NS = 125000
CODE = 1 << 14  # positions the two 7-bit digits on playback 3/4 tell apart
LOOPED = (3, 4)
LSB = [struct.pack("<f", v / 2 ** 23) for v in range(256)]


def rate_status(ucx):
    word = int.from_bytes(ucx.read(STATUS), "little")
    return word, next((r for r, i in RATE_INDEX.items() if i == word >> 24), None)


class Run:
    def __init__(self, ucx, rate, sweep, segment_ms, asap, say):
        self.ucx, self.say = ucx, say
        index = RATE_INDEX[rate]
        speed = (1, 2, 4)[index >> 2]
        self.alt, self.channels = {1: 1, 2: 2, 4: 3}[speed], CHANNELS[speed]
        self.base = index & 3
        self.out_max, self.in_max = MAX_PACKET[self.alt]
        self.max_frames = self.out_max // (4 * (self.channels + 1))
        self.fpu = rate / 8000  # frames per microframe, refined from IN
        self.sweep, self.segment = sweep, segment_ms * 8
        self.asap = asap
        self.A = {}  # IN packet index -> unit position just after its frames (unwrapped)
        self.first = {}  # IN packet index -> (position of its first frame, frames)
        self.data = {}  # IN packet index -> payload
        self.in_status = collections.Counter()
        self.out_status = collections.Counter()
        self.last = None  # (index, A) of the newest IN packet with a header
        self.anchor = None  # (index, A) of the first IN packet with frames, for fpu
        self.in_reaps, self.out_reaps = [], []  # (first packet index, reaped_at ns)
        self.out = {}  # OUT packet index -> (L, N, D); D None while silent
        self.s = None  # OUT packet k goes out with IN packet k + s
        self.s_guess = 0
        self.lead = None  # lead position of the next OUT packet
        self.sweep_from = None  # first OUT packet index of the sweep
        self.prev_d = None

    # IN

    def take(self, i, status, data):
        self.in_status[status] += 1
        if status or len(data) < 4:
            return
        header = struct.unpack_from("<I", data)[0]
        frames, pos = header & 31, header >> 5 & 0xfff
        if self.last is None:
            first = pos
        else:
            li, la = self.last
            expect = round(la + (i - li - 1) * self.fpu)
            delta = (pos - expect) % POSITION_WRAP
            first = expect + (delta - POSITION_WRAP if delta >= POSITION_WRAP // 2 else delta)
        self.A[i] = first + frames
        self.first[i] = (first, frames)
        self.data[i] = data
        self.last = (i, first + frames)
        if frames and self.anchor is None:
            self.anchor = (i, first)
        elif self.anchor and i - self.anchor[0] >= 400:
            self.fpu = (first + frames - self.anchor[1]) / (i + 1 - self.anchor[0])

    def predict(self, i):
        if i in self.A:
            return self.A[i]
        li, la = self.last
        return la + round((i - li) * self.fpu)

    # OUT

    def segment_d(self, k):
        if self.sweep_from is None or k < self.sweep_from:
            return None
        n = (k - self.sweep_from) // self.segment
        return self.sweep[n] if n < len(self.sweep) else None

    def build(self, urb, k0):
        silent = b"\0" * (4 * self.channels)
        before, after = b"\0" * (4 * (LOOPED[0] - 1)), b"\0" * (4 * (self.channels - LOOPED[1]))
        s = self.s if self.s is not None else self.s_guess
        payloads = []
        for k in range(k0, k0 + PACKETS):
            d = self.segment_d(k)
            offset = 64 if d is None else d
            target = self.predict(k + s) + offset
            if self.lead is None or d != self.prev_d or abs(self.lead - target) > self.max_frames:
                self.lead = target
            self.prev_d = d
            n = max(0, min(self.max_frames, self.predict(k + 1 + s) + offset - self.lead))
            frames = []
            for q in range(self.lead, self.lead + n):
                body = silent if d is None else before + LSB[128 + q % 128] + LSB[128 + (q >> 7) % 128] + after
                frames.append(struct.pack("<I", q % POSITION_WRAP) + body)
            payloads.append(b"".join(frames))
            self.out[k] = (self.lead, n, d)
            self.lead += n
        urb.base = k0
        urb.fill(payloads)

    def offset(self, t0, t1):
        """s from the reap times between t0 and t1 (ns): each URB is reaped some time after its
        last microframe, never before, so the earliest reap against each stream's own microframe
        count lines the two up. Short stretches keep the drift between the host's clock and the
        USB clock small. Also gives how late the 10th percentile of reaps is, in us."""
        e_out = [t - (b + PACKETS) * UFRAME_NS for b, t in self.out_reaps if t0 <= t < t1]
        e_in = [t - (b + PACKETS) * UFRAME_NS for b, t in self.in_reaps if t0 <= t < t1]
        lag = lambda e: sorted(x - min(e) for x in e)[len(e) // 10] / 1000  # noqa: E731
        return (min(e_out) - min(e_in)) / UFRAME_NS, lag(e_in), lag(e_out), len(e_in), len(e_out)

    def estimate_offset(self):
        s, lag_in, lag_out, n_in, n_out = self.offset(self.out_reaps[0][1], self.out_reaps[-1][1] + 1)
        self.say(f"  offset from {n_out} OUT and {n_in} IN reaps: OUT packet k goes out with IN packet "
                 f"k + {s:.3f} (guessed {self.s_guess}); 10% of reaps come more than {lag_in:.0f} / "
                 f"{lag_out:.0f} us after the earliest (IN / OUT)")
        return round(s)

    # the run

    def go(self):
        ucx, say = self.ucx, self.say
        ucx.set_alt(AUDIO_IF, self.alt)
        ucx.write(CONTROL, self.base << 4, 0x0030)
        mask = ((1 << self.channels) - 1) << 2
        ucx.write(CONTROL, 0x8000, 0x8000)
        ucx.write(STREAM_29, 0, 0)
        ucx.write(UNMUTE, mask >> 16, mask & 0xffff)
        in_urbs = [IsoUrb(EP_IN, PACKETS, self.in_max, self.asap) for _ in range(DEPTH)]
        out_urbs = [IsoUrb(EP_OUT, PACKETS, self.out_max, self.asap) for _ in range(DEPTH)]
        next_in = next_out = 0
        for u in in_urbs:
            u.expect()
            u.base, next_in = next_in, next_in + PACKETS
            ucx.submit(u)
        start = time.monotonic()
        end_k = None
        try:
            while True:
                for urb in ucx.reap(5):
                    if urb.endpoint == EP_IN:
                        self.in_reaps.append((urb.base, urb.reaped_at))
                        for n, (status, data) in enumerate(urb.received()):
                            self.take(urb.base + n, status, data)
                        urb.base, next_in = next_in, next_in + PACKETS
                        ucx.submit(urb)
                    else:
                        self.out_reaps.append((urb.base, urb.reaped_at))
                        for d in urb.urb.iso_frame_desc:
                            self.out_status[d.status] += 1
                        if end_k is None or next_out < end_k:
                            self.build(urb, next_out)
                            next_out += PACKETS
                            ucx.submit(urb)
                if next_out == 0 and time.monotonic() - start > OUT_AFTER_MS / 1000 and self.last:
                    b, t = self.in_reaps[-1]
                    now = (time.monotonic_ns() - t) // UFRAME_NS + b + PACKETS
                    self.s_guess = now + 16  # IST and the round-up to a frame, roughly
                    for u in out_urbs:
                        self.build(u, next_out)
                        next_out += PACKETS
                        ucx.submit(u)
                if self.s is None and len(self.out_reaps) >= CALIBRATION_URBS:
                    self.s = self.estimate_offset()
                    self.sweep_from = next_out
                    end_k = next_out + len(self.sweep) * self.segment + TAIL_MS * 8
                if end_k is not None and not any(u.endpoint == EP_OUT for u in ucx.pending.values()):
                    break
                if time.monotonic() - start > 30:
                    say("  gave up: the run took more than 30 s")
                    break
        finally:
            ucx.discard_all()
            ucx.write(MUTE, mask >> 16, mask & 0xffff)
            ucx.set_alt(AUDIO_IF, 0)

    # analysis

    def heard(self):
        """Recorded position -> (delay, frame position q it carried), from playback 3/4 looped back."""
        size = 4 * self.channels
        heard = {}
        for i, data in self.data.items():
            first, n = self.first[i]
            if not n or len(data) != 4 + n * size:
                continue
            lo = struct.unpack_from(f"<{n}f", data, 4 + 4 * n * (LOOPED[0] - 1))
            hi = struct.unpack_from(f"<{n}f", data, 4 + 4 * n * (LOOPED[1] - 1))
            for f in range(n):
                a, b = round(lo[f] * 2 ** 23), round(hi[f] * 2 ** 23)
                if 128 <= a < 256 and 128 <= b < 256:
                    p = first + f
                    delay = (p - ((b - 128) << 7 | (a - 128))) % CODE
                    delay -= CODE if delay >= CODE // 2 else 0
                    heard[p - delay] = delay
        return heard

    def report(self):
        say = self.say
        say("  IN packet status: " + ", ".join(f"{s}: {n}" for s, n in sorted(self.in_status.items()))
            + "; OUT: " + ", ".join(f"{s}: {n}" for s, n in sorted(self.out_status.items())))
        idx = sorted(self.first)
        gaps = sum(1 for a, b in zip(idx, idx[1:]) if b != a + 1 or self.first[b][0] != self.A[a])
        say(f"  IN: {len(idx)} packets with a header, {gaps} breaks in index or position; "
            f"{self.fpu * 8000:.1f} frames/s")
        t = [t for _, t in self.out_reaps]
        chunks = [self.offset(t[i], t[min(i + 100, len(t) - 1)])[0] for i in range(0, len(t) - 50, 100)]
        say(f"  offset again per 100 OUT reaps over the run: {min(chunks):.3f} to {max(chunks):.3f}")
        heard = self.heard()
        if not heard:
            say("  nothing of the pattern came back")
            return None
        lat = collections.Counter(heard.values()).most_common(1)[0][0]
        say(f"  {len(heard)} frames of the pattern recorded; most often {lat} frames after their position")
        outcome = {lat: "on time", lat + 128: "a ring late", lat - 128: "a ring early", None: "missing"}
        by_o = collections.defaultdict(collections.Counter)  # frame q - A -> outcomes
        by_d = collections.defaultdict(collections.Counter)  # packet's lead word L - A -> outcomes
        for k, (lead, n, d) in self.out.items():
            if d is None or not n or k + self.s not in self.A:
                continue
            a = self.A[k + self.s]
            got = [outcome.get(heard.get(q), "elsewhere") for q in range(lead, lead + n)]
            for q, g in zip(range(lead, lead + n), got):
                by_o[q - a][g] += 1
            by_d[lead - a][got[0] if len(set(got)) == 1 else "mixed"] += 1
        say("  each frame by its position minus the unit's position at the packet's microframe:")
        for lo, hi, counts in runs(by_o):
            say(f"    {lo:4} to {hi:4}: " + ", ".join(f"{g} {n}" for g, n in counts.most_common()))
        ok = [d for d, c in by_d.items() if c["on time"] == sum(c.values())]
        window = (min(ok), max(ok)) if ok else None
        say(f"  whole packets on time for lead word - position = {window[0]} to {window[1]}" if window else
            "  no packet came through whole")
        return window


def runs(by_key):
    """Consecutive keys whose counters have the same single outcome merge into one row."""
    rows = []
    for key in sorted(by_key):
        c = by_key[key]
        if rows and len(c) == 1 and len(rows[-1][2]) == 1 and c.keys() == rows[-1][2].keys() and rows[-1][1] == key - 1:
            rows[-1][1] = key
            rows[-1][2].update(c)
        else:
            rows.append([key, key, collections.Counter(c)])
    return rows


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("log", nargs="?", help="also write the summary here")
    ap.add_argument("--rate", type=int, default=44100, choices=sorted(RATE_INDEX))
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--sweep", default="-48:176:8", help="D values, start:stop:step (stop excluded)")
    ap.add_argument("--segment-ms", type=int, default=50)
    ap.add_argument("--asap", action="store_true", help="submit with ISO_ASAP, as libusb does")
    args = ap.parse_args()
    sweep = list(range(*(int(x) for x in args.sweep.split(":"))))

    lines = []

    def say(s=""):
        print(s, flush=True)
        lines.append(s)

    ucx = Ucx()
    start_rate = None
    try:
        word, start_rate = rate_status(ucx)
        say(f"Fireface UCX at {ucx.node}: firmware {ucx.read(FIRMWARE).hex(' ')}, status {word:08x} ({start_rate} Hz)")
        say(f"{args.rate} Hz, D from {sweep[0]} to {sweep[-1]} step {sweep[1] - sweep[0] if len(sweep) > 1 else 0}, "
            f"{args.segment_ms} ms each, {'with' if args.asap else 'without'} ISO_ASAP")
        ucx.claim(AUDIO_IF)
        for i in range(LOOPBACK_INDICES):
            ucx.write(LOOPBACK, int(i + 1 in LOOPED), i)
        windows, offsets = [], []
        for n in range(args.runs):
            say(f"run {n + 1}:")
            run = Run(ucx, args.rate, sweep, args.segment_ms, args.asap, say)
            run.go()
            if run.s is None:
                say("  no offset: OUT never got going")
                continue
            offsets.append(run.s)
            windows.append(run.report())
        say(f"offsets (microframes): {offsets}, mod 8: {[s % 8 for s in offsets]}")
        say(f"on-time windows of d: {windows}")
    finally:
        try:
            for i in range(LOOPBACK_INDICES):
                ucx.write(LOOPBACK, 0, i)
            ucx.set_alt(AUDIO_IF, 0)
            if start_rate:
                ucx.write(CONTROL, (RATE_INDEX[start_rate] & 3) << 4, 0x0030)
            word, rate = rate_status(ucx)
            say(f"loopback off, alt 0, status {word:08x} ({rate} Hz)")
        finally:
            ucx.close()
            if args.log:
                with open(args.log, "w") as f:
                    f.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
