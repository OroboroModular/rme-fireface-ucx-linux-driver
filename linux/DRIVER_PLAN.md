# Linux driver plan

What the findings in `re/NOTES.md` mean for the Linux side. Both directions of the iso path have
been measured on Linux (`linux/iso_probe.py`, `linux/out_timing.py`), and a userspace driver
streams both ways cleanly and puts the unit into PipeWire (`crates/ucx-stream`, below,
2026-10-24). A kernel driver doing the same as an ALSA card is written and builds, but hasn't been
loaded yet (`linux/kernel`, below). The packet formats are in `ucx_protocol::iso`.

## Split

Same split as the mainline RME Digiface USB support (`sound/usb/quirks-table.h`, `QUIRK_RME_DIGIFACE`):

- **Kernel: audio streaming** for `2a39:3fc9`. It claims interface 0 (isochronous audio) and
  interface 2 (class-compliant USB-MIDI) and handles sample rates. Unlike the Digiface, the packet
  format isn't one `snd-usb-audio` can copy as it is (below).
- **Userspace: the mixer app** (Rust, `crates/ucx-protocol` + a libusb transport + the Oroboro-style UI).
  It claims interface 1 (register words on bulk `0x0c`/`0x8d`, meters on `0x85`) and uses vendor
  requests on endpoint 0 for the status word, clock source and options.

Nothing binds the unit today: `snd-usb-audio` matches AudioControl interfaces only, and the UCX's
MIDI interface (class 01/03) has none, so even MIDI needs the driver to claim it.

## Streaming, as measured

