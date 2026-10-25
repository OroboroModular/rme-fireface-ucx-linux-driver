"""Briefly stream digital silence through RME's ASIO driver at each sample rate while capturing.

    python tools/asio_stream.py re/captures/12-stream-rates.pcap [--rates 44100,96000] [--seconds 2]

Shows what the driver sends to start and stop streaming and to switch speed (1x/2x/4x), which a
rate change alone doesn't (that only sets the base-rate bits). Uses 2 output and 2 input channels;
every output buffer is zeroed in every callback, so only silence is played. Restores the starting
rate. Needs the capture service; packets are truncated to --snaplen bytes (default 128).
"""

import argparse
import ctypes
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from asio_ctl import RATES, Asio  # noqa: E402

SAMPLE_BYTES = {16: 2, 17: 3, 18: 4, 19: 4, 20: 8}  # ASIOSampleType -> bytes (Int16/24/32LSB, Float32/64LSB)


class ASIOBufferInfo(ctypes.Structure):
    _fields_ = [("isInput", ctypes.c_long), ("channelNum", ctypes.c_long), ("buffers", ctypes.c_void_p * 2)]


class ASIOChannelInfo(ctypes.Structure):
    _fields_ = [("channel", ctypes.c_long), ("isInput", ctypes.c_long), ("isActive", ctypes.c_long),
                ("channelGroup", ctypes.c_long), ("type", ctypes.c_long), ("name", ctypes.c_char * 32)]


BUFFER_SWITCH = ctypes.CFUNCTYPE(None, ctypes.c_long, ctypes.c_long)
RATE_CHANGED = ctypes.CFUNCTYPE(None, ctypes.c_double)
ASIO_MESSAGE = ctypes.CFUNCTYPE(ctypes.c_long, ctypes.c_long, ctypes.c_long, ctypes.c_void_p, ctypes.POINTER(ctypes.c_double))
SWITCH_TIME_INFO = ctypes.CFUNCTYPE(ctypes.c_void_p, ctypes.c_void_p, ctypes.c_long, ctypes.c_long)


class ASIOCallbacks(ctypes.Structure):
    _fields_ = [("bufferSwitch", BUFFER_SWITCH), ("sampleRateDidChange", RATE_CHANGED),
                ("asioMessage", ASIO_MESSAGE), ("bufferSwitchTimeInfo", SWITCH_TIME_INFO)]


