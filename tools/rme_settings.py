"""Operate RME's "Fireface USB Settings" dialog by control ID, to capture what each device setting sends.

    python tools/rme_settings.py status
    python tools/rme_settings.py sweep re/captures/04-settings.pcap

The dialog must be open (launching firefaceusb_x64.exe again opens it). Uses plain Win32
messages on the dialog's standard controls: CB_SETCURSEL + CBN_SELCHANGE for drop-downs, BM_CLICK
for check boxes. "sweep" captures (via tools/capture.py, so start the capture service first)
while it changes one setting at a time and puts it back, then checks every control reads back
its starting value. Buffer size, WDM device count and pitch are never touched.
"""

import ctypes
import ctypes.wintypes as wt
import os
import subprocess
import sys
import time

user32 = ctypes.WinDLL("user32", use_last_error=True)
user32.SendMessageW.restype = ctypes.c_ssize_t
user32.SendMessageW.argtypes = [wt.HWND, wt.UINT, wt.WPARAM, wt.LPARAM]
user32.FindWindowW.restype = wt.HWND
user32.GetDlgItem.restype = wt.HWND
user32.GetParent.restype = wt.HWND
ENUM = ctypes.WINFUNCTYPE(wt.BOOL, wt.HWND, wt.LPARAM)

CB_GETCURSEL, CB_SETCURSEL, CB_GETCOUNT, CB_GETLBTEXT, CB_GETLBTEXTLEN = 0x0147, 0x014E, 0x0146, 0x0148, 0x0149
BM_GETCHECK, BM_CLICK, WM_COMMAND = 0x00F0, 0x00F5, 0x0111
CBN_SELCHANGE, CBN_CLOSEUP, CBN_SELENDOK = 1, 8, 9

COMBOS = {"clock": 1568, "optical out": 1567, "spdif format": 1559}
CHECKS = {"tms": 1556, "word termination": 1561, "eq+d for record": 1558, "word single speed": 1557}
CURRENT_CLOCK = 1565


def dialog():
    top = user32.FindWindowW("#32770", "Fireface USB Settings")
    if not top:
        sys.exit('settings dialog not open (run "C:\\Program Files\\RME\\Fireface\\firefaceusb_x64.exe")')
    return top


def control(top, cid):
    found = []

    def cb(h, _):
        if user32.GetDlgCtrlID(h) == cid:
            found.append(h)
        return True

    user32.EnumChildWindows(top, ENUM(cb), 0)
    if not found:
        raise LookupError(f"no control {cid}")
    return found[0]


def text(h):
    buf = ctypes.create_unicode_buffer(256)
    user32.GetWindowTextW(h, buf, 256)
    return buf.value.strip()


def combo_items(h):
    items = []
    for i in range(user32.SendMessageW(h, CB_GETCOUNT, 0, 0)):
        buf = ctypes.create_unicode_buffer(user32.SendMessageW(h, CB_GETLBTEXTLEN, i, 0) + 1)
        user32.SendMessageW(h, CB_GETLBTEXT, i, ctypes.cast(buf, ctypes.c_void_p).value)
        items.append(buf.value.strip())
    return items


def state(top):
    s = {name: user32.SendMessageW(control(top, cid), CB_GETCURSEL, 0, 0) for name, cid in COMBOS.items()}
    s.update({name: user32.SendMessageW(control(top, cid), BM_GETCHECK, 0, 0) for name, cid in CHECKS.items()})
    return s


def select(top, name, index):
    h = control(top, COMBOS[name])
    user32.SendMessageW(h, CB_SETCURSEL, index, 0)
    # tell the page the user picked it, with the notifications a mouse selection sends
    for code in (CBN_SELENDOK, CBN_SELCHANGE, CBN_CLOSEUP):
        user32.SendMessageW(user32.GetParent(h), WM_COMMAND, (code << 16) | COMBOS[name], h)


def click(top, name):
    user32.SendMessageW(control(top, CHECKS[name]), BM_CLICK, 0, 0)


def main():
    if len(sys.argv) < 2 or sys.argv[1] not in ("status", "sweep"):
        sys.exit(__doc__)
    top = dialog()
    if sys.argv[1] == "status":
        s = state(top)
        for name, cid in COMBOS.items():
            items = combo_items(control(top, cid))
            print(f"{name:18} {items[s[name]] if 0 <= s[name] < len(items) else s[name]}  (options: {items})")
        for name in CHECKS:
            print(f"{name:18} {'on' if s[name] else 'off'}")
        print(f"{'current clock':18} {text(control(top, CURRENT_CLOCK))}")
        return

    out = os.path.abspath(sys.argv[2])
    start = state(top)
    print("starting state:", start)
    for stale in (out + ".log", out + ".stop"):
        if os.path.exists(stale):
            os.remove(stale)
    cap = subprocess.Popen([sys.executable, "-I", os.path.join(os.path.dirname(os.path.abspath(__file__)), "capture.py"), out, "--seconds", "300"])
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
        time.sleep(3)
        for name, cid in COMBOS.items():
            items = combo_items(control(top, cid))
            for i in [i for i in range(len(items)) if i != start[name]] + [start[name]]:
                mark(f"{name} -> {items[i]}" + (" (restore)" if i == start[name] else ""))
                select(top, name, i)
                time.sleep(2.5)
                if name == "clock":
                    mark(f"  current clock shows {text(control(top, CURRENT_CLOCK))!r}")
        for name in CHECKS if "--combos-only" not in sys.argv else []:
            for label in ("toggle", "toggle back"):
                mark(f"{name} {label}")
                click(top, name)
                time.sleep(2.5)
    finally:
        now = state(top)
        for name, want in start.items():  # anything left different goes back
            if now[name] != want:
                mark(f"fix {name}")
                select(top, name, want) if name in COMBOS else click(top, name)
                time.sleep(2.5)
        mark("done")
        marks.close()
        open(out + ".stop", "w").close()
        cap.wait()
    end = state(top)
    print("restored" if end == start else f"NOT restored: {start} -> {end}")


if __name__ == "__main__":
    main()