| Item | Value |
|------|-------|
| Streaming interface | 0, alt 1 / 2 / 3 for 1× / 2× / 4×. The unit starts streaming IN as soon as the alt setting is selected |
| Endpoints | iso OUT `0x0e`, iso IN `0x8f`, one packet per microframe. Descriptors say "no sync": async, with implicit feedback from the IN frame counts |
| IN packet | 4-byte header (frame count in bits 0–4, frame position mod 3072 in bits 5–16, 60 MHz timestamp in bits 21–31), then N frames **planar** (all of channel 1, then channel 2, …) |
| Sample format | **float32 LE**, 1.0 = full scale, slots in DSP order: analog 1–8, SPDIF L/R, ADAT 1–8 |
| Channels | 18 / 14 / 12 per direction at 1× / 2× / 4× (verified both ways) |
| OUT packet | N frames **interleaved**, each = a u32 lead word + one float32 per channel. The first frame's lead word is the frame time the packet plays at |
| OUT timing | The unit plays from a 128-frame ring at its own frame clock (the IN header's position): a packet with lead word p plays at the next frame time ≡ p mod 128. Against A, the IN position after the IN packet of the same microframe, whole packets play on time for p − A from about 2–8 frames (≈ 40 µs) to 128 minus the packet's frames; `iso::OUT_LEAD` = 60 is mid-range at every rate, ±2 microframes at 192 kHz |
| Missing OUT frames | Play silence: the unit clears a ring slot once it has played it, so a late or missing packet is a dropout, never stale audio |
| URB queue | At least 4 ms of iso URBs in flight per direction (on xHCI): with 2 ms IN loses packets and OUT goes off target, 3 ms is marginal |
| First IN packets | Can be stale when IN starts a few ms after SET_INTERFACE: position 0 or an overflow, then a jump to the real position |
| Rates | 32 / 44.1 / 48, 64 / 88.2 / 96, 128 / 176.4 / 192 kHz (both directions verified at all nine) |
| Rate change | req 16 `wValue = base << 4`, `wIndex = 0x0030` (base 0 = 32k, 1 = 44.1k, 2 = 48k family) plus the alt setting |
| Rate readback | req 17 (IN, 4 bytes), top byte = (speed << 2) | base, same table as `rme_digiface_rate_table` |
| Stream start/stop | IN needs nothing. OUT needs req 20 with the playback channel mask (`ucx_protocol::control::stream_start(channels)`, which also sends 16 `0x8000` and 29 as Windows does); stop: req 19 with the same mask |
| Power-on init | Not needed for either direction. Several of its writes are still not understood |

## Kernel side: where the packet handling goes

`snd-usb-audio` copies interleaved frames straight between the ring buffer and the URBs and has no
hook for a per-packet header, a planar IN layout, a lead word per OUT frame, or OUT timestamps
that must match the unit's frame clock. ALSA's `FLOAT_LE` itself is fine. Options:

1. **A quirk plus a packet-copy hook in `snd-usb-audio`**: least new code, but it changes the core
   copy paths (`prepare_playback_urb`, `retire_capture_urb`), so it's the hardest to upstream.
2. **A small driver of its own under `sound/usb/`**, like `hiface` or `line6`: its own iso URBs and
   PCM, header and planar handling done there. More code, but self-contained.
3. **Userspace streaming** (usbfs or libusb into PipeWire or JACK): no kernel work at all, but
   iso timing and latency in userspace are weaker. Neither libusb nor usbfs reports which
   microframe an iso transfer went out in (usbfs never copies `start_frame` back), but the reap
   times give the offset between the OUT and IN streams to a fraction of a microframe
   (`out_timing.py`), which is enough. Done: `ucx-stream` (below).

The hard part is the same in all three: OUT lead words have to follow the unit's frame clock.
The kernel has what it needs: each IN packet's position, and each URB's `start_frame` (exact
when submitted without `URB_ISO_ASAP`, which on xHCI also starts the stream on a frame
boundary). From those, a driver knows the unit's position A in the microframe each OUT packet
goes out in and gives the packet the lead word A + `OUT_LEAD`, sizing packets so the lead words
run on without gaps (they then follow the IN frame counts, as implicit feedback would).

The userspace driver is done and is the reference to check a kernel driver against. Option 2 is
written (`snd-fireface-ucx`, below) and waits for its first load.

## Userspace driver: `ucx-stream` (`crates/ucx-stream`)

Streams interface 0 through usbfs, with nothing in the kernel, and puts the unit into PipeWire as
the sink and source "Fireface UCX".

- **Engine** (`engine.rs`): IN and OUT iso URBs without `URB_ISO_ASAP`, reaped by one SCHED_FIFO
  thread. The IN headers give the unit's position per microframe (re-anchored after the stale
  first packets). OUT starts 50 ms after IN, silent, until 200 ms of reap times give the offset
  between the two streams; from then on each OUT packet's lead word is the unit's predicted
  position in its microframe plus `OUT_LEAD`, and its size is whatever keeps the next packet on
  target, so the sizes follow the unit's frame counts. The offset is measured again every
  100 ms; a change, or a queue running dry, starts streaming over.
- **PipeWire** (`pipewire.rs`): two `pw_filter`s with one float port per channel (AN1–8,
  SPDIF_L/R, ADAT1–8), in one node group, both able to drive (the sink drives). One channel pair
  is FL/FR (`--main-pair`, analog 1/2 by default), the others are AUXn. WirePlumber 0.5 links
  playback streams to the sink's ports by those names, but recording streams to the source's
  first ports whatever their names, so stereo recordings get analog 1/2. The streaming thread starts each graph cycle as soon as its frames have been
  recorded: it writes the graph clock, forcing the unit's rate and its own cycle length as
  PipeWire's JACK tunnel does, and calls `pw_filter_trigger_process`. So the graph runs on the
  unit's clock with nothing to resample and no drift, and both directions use the unit's
  positions: a cycle that records positions P .. P + n plays its output at P + the play delay
  (two cycles + the URB queues + `OUT_LEAD` + 1 ms: 19 ms at 48 kHz with 256-frame cycles).
- libpipewire is loaded at run time and the few structs it needs are copied from the PipeWire 1.6
  headers, so no `-dev` packages are needed.
