"""Talk to TotalMix FX over OSC (its built-in remote control; enable it under Options > Settings > OSC).

    python tools/osc.py listen [--seconds 10]            # print what TotalMix sends back
    python tools/osc.py send /1/busInput 1               # send one message (float args)
    python tools/osc.py send /1/volume1 0.5 --listen 2   # send, then print replies for 2 s

TotalMix listens on 127.0.0.1:7001 and replies to 127.0.0.1:9001 (the ports set in its OSC tab).
Replies come as OSC bundles; each message carries float, int or string arguments.
"""

import argparse
import socket
import struct
import time

TOTALMIX = ("127.0.0.1", 7001)
REPLY_PORT = 9001


def _pad(b):
    return b + b"\0" * (4 - len(b) % 4)


def encode(address, *args):
    tags, data = ",", b""
    for v in args:
        if isinstance(v, str):
            tags += "s"
            data += _pad(v.encode())
        elif isinstance(v, int) and not isinstance(v, bool):
            tags += "i"
            data += struct.pack(">i", v)
        else:
            tags += "f"
            data += struct.pack(">f", float(v))
    return _pad(address.encode()) + _pad(tags.encode()) + data


def _string(buf, i):
    end = buf.index(b"\0", i)
    return buf[i:end].decode(errors="replace"), (end + 4) & ~3


def decode(buf):
    """Yield (address, [args]) for a message or (recursively) a bundle."""
    if buf.startswith(b"#bundle\0"):
        i = 16  # "#bundle\0" + 8-byte timetag
        while i + 4 <= len(buf):
            n = struct.unpack_from(">i", buf, i)[0]
            yield from decode(buf[i + 4:i + 4 + n])
            i += 4 + n
        return
    address, i = _string(buf, 0)
    if i >= len(buf) or buf[i:i + 1] != b",":
        yield address, []
        return
    tags, i = _string(buf, i)
    args = []
    for t in tags[1:]:
        if t == "f":
            args.append(struct.unpack_from(">f", buf, i)[0])  # exact, so it can be sent back unchanged
            i += 4
        elif t == "i":
            args.append(struct.unpack_from(">i", buf, i)[0])
            i += 4
        elif t == "s":
            s, i = _string(buf, i)
            args.append(s)
        else:
            args.append(f"<{t}>")
    yield address, args


class TotalMix:
    def __init__(self, reply_port=REPLY_PORT):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(("127.0.0.1", reply_port))
        self.sock.settimeout(0.1)

    def send(self, address, *args):
        # 48V can damage ribbon mics and thump speakers: no script may ever switch it
        if "phantom" in address.lower():
            raise ValueError(f"refusing to send {address}: phantom power is never automated")
        self.sock.sendto(encode(address, *args), TOTALMIX)

    def read_page(self, page, seconds=1.5, require=(), tries=4):
        """Return {address: args} for a page ("/1", "/2", "/3"). TotalMix only sends a page's
        full contents when the remote switches to it, so hop to another page first. Replies are
        sometimes incomplete; with require, retry until all those addresses are present."""
        for _ in range(tries):
            self.send("/3" if page != "/3" else "/1", 1.0)
            self.receive(0.5)
            self.send(page, 1.0)
            state = self.receive(seconds)
            if all(addr in state for addr in require):
                return state
        raise RuntimeError(f"TotalMix page {page} kept coming back without {[a for a in require if a not in state]}")

    def receive(self, seconds):
        """Collect replies for this long; return {address: latest args}, in arrival order."""
        state, end = {}, time.time() + seconds
        while time.time() < end:
            try:
                buf, _ = self.sock.recvfrom(65536)
            except socket.timeout:
                continue
            except ConnectionResetError:  # Windows reports an ICMP "port unreachable" here
                continue
            for address, args in decode(buf):
                state.pop(address, None)
                state[address] = args
        return state


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    li = sub.add_parser("listen")
    li.add_argument("--seconds", type=float, default=10)
    se = sub.add_parser("send")
    se.add_argument("address")
    se.add_argument("args", nargs="*", type=float)
    se.add_argument("--listen", type=float, default=0, help="print replies for this many seconds")
    a = ap.parse_args()

    tm = TotalMix()
    if a.cmd == "send":
        tm.send(a.address, *(a.args or [1.0]))
        seconds = a.listen
    else:
        seconds = a.seconds
    if seconds:
        state = tm.receive(seconds)
        for address, args in state.items():
            print(address, *(round(x, 4) if isinstance(x, float) else x for x in args))
        print(f"({len(state)} addresses)")


if __name__ == "__main__":
    main()
