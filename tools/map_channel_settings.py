"""Map TotalMix channel settings to UCX register writes, changing only inaudible things.

    python tools/map_channel_settings.py re/captures/08-channel-settings.pcap [--rows input] [--channels 1]

Needs the capture service (python tools/capture.py --start-service) and TotalMix's OSC remote.
For every channel of each row it changes settings of sections that are switched off on that
channel (EQ bands, low cut frequency/slope, dynamics, auto-level parameters), so nothing reaches
the signal. Section on/off switches are only toggled where that can't change the sound: EQ with
all bands flat, low cut at 20 Hz, dynamics at 1:1 with 0 dB gain. Auto-level on/off, gain, pad,
Hi-Z, 48V, faders and mutes are never touched.

--audible maps the controls that do change the sound instead (gain, pad, Hi-Z, phase, reference
level, fader, pan, mute, solo, ...; see AUDIBLE), for when nothing loud is connected or playing.
Buttons are pressed twice, values swept/probed and restored. 48V is never touched.

The first channel of each row gets a sweep (0, 0.25, 0.5, 0.75, 1) per parameter to show the
value scale; the others get one change each to show the register address. Each channel is
restored and verified before moving on; the run stops if a restore doesn't verify.

Writes <out>.steps.jsonl (one line per change: time, row, channel, parameter, OSC value,
TotalMix's display value) and <out minus .pcap>.marks. Analyze with tools/map_report.py.
"""

import argparse
import json
import os
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from osc import TotalMix  # noqa: E402

ROWS = {"input": "/2/busInput", "playback": "/2/busPlayback", "output": "/2/busOutput"}
FLAT = 0.5  # eqGain*/compexpGain at 0 dB
SECTIONS = [
    # (enable switch, parameters changed while it's off, condition for toggling the switch itself)
    ("eqEnable", ["eqType1", "eqGain1", "eqFreq1", "eqQ1", "eqGain2", "eqFreq2", "eqQ2", "eqType3", "eqGain3", "eqFreq3", "eqQ3"],
     lambda s: all(v(s, p) == FLAT for p in ("eqGain1", "eqGain2", "eqGain3"))),
    ("lowcutEnable", ["lowcutFreq", "lowcutGrade"], lambda s: v(s, "lowcutFreq") == 0.0),
    ("compexpEnable", ["compexpGain", "compexpAttack", "compexpRelease", "compTrsh", "compRatio", "expTrsh", "expRatio"],
     lambda s: v(s, "compRatio") == 0.0 and v(s, "expRatio") == 0.0 and v(s, "compexpGain") == FLAT),
    ("alevEnable", ["alevMaxgain", "alevHeadroom", "alevRisetime"], lambda s: False),
]
SWEEP = [0.0, 0.25, 0.5, 0.75, 1.0]
# --audible: controls that change the sound, for when nothing loud is connected or playing.
# Never phantom (osc.py refuses it anyway). Not stereo/cue/talkbackSel/recordEnable/playChannel/
# noTrim: those change TotalMix's layout or behaviour rather than a DSP value.
AUDIBLE = ["gain", "gainRight", "pad", "instrument", "phase", "phaseRight", "refLevel", "autoset", "msProc",
           "width", "loopback", "alevEnable", "crossFeed", "volume", "pan", "mute", "solo", "reverbSend", "reverbReturn"]


def v(state, param):
    return state.get("/2/" + param, [None])[0]


def clamp(value):
    """TotalMix can report values outside 0..1 (reverbSend at -oo reads -3.6154) but ignores
    them when sent back; the in-range end shows the same setting."""
    return min(max(value, 0.0), 1.0)


