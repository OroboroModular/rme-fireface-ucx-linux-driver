"""Capture the UCX while stepping the sample rate through ASIO, then restore the starting rate.

    python tools/rate_sweep.py re/captures/03-rates.pcap [--rates 32000,44100,...] [--gap 4]

Needs the capture service (python tools/capture.py --start-service). Labels each change in
<pcap minus .pcap>.marks and checks the driver reports the requested rate afterwards.
"""

import argparse
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from asio_ctl import RATES, Asio  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--rates", default=",".join(map(str, RATES)))
    ap.add_argument("--gap", type=float, default=4)
    a = ap.parse_args()
    out = os.path.abspath(a.out)
    rates = [int(r) for r in a.rates.split(",")]

    asio = Asio()
    start_rate = round(asio.rate())
    print(f"starting rate {start_rate} Hz")
    for stale in (out + ".log", out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    cap = subprocess.Popen([sys.executable, "-I", os.path.join(os.path.dirname(os.path.abspath(__file__)), "capture.py"),
                            out, "--seconds", str(60 + len(rates) * a.gap * 2)])
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
        time.sleep(a.gap)
        for r in rates + [start_rate]:
            mark(f"rate {r}" + (" (restore)" if r == start_rate and r is rates[-1] is not r else ""))
            err = asio.set_rate(r)
            time.sleep(a.gap)
            now = round(asio.rate())
            mark(f"  -> setSampleRate returned {err}, driver reports {now}")
            if now != r:
                print(f"WARNING: asked for {r}, driver reports {now}")
    finally:
        if round(asio.rate()) != start_rate:
            asio.set_rate(start_rate)
            time.sleep(a.gap)
        mark(f"done, rate {round(asio.rate())}")
        asio.close()
        marks.close()
        open(out + ".stop", "w").close()
        cap.wait()


if __name__ == "__main__":
    main()
