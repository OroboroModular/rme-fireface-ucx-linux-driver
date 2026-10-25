"""Map the mixer matrix: move every input and playback fader once in every output submix.

    python tools/map_matrix.py re/captures/06-matrix.pcap

Needs the capture service and TotalMix's OSC remote; audible, so only with nothing loud connected
or playing. Selects each submix with /setSubmix (0-based mono output index), then on page 2
steps through the input and playback rows, sets each strip's fader to a probe value, restores it
and checks TotalMix shows the old value again before moving on. Puts the submix and channel
selection back at the end. Writes the same .steps.jsonl/.marks as map_channel_settings.py, with
the row recorded as "<row>@<submix>", so tools/map_report.py works on it.
"""

import argparse
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from map_channel_settings import ROWS, Run, clamp, v  # noqa: E402

OUTPUT_PAIRS = [0, 2, 4, 6, 8, 10, 12, 14, 16]  # AN 1/2, AN 3/4, AN 5/6, PH 7/8, SPDIF, ADAT 1/2 ... 7/8
NEED = ("/2/trackname", "/2/labelSubmix", "/2/volume", "/2/volumeVal")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--submixes", default=",".join(map(str, OUTPUT_PAIRS)), help="mono output indexes to map")
    ap.add_argument("--gap", type=float, default=0.45)
    a = ap.parse_args()
    out = os.path.abspath(a.out)
    run = Run(out, a.gap)
    tm = run.tm

    start = tm.read_page("/2", require=NEED)
    start_row = next((r for r, addr in ROWS.items() if start.get(addr) == [1.0]), "input")
    start_name, start_submix = v(start, "trackname"), v(start, "labelSubmix")

    for stale in (out + ".log", out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    cap = subprocess.Popen([sys.executable, "-I", os.path.join(os.path.dirname(os.path.abspath(__file__)), "capture.py"), out, "--seconds", "3500"])
    for _ in range(100):
        if os.path.exists(out + ".log") and open(out + ".log", encoding="utf-8").read().startswith("start"):
            break
        if cap.poll() is not None:
            sys.exit("capture didn't start (is the capture service running? python tools/capture.py --start-service)")
        time.sleep(0.2)

    start_index = None
    try:
        run.mark("baseline")
        time.sleep(2)
        for index in map(int, a.submixes.split(",")):
            tm.send("/setSubmix", float(index))
            submix = tm.receive(0.8).get("/1/labelSubmix", [None])[0] or v(tm.read_page("/2"), "labelSubmix")
            if submix == start_submix and start_index is None:
                start_index = index
            run.mark(f"submix {index} = {submix}")
            print(f"{time.strftime('%H:%M:%S')} submix {submix}", flush=True)
            for row in ("input", "playback"):
                tm.send(ROWS[row], 1.0)
                for _ in range(40):
                    tm.send("/2/track-", 1.0)
                seen = []
                while True:
                    before = tm.read_page("/2", 1.0, require=NEED)
                    name = v(before, "trackname")
                    if seen and name == seen[-1]:
                        break
                    if v(before, "labelSubmix") != submix:
                        raise SystemExit(f"submix changed under us: {v(before, 'labelSubmix')} != {submix}")
                    cur = v(before, "volume")
                    probe = 0.5 if abs(cur - 0.5) > 0.01 else 0.75
                    run.set(f"{row}@{submix}", name, "volume", probe, "probe")
                    run.set(f"{row}@{submix}", name, "volume", clamp(cur), "restore")
                    now = tm.read_page("/2", 1.0, require=NEED)
                    if now.get("/2/volumeVal") != before.get("/2/volumeVal"):
                        raise SystemExit(f"{row} {name} in {submix}: fader not restored: {before.get('/2/volumeVal')} -> {now.get('/2/volumeVal')}")
                    seen.append(name)
                    tm.send("/2/track+", 1.0)
    finally:
        tm.send("/setSubmix", float(start_index if start_index is not None else 0))
        tm.send(ROWS[start_row], 1.0)
        for _ in range(40):
            tm.send("/2/track-", 1.0)
        for _ in range(40):
            if v(tm.read_page("/2", 0.8), "trackname") == start_name:
                break
            tm.send("/2/track+", 1.0)
        run.mark("done")
        time.sleep(1)
        open(out + ".stop", "w").close()
        cap.wait()
    end = tm.read_page("/2")
    print(f"submix back on {v(end, 'labelSubmix')}, selection {v(end, 'trackname')}; all faders restored and verified")


if __name__ == "__main__":
    main()
