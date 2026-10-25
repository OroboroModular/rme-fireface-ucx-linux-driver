"""Summarize a USBPcap capture of the UCX with tshark.

    python tools/pcap_summary.py re/captures/02-idle.pcap [--marks re/captures/02-idle.marks] [--controls]

Prints traffic per endpoint, the vendor control requests grouped by request number, and sample
payloads per endpoint. --controls lists every vendor control transfer in order, interleaved with
the labels from a tools/mark.py file.
"""

import argparse
import collections
import subprocess
import sys

TSHARK = r"C:\Program Files\Wireshark\tshark.exe"
FIELDS = [
    "frame.number", "frame.time_epoch", "usb.irp_info.direction", "usb.endpoint_address", "usb.transfer_type",
    "usb.data_len", "usb.control_stage", "usb.bmRequestType", "usb.setup.bRequest", "usb.setup.wValue",
    "usb.setup.wIndex", "usb.setup.wLength", "usb.data_fragment", "usb.control.Response", "usb.capdata", "usb.urb_status",
]
TYPES = {"0x00": "iso", "0x01": "intr", "0x02": "ctrl", "0x03": "bulk"}


def rows(path):
    cmd = [TSHARK, "-r", path, "-T", "fields", "-E", "separator=\t", "-E", "occurrence=f"]
    for f in FIELDS:
        cmd += ["-e", f]
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    for line in out.splitlines():
        r = dict(zip(FIELDS, line.split("\t")))
        r["t"] = float(r["frame.time_epoch"] or 0)
        r["data"] = (r["usb.data_fragment"] or r["usb.control.Response"] or r["usb.capdata"]).replace(":", "")
        yield r


def h(v):
    """tshark prints setup fields as decimal or hex depending on version; normalize to int."""
    return int(v, 0) if v else None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("pcap")
    ap.add_argument("--marks")
    ap.add_argument("--controls", action="store_true", help="list every vendor control transfer in order")
    ap.add_argument("--samples", type=int, default=3, help="payload samples per endpoint")
    ap.add_argument("--timeline", action="store_true", help="marks, vendor writes and interface-1 words in time order")
    a = ap.parse_args()

    data = list(rows(a.pcap))
    if not data:
        sys.exit("no packets")
    t0 = data[0]["t"]
    print(f"{len(data)} packets over {data[-1]['t'] - t0:.3f} s")

    # completions (irp direction 1, device -> host) carry IN data; submissions carry OUT data
    eps = collections.OrderedDict()
    for r in data:
        ep = r["usb.endpoint_address"]
        key = (ep, TYPES.get(r["usb.transfer_type"], r["usb.transfer_type"]))
        e = eps.setdefault(key, {"submit": 0, "complete": 0, "bytes": 0, "lens": collections.Counter(), "samples": []})
        e["submit" if not h(r["usb.irp_info.direction"]) else "complete"] += 1
        n = int(r["usb.data_len"] or 0)
        if n:
            e["bytes"] += n
            e["lens"][n] += 1
            if len(e["samples"]) < a.samples and r["usb.transfer_type"] != "0x02":
                e["samples"].append((r["t"] - t0, n, r["data"][:96]))

    print("\nendpoint  type   submits  completions  data bytes  common lengths")
    for (ep, ty), e in sorted(eps.items()):
        lens = ", ".join(f"{n}B x{c}" for n, c in e["lens"].most_common(4))
        print(f"{ep:>8}  {ty:<5}  {e['submit']:>7}  {e['complete']:>11}  {e['bytes']:>10}  {lens}")

    # vendor control transfers: bmRequestType 0x40 (out) / 0xc0 (in)
    ctrl = [r for r in data if r["usb.bmRequestType"] and h(r["usb.bmRequestType"]) & 0x60 == 0x40]
    groups = collections.OrderedDict()
    for r in ctrl:
        k = (h(r["usb.bmRequestType"]), h(r["usb.setup.bRequest"]))
        g = groups.setdefault(k, {"n": 0, "vi": collections.Counter()})
        g["n"] += 1
        g["vi"][(h(r["usb.setup.wValue"]), h(r["usb.setup.wIndex"]), h(r["usb.setup.wLength"]))] += 1
    print(f"\nvendor control requests ({len(ctrl)} setups)")
    for (bm, req), g in groups.items():
        top = ", ".join(f"v=0x{v:04x} i=0x{i:04x} len={ln} x{c}" for (v, i, ln), c in g["vi"].most_common(4))
        more = f" (+{len(g['vi']) - 4} more combos)" if len(g["vi"]) > 4 else ""
        print(f"  0x{bm:02x} req {req:>3} (0x{req:02x}): {g['n']:>5}  {top}{more}")

    print("\npayload samples")
    for (ep, ty), e in sorted(eps.items()):
        for t, n, d in e["samples"]:
            print(f"  {ep} {ty:<4} +{t:8.4f}s {n:>5}B {d}")

    if a.controls:
        marks = []
        if a.marks:
            for line in open(a.marks, encoding="utf-8"):
                ts, _, label = line.rstrip("\n").split("\t", 2)
                marks.append((float(ts), label))
        # pair each setup (submission) with its completion to get IN data
        pending = {}
        print("\nvendor control transfers in order")
        for r in data:
            while marks and marks[0][0] <= r["t"]:
                print(f"  ---- +{marks[0][0] - t0:8.4f}s MARK: {marks[0][1]}")
                marks.pop(0)
            if r["usb.transfer_type"] != "0x02":
                continue
            if r["usb.bmRequestType"]:
                bm = h(r["usb.bmRequestType"])
                if bm & 0x60 != 0x40:
                    continue
                line = f"  +{r['t'] - t0:8.4f}s 0x{bm:02x} req {h(r['usb.setup.bRequest']):>3} v=0x{h(r['usb.setup.wValue']):04x} i=0x{h(r['usb.setup.wIndex']):04x} len={h(r['usb.setup.wLength'])}"
                if bm & 0x80:
                    pending[r["frame.number"]] = line
                    pending["last"] = line
                else:
                    print(line + (f" out={r['data']}" if r["data"] else ""))
            elif h(r["usb.irp_info.direction"]) == 1 and "last" in pending and r["data"]:
                print(pending.pop("last") + f" in={r['data']}")

    if a.timeline:
        timeline(data, t0, a.marks)