class Stream:
    """Silent 2-in/2-out ASIO stream on an open Asio. With pattern=True: all channels, and output
    channel n carries the constant 24-bit value n + 1 (about -138 dBFS, inaudible) so its slot can
    be found in the USB packets."""

    def __init__(self, asio, pattern=False):
        self.asio = asio
        mn, mx, pref, gran = (ctypes.c_long() for _ in range(4))
        asio.call("getBufferSize", ctypes.c_long, [ctypes.POINTER(ctypes.c_long)] * 4,
                  ctypes.byref(mn), ctypes.byref(mx), ctypes.byref(pref), ctypes.byref(gran))
        self.frames = pref.value
        info = ASIOChannelInfo(channel=0, isInput=0)
        asio.call("getChannelInfo", ctypes.c_long, [ctypes.POINTER(ASIOChannelInfo)], ctypes.byref(info))
        if info.type not in SAMPLE_BYTES:
            raise OSError(f"unknown ASIO sample type {info.type}")
        self.bytes = self.frames * SAMPLE_BYTES[info.type]
        n_in, n_out = ctypes.c_long(), ctypes.c_long()
        asio.call("getChannels", ctypes.c_long, [ctypes.POINTER(ctypes.c_long)] * 2, ctypes.byref(n_in), ctypes.byref(n_out))
        self.n_out, self.n_in = (n_out.value, n_in.value) if pattern else (2, 2)
        if pattern and info.type != 18:
            raise OSError(f"pattern needs Int32LSB samples, driver uses type {info.type}")
        chans = [ASIOBufferInfo(0, c) for c in range(self.n_out)] + [ASIOBufferInfo(1, c) for c in range(self.n_in)]
        self.infos = (ASIOBufferInfo * len(chans))(*chans)
        # one prepared buffer per output: zeros, or the channel's constant (24-bit n+1 in the top bits)
        self.fill = [(ctypes.c_int32 * self.frames)(*([((c + 1) << 8) if pattern else 0] * self.frames)) for c in range(self.n_out)]
        self.switches = 0

        def buffer_switch(index, direct):
            for c, bi in enumerate(self.infos[:self.n_out]):
                ctypes.memmove(bi.buffers[index], self.fill[c], self.bytes)
            self.switches += 1

        def message(selector, value, msg, opt):
            if selector == 1:  # kAsioSelectorSupported
                return 1 if value in (1, 2) else 0
            return 2 if selector == 2 else 0  # kAsioEngineVersion

        # keep the ctypes callback objects alive as long as the driver may call them
        self.cb = ASIOCallbacks(BUFFER_SWITCH(buffer_switch), RATE_CHANGED(lambda r: None),
                                ASIO_MESSAGE(message), SWITCH_TIME_INFO(lambda p, i, d: None))
        err = asio.call("createBuffers", ctypes.c_long, [ctypes.POINTER(ASIOBufferInfo), ctypes.c_long, ctypes.c_long,
                        ctypes.POINTER(ASIOCallbacks)], self.infos, len(chans), self.frames, ctypes.byref(self.cb))
        if err:
            raise OSError(f"createBuffers failed: {err}")
        for c, bi in enumerate(self.infos[:self.n_out]):
            for b in bi.buffers:
                ctypes.memmove(b, self.fill[c], self.bytes)

    def run(self, seconds):
        err = self.asio.call("start", ctypes.c_long, [])
        if err:
            raise OSError(f"start failed: {err}")
        time.sleep(seconds)
        self.asio.call("stop", ctypes.c_long, [])

    def close(self):
        self.asio.call("disposeBuffers", ctypes.c_long, [])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--rates", default=",".join(map(str, RATES)))
    ap.add_argument("--seconds", type=float, default=2)
    ap.add_argument("--snaplen", type=int, default=128)
    ap.add_argument("--pattern", action="store_true", help="all channels, output n carries the inaudible constant n+1")
    a = ap.parse_args()
    out = os.path.abspath(a.out)
    rates = [int(r) for r in a.rates.split(",")]

    asio = Asio()
    start_rate = round(asio.rate())
    for stale in (out + ".log", out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    cap = subprocess.Popen([sys.executable, "-I", os.path.join(os.path.dirname(os.path.abspath(__file__)), "capture.py"),
                            out, "--seconds", str(60 + len(rates) * (a.seconds + 4)), "--snaplen", str(a.snaplen)])
    for _ in range(100):
        if os.path.exists(out + ".log") and open(out + ".log", encoding="utf-8").read().startswith("start"):
            break
        if cap.poll() is not None:
            sys.exit("capture didn't start (is the capture service running?)")
        time.sleep(0.2)
    marks = open(os.path.splitext(out)[0] + ".marks", "w", encoding="utf-8")

    def mark(label):
        t = time.time()
        marks.write(f"{t:.3f}\t{time.strftime('%H:%M:%S', time.localtime(t))}.{int(t * 1000) % 1000:03d}\t{label}\n")
        marks.flush()
        print(time.strftime("%H:%M:%S"), label, flush=True)

    try:
        mark("baseline")
        time.sleep(2)
        for r in rates:
            mark(f"rate {r}")
            asio.set_rate(r)
            time.sleep(1.5)
            s = Stream(asio, a.pattern)
            mark(f"start stream at {round(asio.rate())} Hz ({s.frames} frames, {s.bytes // s.frames} bytes/sample, {s.n_out} out / {s.n_in} in)")
            s.run(a.seconds)
            mark(f"stopped after {s.switches} buffer switches")
            s.close()
            time.sleep(1.5)
    finally:
        asio.set_rate(start_rate)
        time.sleep(1.5)
        mark(f"done, rate {round(asio.rate())}")
        asio.close()
        marks.close()
        open(out + ".stop", "w").close()
        cap.wait()


if __name__ == "__main__":
    main()
