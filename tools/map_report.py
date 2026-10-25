"""Match the changes logged by tools/map_channel_settings.py to the register writes in the capture.

    python tools/map_report.py re/captures/08-settings-mic1.pcap [--md re/maps/settings.md]

For each change, lists the interface-1 words (address:value, parity bit stripped) and vendor
control writes the driver sent before the next change. Then summarizes per parameter: which
register addresses it touched and how the OSC value / TotalMix's display maps to the register value.
"""

import argparse
import collections
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pcap_summary import POLL_STATUS, h, is_idle_word, rows  # noqa: E402


def writes(pcap):
    """(time, text, (addr, value) or None) for each non-idle interface-1 out word and vendor write."""
    for r in rows(pcap):
        if r["usb.bmRequestType"]:
            bm, req = h(r["usb.bmRequestType"]), h(r["usb.setup.bRequest"])
            if bm == 0x40 or (bm == 0xC0 and req not in POLL_STATUS):
                yield r["t"], f"req{req} v=0x{h(r['usb.setup.wValue']):04x} i=0x{h(r['usb.setup.wIndex']):04x}", None
        elif r["usb.endpoint_address"] == "0x0c" and not h(r["usb.irp_info.direction"]) and r["data"]:
            b = bytes.fromhex(r["data"])
            for i in range(0, len(b) - 3, 4):
                w = int.from_bytes(b[i:i + 4], "little")
                if not is_idle_word(w):
                    w &= 0x7FFFFFFF
                    yield r["t"], f"{w >> 16:04x}:{w & 0xffff:04x}", (w >> 16, w & 0xFFFF)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("pcap")
    ap.add_argument("--md", help="also write the report as Markdown here")
    a = ap.parse_args()

    steps = [json.loads(line) for line in open(a.pcap + ".steps.jsonl", encoding="utf-8")]
    ws = sorted(writes(a.pcap))
    lines, per_param = [], collections.OrderedDict()
    j = 0
    for i, s in enumerate(steps):
        end = steps[i + 1]["t"] if i + 1 < len(steps) else s["t"] + 2
        while j < len(ws) and ws[j][0] < s["t"]:
            j += 1
        got = []
        while j < len(ws) and ws[j][0] < end:
            got.append(ws[j])
            j += 1
        key = (s["row"], s["channel"], s["param"])
        p = per_param.setdefault(key, {"addrs": collections.Counter(), "points": []})
        for _, _, aw in got:
            if aw:
                p["addrs"][aw[0]] += 1
        p["points"].append((s["value"], s["shown"], s["kind"], [g[1] for g in got]))
        lines.append(f"| {s['row']} | {s['channel']} | {s['param']} | {s['kind']} | {s['value']:.4g} | {s['shown'] or ''} | {' '.join(g[1] for g in got) or '(none)'} |")

    out = ["# Channel settings → register writes", "",
           f"From `{os.path.basename(a.pcap)}` ({len(steps)} changes). Words are `address:value` (hex, bits 30–16 : 15–0).", "",
           "## Per parameter", "",
           "| Row | Channel | Parameter | Addresses | OSC value → display → words |", "|---|---|---|---|---|"]
    for (row, ch, param), p in per_param.items():
        addrs = " ".join(f"`{ad:04x}`" for ad in p["addrs"]) or "none"
        pts = "<br>".join(f"{val:.4g} → {shown or '?'} → {' '.join(w) or '(none)'}" for val, shown, kind, w in p["points"])
        out.append(f"| {row} | {ch} | {param} | {addrs} | {pts} |")
    out += ["", "## Every change in order", "", "| Row | Channel | Parameter | Kind | OSC value | Display | Writes |", "|---|---|---|---|---|---|---|"] + lines
    text = "\n".join(out) + "\n"
    if a.md:
        os.makedirs(os.path.dirname(os.path.abspath(a.md)), exist_ok=True)
        open(a.md, "w", encoding="utf-8").write(text)
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")  # Windows consoles default to cp1252
    print(text)


if __name__ == "__main__":
    main()
