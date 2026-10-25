"""End-to-end check of the snd-fireface-ucx kernel driver through ALSA alone, inaudible: a
position code goes into the card's playback and back out of its capture through the unit's
loopback, and every frame of it has to come back once, in order.

    python3 linux/alsa_check.py [--rate 48000 | --all-rates] [--pair 3] [--seconds S] [--period N] [--buffer N] [LOG]

Needs the module loaded (`make -C linux/kernel`, `sudo insmod linux/kernel/snd-fireface-ucx.ko`)
and `target/release/ucx-stream` built, which turns loopback on for outputs 3/4 meanwhile (vendor
requests need no interface, so it works next to the driver). PipeWire is asked to let go of the
card for the run (pw-reserve); with linux/wireplumber/51-fireface-ucx-off.conf it never takes it.

aplay plays all channels with the code on playback 3/4 (the selftest's: 128 + q mod 128 and
128 + (q >> 7) mod 128 LSB of 24 bits, -96 to -90 dBFS) and zeros elsewhere, so nothing else
plays; arecord records all channels, and the code has to come back on capture 3/4. The driver's
own counters (/proc/asound/cardN/ucx) are printed at the end.

Loopback records the output after the mixer, so the code comes back only while the mix routes
playback 3/4 to outputs 3/4 at 0 dB, with that output's processing off (TotalMix's mix does).
`--pair 5` uses playback 5/6, outputs 5/6 and capture 5/6 instead, for when 3/4 is mixed otherwise.
"""

import argparse
import array
import os
import re
import signal
import subprocess
import sys
import tempfile
import time

from pipewire_check import check_code, write_code

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
UCX_STREAM = os.path.join(ROOT, "target", "release", "ucx-stream")


def find_card():
    """The card number of the Fireface UCX, from /proc/asound/cards."""
    with open("/proc/asound/cards") as f:
        for line in f:
            m = re.match(r"\s*(\d+) \[(\S+)\s*\]: (\S+) - ", line)
            if m and m.group(3) == "FirefaceUCX":
                return int(m.group(1))
    return None


def channels(rate):
    return 18 if rate <= 48000 else 14 if rate <= 96000 else 12


RATES = [32000, 44100, 48000, 64000, 88200, 96000, 128000, 176400, 192000]


def silent(path, channels, first):
    """True if channels `first` and `first` + 1 of the recording (interleaved float32) are all zero."""
    a = array.array("f")
    with open(path, "rb") as f:
        a.frombytes(f.read())
    return not any(a[i + c] for i in range(0, len(a) - channels + 1, channels) for c in (first, first + 1))


def check_rate(card, rate, seconds, period, buffer, say, pair=3):
    """One run at `rate`: the code played and recorded through the card and the loopback on
    outputs `pair`/`pair + 1` (1-based), every frame checked. True if clean."""
    n = channels(rate)
    first = pair - 1
    frames = int(rate * seconds)
    tmp = tempfile.mkdtemp(prefix="ucx-alsa-check-")
    play, rec = os.path.join(tmp, "play.raw"), os.path.join(tmp, "rec.raw")
    write_code(play, frames, channels=n, first=first)
    fmt = ["-q", "-D", f"hw:{card},0", "-t", "raw", "-f", "FLOAT_LE", "-c", str(n), "-r", str(rate)]
    if period:
        fmt += ["--period-size", str(period)]
    if buffer:
        fmt += ["--buffer-size", str(buffer)]
    looped = f"{pair},{pair + 1}"
    say(f"card {card}, {rate} Hz, {n} channels, {seconds} s of code on playback {pair}/{pair + 1} "
        f"through loopback {pair}/{pair + 1}")

    reserve = subprocess.Popen(["pw-reserve", "-n", f"Audio{card}", "-r"], stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
    time.sleep(1.0)
    recorder = None
    try:
        subprocess.run([UCX_STREAM, "loopback", looped], check=True, stderr=subprocess.DEVNULL)
        recorder = subprocess.Popen(["arecord", *fmt, rec])
        time.sleep(1.0)
        subprocess.run(["aplay", *fmt, play], check=True)
        time.sleep(0.5)
    finally:
        if recorder:
            recorder.send_signal(signal.SIGINT)
            recorder.wait()
        subprocess.run([UCX_STREAM, "loopback", "off"], stderr=subprocess.DEVNULL)
        reserve.terminate()
        reserve.wait()
    ok = check_code(rec, frames, say, channels=n, first=first)
    with open(f"/proc/asound/card{card}/ucx") as f:
        for line in f:
            say("driver: " + line.rstrip())
    say("clean" if ok else "NOT clean")
    if not ok and silent(rec, n, first):
        say(f"capture {pair}/{pair + 1} was all zeros: loopback was on but nothing reached outputs "
            f"{pair}/{pair + 1}; is playback {pair}/{pair + 1} routed there at 0 dB in the mix? "
            f"(or try another --pair)")
    return ok


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("log", nargs="?", help="also write the summary here")
    ap.add_argument("--rate", type=int, default=48000)
    ap.add_argument("--all-rates", action="store_true", help="one run at each of the nine rates, in turn")
    ap.add_argument("--pair", type=int, default=3, choices=[1, 3, 5, 7],
                    help="the analog pair the code goes through (3 = playback, outputs and capture 3/4)")
    ap.add_argument("--seconds", type=float, help="per run (10 s, or 5 with --all-rates)")
    ap.add_argument("--period", type=int, help="period size in frames (aplay/arecord's default if not given)")
    ap.add_argument("--buffer", type=int, help="buffer size in frames")
    args = ap.parse_args()
    lines = []

    def say(s):
        print(s, flush=True)
        lines.append(s)

    card = find_card()
    if card is None:
        raise SystemExit("no FirefaceUCX card in /proc/asound/cards: is snd-fireface-ucx loaded?")
    rates = RATES if args.all_rates else [args.rate]
    seconds = args.seconds or (5.0 if args.all_rates else 10.0)
    results = []
    for rate in rates:
        results.append((rate, check_rate(card, rate, seconds, args.period, args.buffer, say, args.pair)))
    if len(results) > 1:
        say("summary: " + ", ".join(f"{r} {'clean' if ok else 'NOT clean'}" for r, ok in results))
    if args.log:
        with open(args.log, "w") as f:
            f.write("\n".join(lines) + "\n")
    sys.exit(0 if all(ok for _, ok in results) else 1)


if __name__ == "__main__":
    main()
