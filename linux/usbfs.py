"""The UCX through Linux usbfs directly (ioctls on its device node), for what libusb doesn't offer.

libusb submits every iso transfer with USBDEVFS_URB_ISO_ASAP: the host controller starts the
stream whenever it can. Without that flag the xHCI driver starts a stream on a USB frame boundary
(drivers/usb/host/xhci-ring.c, xhci_get_isoc_start_frame), so streams started together differ by
whole frames. usbfs doesn't report the frame a URB went out in (processcompl() copies back status
and lengths only), so each URB here records when it was reaped instead.

Same control calls as iso_probe.Ucx. Needs write access to the device node
(linux/70-fireface-ucx.rules) and no kernel driver on the interfaces it claims.
"""

import ctypes as C
import errno
import glob
import os
import select
import time

VID, PID = 0x2a39, 0x3fc9
URB_TYPE_ISO = 0
URB_ISO_ASAP = 0x02


class CtrlTransfer(C.Structure):
    _fields_ = [("bRequestType", C.c_uint8), ("bRequest", C.c_uint8), ("wValue", C.c_uint16),
                ("wIndex", C.c_uint16), ("wLength", C.c_uint16), ("timeout", C.c_uint32), ("data", C.c_void_p)]


class SetInterface(C.Structure):
    _fields_ = [("interface", C.c_uint), ("altsetting", C.c_uint)]


class IsoPacket(C.Structure):
    _fields_ = [("length", C.c_uint), ("actual_length", C.c_uint), ("status", C.c_int)]


def urb_struct(packets):
    """struct usbdevfs_urb with its iso_frame_desc array sized to `packets`."""

    class Urb(C.Structure):
        _fields_ = [("type", C.c_uint8), ("endpoint", C.c_uint8), ("status", C.c_int), ("flags", C.c_uint),
                    ("buffer", C.c_void_p), ("buffer_length", C.c_int), ("actual_length", C.c_int),
                    ("start_frame", C.c_int), ("number_of_packets", C.c_int), ("error_count", C.c_int),
                    ("signr", C.c_uint), ("usercontext", C.c_void_p), ("iso_frame_desc", IsoPacket * packets)]

    return Urb


def _ioc(direction, nr, size):  # asm-generic/ioctl.h: _IOC_WRITE = 1, _IOC_READ = 2
    return direction << 30 | size << 16 | ord("U") << 8 | nr


CONTROL = _ioc(3, 0, C.sizeof(CtrlTransfer))
SETINTERFACE = _ioc(2, 4, C.sizeof(SetInterface))
SUBMITURB = _ioc(2, 10, C.sizeof(urb_struct(0)))
DISCARDURB = _ioc(0, 11, 0)
REAPURBNDELAY = _ioc(1, 13, C.sizeof(C.c_void_p))
CLAIMINTERFACE = _ioc(2, 15, C.sizeof(C.c_uint))
RELEASEINTERFACE = _ioc(2, 16, C.sizeof(C.c_uint))

# The URB's address goes to the kernel, which writes the results back there on completion, so
# ioctls go through libc with ctypes pointers (fcntl.ioctl would pass a temporary copy).
_libc = C.CDLL(None, use_errno=True)
_ioctl = _libc.ioctl
_ioctl.restype, _ioctl.argtypes = C.c_int, [C.c_int, C.c_ulong, C.c_void_p]


def find_node(vid=VID, pid=PID):
    for d in glob.glob("/sys/bus/usb/devices/*"):
        try:
            with open(d + "/idVendor") as f, open(d + "/idProduct") as g:
                if (int(f.read(), 16), int(g.read(), 16)) != (vid, pid):
                    continue
            with open(d + "/busnum") as f, open(d + "/devnum") as g:
                return f"/dev/bus/usb/{int(f.read()):03}/{int(g.read()):03}"
        except (OSError, ValueError):
            continue
    raise SystemExit(f"no {vid:04x}:{pid:04x} on the bus")