- WirePlumber links streams only to a filter node that has `EnumFormat`/`Format` params and
  answers `PortConfig`: before linking it suspends the node, sets PortConfig and waits for new
  ports or, when the node has its ports already, for a `Props` update
  (`module-si-audio-adapter.c`). The filters answer with Props (not muted, every volume 1.0);
  volume set from outside is ignored, levels belong to the unit's own mixer.
- `priority.session` is low, so WirePlumber never makes the unit the default device by itself.

Checked (`re/captures/25-*.log`):

- `ucx-stream selftest` at all nine rates, 60 s at 192 kHz and 300 s at 48 kHz: every frame of
  the loopback code on time, except once at 32 kHz when the host held the stream up for 8 ms
  (`re/NOTES.md`, "Streaming both ways from userspace").
- `linux/pipewire_check.py` (`25-pipewire-check.log`): the code played by pw-cat into the sink
  comes back from the source through the loopback, every frame once and in order, in 14 of 15
  runs: 3 at 48 kHz, 44.1, 96, 2 at 192 kHz, 5 at 48 kHz with 128-frame cycles and 2 of 3 with
  64-frame cycles. In the third 64-frame run one cycle's output came 2 frames too late for its
  OUT packet, and the code broke there: with such short cycles the default play delay leaves the
  graph about 2.3 ms (`--play-delay` gives it more). Earlier runs at 128 frames also caught
  pw-cat itself missing cycles (PipeWire counted xruns on its nodes), so a failed check with
  short cycles isn't necessarily the bridge's.

