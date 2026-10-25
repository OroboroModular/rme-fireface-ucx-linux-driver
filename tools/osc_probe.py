"""Change TotalMix parameters over OSC while capturing, to see what each change sends over USB.

    python tools/osc_probe.py re/captures/08-eq-enable.pcap --page /2 \\
        --set "eq on" /2/eqEnable 1 --set "eq off" /2/eqEnable 0

Reads the page's current values first, captures (via tools/capture.py, so start the capture
service first), sends each --set with a label in <pcap minus .pcap>.marks, waits --gap seconds
between steps, and finally puts every address it touched back to its starting value.
Addresses act on whatever channel and bus TotalMix has selected, so --page also selects the page.
TotalMix's OSC buttons (eqEnable, mute, phantom, ...) are toggles: 1 presses, 0 is ignored. So a
button step is "--set on /2/eqEnable 1 --set off /2/eqEnable 1".
"""

import argparse
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from osc import TotalMix  # noqa: E402


def mark(f, label):
    t = time.time()
    f.write(f"{t:.3f}\t{time.strftime('%H:%M:%S', time.localtime(t))}.{int(t * 1000) % 1000:03d}\t{label}\n")
    f.flush()
    print(f"{time.strftime('%H:%M:%S')} {label}", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out", help="output .pcap path")
    ap.add_argument("--page", default="/1", help="OSC page to select and read first (/1, /2 or /3)")
    ap.add_argument("--set", nargs=3, action="append", metavar=("LABEL", "ADDRESS", "VALUE"), required=True)
    ap.add_argument("--gap", type=float, default=3, help="seconds between steps")
    a = ap.parse_args()
    out = os.path.abspath(a.out)

    tm = TotalMix()
    before = tm.read_page(a.page)
    touched = []
    for _, address, _ in a.set:
        if address not in before or not before[address] or not isinstance(before[address][0], float):
            sys.exit(f"{address} not reported by TotalMix on page {a.page}; refusing to change what I can't restore")
        if address not in touched:
            touched.append(address)
    print("starting values:", ", ".join(f"{ad}={before[ad][0]}" for ad in touched))

    for stale in (out + ".log", out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    cap = subprocess.Popen([sys.executable, "-I", os.path.join(os.path.dirname(os.path.abspath(__file__)), "capture.py"),
                            out, "--seconds", str(60 + len(a.set) * (a.gap + 1) * 2)])
    for _ in range(100):
        if os.path.exists(out + ".log") and open(out + ".log", encoding="utf-8").read().startswith("start"):
            break
        if cap.poll() is not None:
            sys.exit("capture didn't start (is the capture service running? python tools/capture.py --start-service)")
        time.sleep(0.2)
    else:
        cap.terminate()
        sys.exit("capture didn't start in 20 s")

    marks = open(os.path.splitext(out)[0] + ".marks", "w", encoding="utf-8")
    try:
        mark(marks, "baseline")
        time.sleep(a.gap)
        for label, address, value in a.set:
            mark(marks, f"{label}: {address} {value}")
            tm.send(address, float(value))
            time.sleep(a.gap)
    finally:
        changed = restore(tm, a.page, before, touched, lambda label: mark(marks, label))
        time.sleep(a.gap)
        mark(marks, "done")
        marks.close()
        open(out + ".stop", "w").close()
        cap.wait()
    print("restored" if not changed else f"NOT restored: {', '.join(changed)}")


def restore(tm, page, before, touched, log):
    """Put touched addresses back. TotalMix buttons are toggles (1 = press, 0 is ignored), so
    re-read after sending the old value and press once more where a button is still wrong.
    Buttons are the addresses without a "<address>Val" display string; faders and knobs have one,
    and pressing those with 1.0 would set them to maximum."""
    def is_button(address):
        return address + "Val" not in before and before[address][0] in (0.0, 1.0)

    for address in touched:
        if not is_button(address):  # sending a button its old value would press it
            log(f"restore: {address} {before[address][0]}")
            tm.send(address, before[address][0])
    now = tm.read_page(page)
    for address in touched:
        if now.get(address) != before[address] and is_button(address):
            log(f"restore (toggle): {address} 1")
            tm.send(address, 1.0)
    now = tm.read_page(page)
    return [f"{ad} {before[ad]} -> {now.get(ad)}" for ad in touched if now.get(ad) != before[ad]]


if __name__ == "__main__":
    main()