class IsoUrb:
    """One iso URB of `packets` packets of up to `size` bytes each, with its buffer. After a
    reap, `reaped_at` is time.monotonic_ns() right after the kernel handed it back."""

    def __init__(self, endpoint, packets, size, asap=False):
        self.urb = urb_struct(packets)()
        self.buf = (C.c_uint8 * (packets * size))()
        self.endpoint, self.packets, self.size = endpoint, packets, size
        self.urb.type, self.urb.endpoint = URB_TYPE_ISO, endpoint
        self.urb.flags = URB_ISO_ASAP if asap else 0
        self.urb.buffer, self.urb.number_of_packets = C.addressof(self.buf), packets
        self.address = C.addressof(self.urb)
        self.reaped_at = None
        self.base = 0  # caller's index of the first packet

    def fill(self, payloads):
        """OUT: the packets' payloads, back to back."""
        offset = 0
        for i, p in enumerate(payloads):
            C.memmove(C.addressof(self.buf) + offset, p, len(p))
            self.urb.iso_frame_desc[i].length = len(p)
            offset += len(p)
        self.urb.buffer_length = offset

    def expect(self):
        """IN: room for a full packet in every slot."""
        for i in range(self.packets):
            self.urb.iso_frame_desc[i].length = self.size
        self.urb.buffer_length = self.packets * self.size

    def received(self):
        """IN, after a reap: (status, data) per packet. Packet i sits at i * size."""
        base = C.addressof(self.buf)
        return [(d.status, C.string_at(base + i * self.size, d.actual_length))
                for i, d in enumerate(self.urb.iso_frame_desc)]


class Ucx:
    """The first 2a39:3fc9 on the bus, opened for usbfs ioctls."""

    def __init__(self):
        self.node = find_node()
        try:
            self.fd = os.open(self.node, os.O_RDWR | os.O_CLOEXEC)
        except PermissionError:
            raise SystemExit(f"no write access to {self.node}: install linux/70-fireface-ucx.rules (see the file)")
        self.claimed = []
        self.pending = {}  # URB address -> IsoUrb, submitted and not reaped yet
        self.poller = select.poll()
        self.poller.register(self.fd, select.POLLOUT)

    def _ioctl(self, request, arg, what):
        rc = _ioctl(self.fd, request, arg)
        if rc < 0:
            e = C.get_errno()
            raise OSError(e, f"{what}: {os.strerror(e)}")
        return rc

    def control(self, request_type, request, value, index, length=0):
        buf = (C.c_uint8 * max(length, 1))()
        ct = CtrlTransfer(request_type, request, value, index, length, 1000, C.addressof(buf))
        n = self._ioctl(CONTROL, C.byref(ct), f"request {request} {value:04x}/{index:04x}")
        return bytes(buf[:n])

    def read(self, request):
        return self.control(0xc0, request, 0, 0, 4)

    def write(self, request, value, index):
        self.control(0x40, request, value, index)

    def claim(self, interface):
        self._ioctl(CLAIMINTERFACE, C.byref(C.c_uint(interface)), f"claim interface {interface}")
        self.claimed.append(interface)

    def set_alt(self, interface, alt):
        self._ioctl(SETINTERFACE, C.byref(SetInterface(interface, alt)), f"interface {interface} alt {alt}")

    def submit(self, urb):
        self._ioctl(SUBMITURB, urb.address, f"submit on {urb.endpoint:#04x}")
        self.pending[urb.address] = urb

    def reap(self, timeout_ms):
        """The URBs that have completed, waiting up to timeout_ms for the first."""
        done = []
        if not self.poller.poll(timeout_ms):
            return done
        while True:
            p = C.c_void_p()
            if _ioctl(self.fd, REAPURBNDELAY, C.byref(p)) < 0:
                e = C.get_errno()
                if e == errno.EAGAIN:
                    return done
                raise OSError(e, f"reap: {os.strerror(e)}")
            urb = self.pending.pop(p.value)
            urb.reaped_at = time.monotonic_ns()
            done.append(urb)

    def discard_all(self, timeout=1.0):
        """Cancel every pending URB and reap them."""
        for address in list(self.pending):
            _ioctl(self.fd, DISCARDURB, address)  # fails only if it completed meanwhile
        deadline = time.monotonic() + timeout
        while self.pending and time.monotonic() < deadline:
            self.reap(10)

    def close(self):
        if self.pending:
            self.discard_all()
        for i in self.claimed:
            _ioctl(self.fd, RELEASEINTERFACE, C.byref(C.c_uint(i)))
        os.close(self.fd)