- **Latency**: each port reports it as PipeWire's ALSA nodes do, one cycle plus what the
  bridge adds past it. Playback (the sink's ports, input direction): `play_delay` less two
  cycles and an IN URB, so the OUT lead, the OUT queue and 1 ms (396 frames at 48 kHz with the
  defaults). Capture (the source's ports, output direction): an IN URB (12 frames). Seen in
  `pw-dump`.

Not done yet: rate changes while running (it runs at the rate it was started with, and PipeWire
resamples streams to it) and MIDI (interface 2). Unplugging ends it; as a user service
(`linux/ucx-stream.service`) it's started again when the unit is back.

## Kernel driver: `snd-fireface-ucx` (`linux/kernel`)

Option 2: a small ALSA USB driver of its own (`fireface_ucx.c`), doing what `ucx-stream`'s engine
does, in the kernel. Written 2026-10-24; it builds against kernel 7.0 with `W=1` and passes
`checkpatch`. First loaded 2026-10-25: the code came back frame for frame through ALSA and the
loopback at all nine rates (`re/captures/28-kernel-driver-alsa-all-rates.log`), and music played
through PipeWire (Multichannel Duplex profile) with 0 errors, gaps and resyncs.

- **Binding**: interface 0 of `2a39:3fc9` (`USB_DEVICE_INTERFACE_NUMBER`), and interface 2
  (MIDI) claimed from its probe; interface 1 (mixer, meters) and the vendor requests stay free
  for userspace: `ucx-stream meters` and `ucx-stream loopback` work next to it. `ucx-stream
  selftest` and `pipewire` need interface 0, so the module has to be unloaded for them.
- **MIDI** (added 2026-10-25): interface 2 is standard USB-MIDI (class 01/03, bulk `0x07` OUT
  and `0x86` IN, two ports each way named "Port 1" and "Port 2" by their iJack strings: the
  unit's two MIDI I/Os, on four 5-pin DIN sockets of the breakout cable, User's Guide 21.3; the
  REMOTE mini-DIN is for RME's wired remote, Volume/Store/Dim/Recall, and carries no MIDI), but the
  unit has no AudioControl interface, so snd-usb-audio leaves it. The driver hands it to
  snd-usbmidi-lib with no quirk (a standard MIDIStreaming interface), as snd-ua101 does: rawmidi
  device 0 on the card, ports "Fireface UCX Port 1/2", in the ALSA sequencer and PipeWire's
  MIDI bridge. `usbmidi.h` declares the library's functions (`sound/usb/midi.h` isn't in the
  headers packages). Module parameter `midi=0` leaves the interface alone; so does a program
  holding it through usbfs at probe time. Unbinding interface 2 alone takes the whole card
  down, as in snd-ua101. Checked from userspace before the driver took it: the unit accepts
  OUT packets without any setup. Loaded 2026-10-25: both ports in `amidi -l`, the sequencer and
  Bitwig; MIDI sent from Bitwig to Port 1 / 2 blinks the front panel's Out 1 / Out 2 LED.
  MIDI input not tried yet (the breakout cable wasn't at hand).
- **ALSA**: card `FirefaceUCX`, PCM 0 with one playback and one capture substream, `FLOAT_LE`
  interleaved (the unit's own samples, copied as they are), 18 / 14 / 12 channels at 1× / 2× / 4×
  and the nine rates. Both directions run at one rate: whichever is set up second is held to the
  first's.
- **Engine**: starts on the first `prepare`, as `Engine::run` does (alt setting, base rate,
  stream start requests, IN and OUT URBs), and stops when the last direction is freed. Four URBs
  of eight microframes per direction by default (module parameters `packets`, `urbs`),
  resubmitted from their completions.
- **Timing**: one microframe timeline for both streams, from `urb->start_frame`. Without
  `URB_ISO_ASAP`, an xHCI with Contiguous Frame ID support gives every TD an explicit frame and
  reports it, contiguous while the queue doesn't run dry; a URB queued too close to the current
  frame is moved a frame later and `start_frame` says so (`xhci_queue_isoc_tx_prepare`,
  `xhci_get_isoc_frame_id` in 7.0). The Intel xHCI it was tested on has it (hcc params
  `0x20007fc1`, bit 11). So no reap-time calibration: an IN packet's microframe gives the unit's
  position, and an OUT packet's microframe gives the position to predict for its lead word.
  OUT stays empty (zero-length packets) until 400 IN packets in a row have followed on from each
  other, then runs `build_out`'s servo: lead word = predicted position + `lead` (module
  parameter, default `OUT_LEAD` = 60), frame count whatever keeps the next packet on target.
- **Capture** stays on the unit's clock: frames of IN packets that went missing become silence.
- **Latency**: playback reports the frames sent that the unit hasn't played yet as the PCM's
  delay.
- **`/proc/asound/cardN/ucx`**: rate, URB counts, IN/OUT errors, gaps, position jumps, resyncs,
  filled capture frames, and a check of the timeline: each stream's earliest completion against
  the end of its microframes, OUT minus IN, which is near 0 if `start_frame` is right (as
  `ucx-stream` lines the streams up).
- **Locking**: the completions run under the driver's spinlock and call
  `snd_pcm_period_elapsed` inside it; `trigger` and `pointer` (under ALSA's stream lock) don't
  take it. `sync_stop` takes it once after a stop, so a completion still copying has finished
  before ALSA frees or reallocates the buffer, and `close` clears the substream under it.
- **Periods** are at least one URB long (`hw_ptr` moves a URB at a time), and a stream can't
  start on an engine that lost a URB: it gets an xrun and prepares again, which restarts it.

PipeWire: alsa-lib has no profile for this card, so PipeWire offers its generic ones (Pro Audio
with all channels). A USB card usually becomes the default device by itself:
`linux/wireplumber/51-fireface-ucx.conf` keeps its nodes' `priority.session` low (like
`ucx-stream`'s), and `51-fireface-ucx-off.conf` keeps PipeWire off the card altogether, for the
first tests through ALSA alone.

Not done yet: MIDI (interface 2, through `snd-usbmidi-lib`, whose header isn't in the kernel
headers package), ALSA controls (clock source, options, loopback), a profile for PipeWire that
names the channels, and tuning the queue for lower latency.

## Open questions

1. Whether request 16 `0x8000` and 29 matter for OUT, or only request 20's mask.
2. Which power-on writes are really needed at all (req 22, 27 and the req-16 bits 0, 11, 12–13 are
   not understood).

## Running the probe

```
sudo cp linux/70-fireface-ucx.rules /etc/udev/rules.d/    # once, gives the logged-in user access
sudo udevadm control --reload && sudo udevadm trigger --attr-match=idVendor=2a39
python3 linux/iso_probe.py --info                           # reads only
python3 linux/iso_probe.py re/captures/NN-topic.log --rate 96000 --out none --meters
```

Playback through the loopback, which shows exactly what the unit plays (inaudible ramp below
−71 dBFS, but see the docstring before sending floats):

```
python3 linux/iso_probe.py --out ramp --ramp-after 8192 --out-layout framed --frame-word pos \
    --frame-offset 64 --stream-start --loopback 3,4 --meters
```

It puts the starting base rate back, turns loopback off again, and leaves interface 0 at alt 0.
It writes no mixer registers.

The OUT timing window, three runs at 192 kHz (also through the loopback, below −90 dBFS):

```
python3 linux/out_timing.py re/captures/NN-topic.log --rate 192000 --runs 3 --sweep=-24:152:2 --segment-ms 25
```

The userspace driver (`cargo build --release` first):

```
target/release/ucx-stream selftest --rate 96000 --seconds 10    # loopback 3/4, below −90 dBFS
target/release/ucx-stream pipewire --rate 48000                  # until Ctrl-C
python3 linux/pipewire_check.py --rate 48000                     # through PipeWire, inaudible
```

`pipewire` plays whatever is routed to the "Fireface UCX" sink at full scale (the sink has no
volume control), through the unit's mixer as TotalMix last left it.

The kernel driver, first load (keeps PipeWire off the card, then checks it inaudibly through ALSA
and the loopback):

```
make -C linux/kernel
mkdir -p ~/.config/wireplumber/wireplumber.conf.d
cp linux/wireplumber/51-fireface-ucx-off.conf ~/.config/wireplumber/wireplumber.conf.d/
systemctl --user restart wireplumber
sudo insmod linux/kernel/snd-fireface-ucx.ko
cat /proc/asound/cards; journalctl -k | tail                     # card FirefaceUCX
python3 linux/alsa_check.py --rate 48000                          # code through loopback 3/4
cat /proc/asound/card*/ucx                                         # the driver's counters
sudo rmmod snd_fireface_ucx    # once nothing plays: PipeWire closes the card ~5 s after
```

Installed for good, rebuilt for every kernel by DKMS, and loaded whenever the unit is plugged in
(with `51-fireface-ucx.conf` instead of the `-off` rule, so the card is in PipeWire but never the
default by itself):

```
make -C linux/kernel clean
sudo dkms add linux/kernel
sudo dkms install snd-fireface-ucx/0.1
sudo chown -R root:root /usr/src/snd-fireface-ucx-0.1    # the copy root builds from on kernel updates
```

`dkms add` copies the source to `/usr/src/snd-fireface-ucx-0.1`, so a newer driver from the repo
replaces that copy (with nothing playing through the card):

```
sudo dkms remove snd-fireface-ucx/0.1 --all
sudo rm -r /usr/src/snd-fireface-ucx-0.1
make -C linux/kernel clean
sudo dkms add linux/kernel
sudo dkms install snd-fireface-ucx/0.1
sudo chown -R root:root /usr/src/snd-fireface-ucx-0.1
sudo rmmod snd_fireface_ucx && sudo modprobe snd-fireface-ucx
```

While it's installed it binds the unit's audio interface whenever the unit is plugged in, so
`ucx-stream selftest` and `pipewire` need `sudo rmmod snd_fireface_ucx` first.

Like `ucx-stream pipewire`, the card plays whatever reaches it at full scale, through the unit's
mixer as TotalMix last left it.

## Not covered yet

External clock lock/sync bits in the status word (needs a cable, e.g. optical out → optical in),
the idle poll `0x3f00|N` and its replies, DSP-internal FX (reverb/echo parameters, page 3 of OSC),
and TotalMix's snapshot/workspace handling (likely pure host-side state).
