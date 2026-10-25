"""Timestamped action log for lining up USB captures with what you did.

    python tools/mark.py re/captures/06-matrix.marks [--stop re/captures/06-matrix.pcap]

Type a label and press Enter before each action; an empty line or Ctrl+C ends the session.
Each line is written as "<unix time with ms>\t<local time>\t<label>".
With --stop, ending the session also ends the tools/capture.py capture writing that pcap.
"""

import argparse
import time


def main():
    ap = argparse.ArgumentParser(usage=__doc__)
    ap.add_argument("marks")
    ap.add_argument("--stop", metavar="PCAP", help="stop the capture writing this pcap when done")
    a = ap.parse_args()
    print("Type a label and press Enter before each action. Empty line ends the session.")
    with open(a.marks, "a", encoding="utf-8") as f:
        try:
            while True:
                label = input("> ").strip()
                if not label:
                    break
                t = time.time()
                f.write(f"{t:.3f}\t{time.strftime('%H:%M:%S', time.localtime(t))}.{int(t * 1000) % 1000:03d}\t{label}\n")
                f.flush()
        except (KeyboardInterrupt, EOFError):
            pass
    if a.stop:
        open(a.stop + ".stop", "w").close()
        print("capture stopped")


if __name__ == "__main__":
    main()