POLL_STATUS = {17, 28, 30, 31}
KEEPALIVE = {0x3F98FFFF, 0x3F9E0000}


def is_idle_word(w):
    """The constant interface-1 polling: 0x3f, two zero bytes, a 4-bit counter; plus the keep-alive pair."""
    v = w & 0x7FFFFFFF
    return v in KEEPALIVE or (v >> 24 == 0x3F and v & 0x00FFFFF0 == 0)


def timeline(data, t0, marks_path):
    """Marks, vendor control writes and interface-1 out words in time order, idle polling left out."""
    events = []
    if marks_path:
        for line in open(marks_path, encoding="utf-8"):
            ts, _, label = line.rstrip("\n").split("\t", 2)
            events.append((float(ts), 0, f"---- MARK: {label}"))
    for r in data:
        if r["usb.bmRequestType"]:
            bm, req = h(r["usb.bmRequestType"]), h(r["usb.setup.bRequest"])
            if bm == 0x40 or (bm == 0xC0 and req not in POLL_STATUS):
                events.append((r["t"], 1, f"ctrl 0x{bm:02x} req {req:>3} v=0x{h(r['usb.setup.wValue']):04x} i=0x{h(r['usb.setup.wIndex']):04x}"
                               + (f" out={r['data']}" if r["data"] else "")))
        elif r["usb.endpoint_address"] == "0x0c" and not h(r["usb.irp_info.direction"]) and r["data"]:
            b = bytes.fromhex(r["data"])
            for i in range(0, len(b) - 3, 4):
                w = int.from_bytes(b[i:i + 4], "little")
                if not is_idle_word(w):
                    v = w & 0x7FFFFFFF
                    events.append((r["t"], 1, f"0x0c word 0x{v:08x}  [{v >> 24:02x} | {(v >> 16) & 0xff:02x} {(v >> 8) & 0xff:02x} {v & 0xff:02x}]"
                                   + ("" if bin(w).count("1") % 2 else "  (EVEN PARITY)")))
    print("\ntimeline (idle polling left out)")
    for t, _, text in sorted(events, key=lambda e: (e[0], e[1])):
        print(f"  +{t - t0:8.4f}s {text}")


if __name__ == "__main__":
    main()
