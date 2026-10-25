"""Export test fixtures for crates/ucx-protocol from the captures in re/captures.

    python tools/export_fixtures.py

Writes crates/ucx-protocol/tests/fixtures/:
  state_push.txt     the 1,108 interface-1 words TotalMix sent after power-on (raw u32 hex, as on the wire)
  power_on.txt       the driver's vendor control writes after power-on, in order ("request value index", hex)
  meters.txt         one meter packet of each type from the inaudible pattern stream (244 bytes hex per line)
  matrix_probes.txt  TotalMix display dB and the matrix value written, for playback fader probes ("dB value")
"""

import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pcap_summary import POLL_STATUS, h, rows  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CAP = os.path.join(ROOT, "re", "captures")
OUT = os.path.join(ROOT, "crates", "ucx-protocol", "tests", "fixtures")
TSHARK = r"C:\Program Files\Wireshark\tshark.exe"


def bulk_out_words(pcap):
    for r in rows(pcap):
        if r["usb.endpoint_address"] == "0x0c" and not h(r["usb.irp_info.direction"]) and r["data"]:
            b = bytes.fromhex(r["data"])
            if len(b) > 8:  # the state push bursts, not the 4-byte polls or 8-byte refresh
                for i in range(0, len(b) - 3, 4):
                    yield int.from_bytes(b[i:i + 4], "little")


def main():
    os.makedirs(OUT, exist_ok=True)
    init = os.path.join(CAP, "01-init-a.pcap")

    words = list(bulk_out_words(init))
    with open(os.path.join(OUT, "state_push.txt"), "w") as f:
        f.write("# interface-1 words from re/captures/01-init-a.pcap (TotalMix state push after power-on)\n")
        f.writelines(f"{w:08x}\n" for w in words)

    with open(os.path.join(OUT, "power_on.txt"), "w") as f:
        f.write("# vendor OUT control writes after power-on, re/captures/01-init-a.pcap: request value index (hex)\n")
        for r in rows(init):
            if r["usb.bmRequestType"] and h(r["usb.bmRequestType"]) == 0x40:
                f.write(f"{h(r['usb.setup.bRequest']):02x} {h(r['usb.setup.wValue']):04x} {h(r['usb.setup.wIndex']):04x}\n")

    out = subprocess.run([TSHARK, "-r", os.path.join(CAP, "02-meters-pattern.pcap"), "-Y",
                          "usb.endpoint_address == 0x85 && usb.irp_info.direction == 1 && usb.data_len == 244",
                          "-T", "fields", "-e", "frame.time_relative", "-e", "usb.capdata"], capture_output=True, text=True).stdout
    seen = {}
    for line in out.splitlines():
        t, d = line.split("\t")
        if 5.0 <= float(t) <= 6.5:  # while the pattern streams
            b = bytes.fromhex(d.replace(":", ""))
            seen.setdefault(b[240:], b)
    with open(os.path.join(OUT, "meters.txt"), "w") as f:
        f.write("# one 244-byte meter packet per type from re/captures/02-meters-pattern.pcap; playback n carried 24-bit n+1\n")
        for tail in sorted(seen):
            f.write(seen[tail].hex() + "\n")

    with open(os.path.join(OUT, "matrix_probes.txt"), "w") as f:
        f.write("# playback fader probes (stereo strips, balance centre): TotalMix display dB, matrix value written\n")
        for line in open(os.path.join(ROOT, "re", "maps", "audible-first.md"), encoding="utf-8"):
            m = re.match(r"\| playback \| AN 1/2 \| volume \| \w+ \| [^|]+ \| (-?[\d.]+) dB \| 4020:([0-9a-f]{4}) 4061:\2", line.strip())
            if m:
                f.write(f"{m.group(1)} {m.group(2)}\n")
    print("wrote", ", ".join(sorted(os.listdir(OUT))), f"({len(words)} push words, {len(seen)} meter packets)")


if __name__ == "__main__":
    main()
