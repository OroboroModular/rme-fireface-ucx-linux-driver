"""End-to-end check of `ucx-stream pipewire`, inaudible: a position code goes through PipeWire
into the unit and back, and every frame of it has to come back once, in order.

    python3 linux/pipewire_check.py [--rate 48000] [--quantum 256] [--seconds 10] [LOG]

Starts `target/release/ucx-stream pipewire --main-pair 3 --loopback 3,4` (build it first:
`cargo build --release`), so stereo streams go to and come from AN3/AN4, plays the code with pw-cat
into the "Fireface UCX" sink and records the source with another pw-cat; the unit's loopback
copies outputs 3/4 into recording channels 3/4. The code is the selftest's
(crates/ucx-stream/src/selftest.rs): frame q carries 128 + q mod 128 and 128 + (q >> 7) mod 128
LSB of 24 bits, -96 to -90 dBFS, nothing else plays.

Seen with PipeWire 1.6.2 / WirePlumber 0.5.13: a playback stream is linked to the sink's ports
by channel name (FL/FR, hence `--main-pair 3`), but a recording stream to the source's first
ports whatever their names, so the stereo recorder is relinked by hand from AN1/AN2 to AN3/AN4,
and the code only starts once those links are running. (pw-cat records channels at AUX
positions as silence, so a wider recorder doesn't help.)
"""

import argparse
import array
import json
import os
import signal
import subprocess
import sys
import tempfile
import time

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
BINARY = os.path.join(ROOT, "target", "release", "ucx-stream")
SINK, SOURCE = "fireface_ucx.playback", "fireface_ucx.capture"
REC, PLAY = "ucx_check_rec", "ucx_check_play"
LSB = 2.0 ** -23
CODE = 1 << 14


def write_code(path, frames, channels=2, first=0):
    """Interleaved float32 frames with the code on channels `first` and `first` + 1, zeros
    elsewhere."""
    a = array.array("f", [0.0]) * (channels * frames)
    for q in range(frames):
        a[channels * q + first] = (128 + q % 128) * LSB
        a[channels * q + first + 1] = (128 + (q >> 7) % 128) * LSB
    with open(path, "wb") as f:
        a.tofile(f)


def check_code(path, played, say, channels=2, first=0):
    """True if the recording (interleaved float32) holds all `played` frames of the code on
    channels `first` and `first` + 1, from code 0 on, each one following the one before."""
    a = array.array("f")
    with open(path, "rb") as f:
        a.frombytes(f.read())
    codes = []
    for i in range(len(a) // channels):
        x, y = round(a[channels * i + first] / LSB), round(a[channels * i + first + 1] / LSB)
        codes.append((y - 128) << 7 | (x - 128) if 128 <= x < 256 and 128 <= y < 256 else None)
    start = next((i for i in range(len(codes) - 64) if all(codes[i + j] == j for j in range(64))), None)
    if start is None:
        say(f"recorded {len(codes)} frames; the code never came back")
        return False
    n = 0
    while start + n < len(codes) and codes[start + n] == n % CODE:
        n += 1
    say(f"recorded {len(codes)} frames; the code starts at recorded frame {start} and runs on for {n} of the "
        f"{played} frames played")
    if n < played:
        after = codes[start + n:start + n + 8]
        say(f"  at code {n % CODE} (frame {n} played) came {after}")
    return n == played


def node_up(name):
    out = subprocess.run(["pw-dump"], capture_output=True, text=True).stdout
    return any(o.get("type") == "PipeWire:Interface:Node" and o["info"]["props"].get("node.name") == name
               for o in json.loads(out or "[]"))


def links():
    return subprocess.run(["pw-link", "-l"], capture_output=True, text=True).stdout


def active(pairs):
    """True if a link runs from each (output port, input port) pair, by port name."""
    out = json.loads(subprocess.run(["pw-dump"], capture_output=True, text=True).stdout or "[]")
    ports = {o["id"]: o["info"]["props"].get("port.name") for o in out if o.get("type") == "PipeWire:Interface:Port"}
    running = {(ports.get(o["info"]["output-port-id"]), ports.get(o["info"]["input-port-id"]))
               for o in out if o.get("type") == "PipeWire:Interface:Link" and o["info"].get("state") == "active"}
    return all(p in running for p in pairs)


def wait_for(test, what, timeout=10.0):
    end = time.monotonic() + timeout
    while not test():
        if time.monotonic() > end:
            raise SystemExit(f"timed out waiting for {what}")
        time.sleep(0.1)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("log", nargs="?", help="also write the summary here")
    ap.add_argument("--rate", type=int, default=48000)
    ap.add_argument("--quantum", type=int, default=256)
    ap.add_argument("--seconds", type=float, default=10.0)
    args = ap.parse_args()
    lines = []

    def say(s):
        print(s, flush=True)
        lines.append(s)

    tmp = tempfile.mkdtemp(prefix="ucx-check-")
    play, rec, bridge_log = (os.path.join(tmp, n) for n in ("play.raw", "rec.raw", "bridge.log"))
    write_code(play, int(args.rate * args.seconds))
    fmt = ["--raw", "--format", "f32", "--rate", str(args.rate)]
    say(f"{args.rate} Hz, quantum {args.quantum}, {args.seconds} s of code through PipeWire and loopback 3/4")
    with open(bridge_log, "w") as log:
        bridge = subprocess.Popen([BINARY, "pipewire", "--rate", str(args.rate), "--quantum", str(args.quantum),
                                   "--main-pair", "3", "--loopback", "3,4", "--seconds", str(args.seconds + 30)],
                                  stderr=log)
    recorder = None
    try:
        wait_for(lambda: node_up(SOURCE) and node_up(SINK), "the Fireface UCX nodes")
        recorder = subprocess.Popen(["pw-cat", "--record", *fmt, "--channels", "2", "--target", SOURCE,
                                     "-P", f"{{ node.name={REC} }}", rec], stderr=subprocess.DEVNULL)
        wait_for(lambda: f"{REC}:input_FR" in links(), "the recorder's links")
        for old, new, port in (("AN1", "AN3", "FL"), ("AN2", "AN4", "FR")):
            subprocess.run(["pw-link", "-d", f"{SOURCE}:capture_{old}", f"{REC}:input_{port}"], check=True)
            subprocess.run(["pw-link", f"{SOURCE}:capture_{new}", f"{REC}:input_{port}"], check=True)
        wait_for(lambda: active([("capture_AN3", "input_FL"), ("capture_AN4", "input_FR")]), "the new links")
        time.sleep(0.5)
        subprocess.run(["pw-cat", "--playback", *fmt, "--channels", "2", "--target", SINK,
                        "-P", f"{{ node.name={PLAY} }}", play], check=True)
        time.sleep(0.5)
    finally:
        if recorder:
            recorder.send_signal(signal.SIGINT)
            recorder.wait()
        bridge.send_signal(signal.SIGINT)
        bridge.wait()
    with open(bridge_log) as f:
        report = [line.rstrip() for line in f if " Hz; graph:" in line or "lost" in line]
    if report:
        say("bridge at the end: " + report[-1])
    ok = check_code(rec, int(args.rate * args.seconds), say) and bridge.returncode == 0
    say("clean" if ok else "NOT clean")
    if args.log:
        with open(args.log, "w") as f:
            f.write("\n".join(lines) + "\n")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