class Run:
    def __init__(self, out, gap, audible=False):
        self.tm, self.gap, self.audible = TotalMix(), gap, audible
        self.marks = open(os.path.splitext(out)[0] + ".marks", "w", encoding="utf-8")
        self.steps = open(out + ".steps.jsonl", "w", encoding="utf-8")

    def mark(self, label):
        t = time.time()
        self.marks.write(f"{t:.3f}\t{time.strftime('%H:%M:%S', time.localtime(t))}.{int(t * 1000) % 1000:03d}\t{label}\n")
        self.marks.flush()
        return t

    def set(self, row, channel, param, value, kind):
        """Send one change, note TotalMix's display value, wait out the gap."""
        t = self.mark(f"{row} {channel} {param} {value} ({kind})")
        self.tm.send("/2/" + param, value)
        replies = self.tm.receive(self.gap)
        shown = replies.get("/2/" + param + "Val", [None])[0]
        self.steps.write(json.dumps({"t": round(t, 3), "row": row, "channel": channel, "param": param,
                                     "value": value, "kind": kind, "shown": shown}) + "\n")
        self.steps.flush()

    def same(self, before, now, param):
        a = "/2/" + param
        if a + "Val" in before:  # compare what TotalMix displays; raw floats come back rounded
            return before.get(a + "Val") == now.get(a + "Val")
        return before.get(a) == now.get(a)

    def channel(self, row, channel, sweep):
        before = self.tm.read_page("/2")
        if v(before, "trackname") != channel:
            raise SystemExit(f"expected {channel}, TotalMix shows {v(before, 'trackname')}; stopping")
        touched, toggled = [], []
        try:
            for p in AUDIBLE if self.audible else []:
                cur = v(before, p)
                if cur is None:
                    continue
                touched.append(p)
                if "/2/" + p + "Val" not in before:  # a button: press twice, back where it was
                    self.set(row, channel, p, 1.0, "press")
                    toggled.append(p)
                    self.set(row, channel, p, 1.0, "press back")
                    toggled.remove(p)
                    continue
                values = [x for x in SWEEP if abs(x - cur) > 0.01] if sweep else [0.75 if abs(cur - 0.75) > 0.01 else 0.25]
                for x in values:
                    self.set(row, channel, p, x, "sweep" if sweep else "probe")
                self.set(row, channel, p, clamp(cur), "restore")
            for switch, params, toggle_ok in SECTIONS if not self.audible else []:
                if v(before, switch) != 0.0:
                    self.mark(f"{row} {channel} skip {switch} section: it's on")
                    continue
                for p in params:
                    cur = v(before, p)
                    if cur is None:
                        continue
                    values = [x for x in SWEEP if abs(x - cur) > 0.01] if sweep else [0.75 if abs(cur - 0.75) > 0.01 else 0.25]
                    for x in values:
                        self.set(row, channel, p, x, "sweep" if sweep else "probe")
                    touched.append(p)
                    self.set(row, channel, p, clamp(cur), "restore")
                if toggle_ok(before):
                    self.set(row, channel, switch, 1.0, "toggle on")
                    toggled.append(switch)
                    self.set(row, channel, switch, 1.0, "toggle off")
                    toggled.remove(switch)
        finally:
            for switch in toggled:  # interrupted between the two presses
                self.set(row, channel, switch, 1.0, "toggle back")
        now = self.tm.read_page("/2")
        wrong = [p for p, *_ in SECTIONS if not self.same(before, now, p)] + [p for p in touched if not self.same(before, now, p)]
        if wrong:
            diffs = ", ".join(f"{p} {before.get('/2/' + p + 'Val', before.get('/2/' + p))} -> {now.get('/2/' + p + 'Val', now.get('/2/' + p))}" for p in wrong)
            raise SystemExit(f"{row} {channel}: not restored: {diffs}")
        self.mark(f"{row} {channel} restored")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--rows", default="input,playback,output")
    ap.add_argument("--channels", type=int, default=0, help="only the first N channels of each row (0 = all)")
    ap.add_argument("--gap", type=float, default=0.45, help="seconds per change")
    ap.add_argument("--audible", action="store_true", help="map the audible controls instead (nothing loud may be connected or playing)")
    ap.add_argument("--resume-at", metavar="ROW:CHANNEL", help='skip the channels before this one, e.g. "input:AN 7/8"')
    a = ap.parse_args()
    out = os.path.abspath(a.out)
    run = Run(out, a.gap, a.audible)
    tm = run.tm

    start = tm.read_page("/2")
    start_row = next((r for r, addr in ROWS.items() if start.get(addr) == [1.0]), "input")
    start_name = v(start, "trackname")

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

    try:
        run.mark("baseline")
        time.sleep(2)
        rows = a.rows.split(",")
        resume_row, _, resume_name = (a.resume_at or "").partition(":")
        skipping = bool(a.resume_at)
        for row in rows:
            if skipping and row != resume_row and rows.index(row) < rows.index(resume_row):
                continue
            tm.send(ROWS[row], 1.0)
            for _ in range(40):
                tm.send("/2/track-", 1.0)
            seen = []
            while True:
                name = v(tm.read_page("/2", 1.0), "trackname")
                if (seen and name == seen[-1]) or (a.channels and len(seen) >= a.channels):
                    break
                if skipping and name != resume_name:  # mapped in an earlier run
                    seen.append(name)
                    tm.send("/2/track+", 1.0)
                    continue
                skipping = False
                run.mark(f"{row} {name} selected")
                print(f"{time.strftime('%H:%M:%S')} {row} {name}", flush=True)
                run.channel(row, name, sweep=not seen)
                seen.append(name)
                tm.send("/2/track+", 1.0)
    finally:
        # put the selection back where it was
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
    print("all channels restored and verified")


if __name__ == "__main__":
    main()
