# Fireface UCX (USB, non-CC) — reverse-engineering notes

## Device

- USB `2a39:3fc9`, high speed, bcdDevice 0x0001, product string `Fireface UCX (<serial>)`, the unit's serial number in brackets.
- Windows driver: RME "Fireface USB" 1.2.76 (`fireface_usb_amd.sys`, service `firefaceu`). One
  driver covers UC, UCX, UFX, 802, Babyface (Pro), UCX II, so the control protocol is shared.
- Raw descriptors: `descriptors/2a39_3fc9_*.bin`, decoded: `descriptors/2a39_3fc9.txt`
  (dumped with `tools/win_usb_descriptors.py`, which asks the hub and leaves the driver bound).

## Interface layout (one configuration, 6 interfaces)

| If | Alt | Endpoints | Guess |
|----|-----|-----------|-------|
| 0 | 0–3 | iso OUT `0x0e`, iso IN `0x8f` | Audio streaming, "Mac-style" isochronous. Alt 1/2/3 = 1x/2x/4x speed |
| 1 | 0 | bulk OUT `0x0c`, bulk IN `0x8d`, bulk IN `0x85` | DSP / mixer / meters? |
| 2 | 0 | bulk OUT `0x07`, bulk IN `0x86` | **Standard USB-MIDI** (class 01/03), 2 in + 2 out jacks, "Port 1" / "Port 2" = MIDI I/O 1 / 2 on the breakout cable (snd-fireface-ucx's MIDI) |
| 3 | 0 | bulk IN `0x88`, bulk OUT `0x09` | Unknown (firmware flash? second control channel?) |
| 4 | 0 | interrupt IN `0x8b`, 4 bytes | Status / event notification? |
| 5 | 0–3 | intr OUT `0x01`, intr IN `0x82`, bulk/intr `0x03`/`0x84` | Audio streaming as used by the **Windows** driver |

While Windows is running, the driver has interface 0 at alt 0 (zero bandwidth) and interface 5 at
alt 1, so Windows streams over interface 5, not the iso endpoints.

### Packet-size arithmetic (hypotheses, all 4-byte samples)

UCX channel counts per direction: 18 at 1x (8 analog + 2 SPDIF + 8 ADAT), 14 at 2x (ADAT S/MUX),
12 at 4x (ADAT S/MUX4).

Interface 0 (iso, frames per 125 µs microframe = nominal + 1):

| Alt | Speed | IN max | = | OUT max | = |
|-----|-------|--------|---|---------|---|
| 1 | ≤48k  | 508  | 7 × 18 × 4 + 4  | 532  | 7 × 19 × 4 |
| 2 | ≤96k  | 732  | 13 × 14 × 4 + 4 | 780  | 13 × 15 × 4 |
| 3 | ≤192k | 1208 (604×2) | 25 × 12 × 4 + 8 | 1304 (652×2) | 25 × 13 × 4 + 4 |

IN fits "N frames × channels × 4 + one 4-byte trailer per transaction" exactly (the UC project
saw the same 4-byte field). OUT appears to have one extra 4-byte slot per frame. Unconfirmed.

Measured on Linux (see "Isochronous streaming, interface 0" below): IN is a 4-byte *header* per
microframe, then N frames of planar float32. OUT is N interleaved frames of a 4-byte lead word plus
float32 samples, which is the "extra slot per frame". The max sizes leave room for one frame more
than nominal, plus 4 spare bytes at 4×.

Interface 5 (Windows): 576 = 8 × 18 × 4, 896 = 16 × 14 × 4, 2 × 768 = 32 × 12 × 4.

## Idle traffic (first look, 3 s test capture, 2026-10-23)

No audio streaming while nothing plays (interface 5 silent). The Windows driver polls four vendor
IN requests every ~16 ms, each `wValue 0 wIndex 0 wLength 4`, in this order:

| bRequest | Reply (bytes) | Meaning |
|----------|---------------|---------|
| 28 (0x1c) | `03 01 31 00` | Constant. The UC project calls 28 "get firmware version" |
| 30 (0x1e) | `3c 48 e7 e9` | Serial number, low part: LE `0xe9e7483c` → `E9E7483C` |
| 31 (0x1f) | `ss ss ss a5` | Serial number, high part: the bytes `ss ss ss` little-endian are the USB serial string's first six hex digits |
| 17 (0x11) | `00 ff f0 01` | Status word, LE `0x01f0ff00` (the Digiface driver also reads status with 17) |

Interface 1 bulk also runs constantly (probably TotalMix): 4-byte writes on `0x0c`
(`NN 00 00 3f|bf`, first byte counts up 0, 1, 2…) answered by bursts of 4-byte words on `0x8d`.
Looks like a register read/meter protocol; session 02 (TotalMix open vs closed) should tell.

## Interface 1 word format

Little-endian 32-bit words on both `0x0c` (out) and `0x8d` (in). **Bit 31 is odd parity over the
word**: all 23,300 words in `captures/01-init-a.pcap` check out. Bits 30–24 look like a
command/register field: `0x3f` for the idle poll (the low byte is a counter 0x0–0xf), `0x40`–`0x47`
and `0x12`–`0x23` in the state push.

Working hypothesis from the first OSC probe: **bits 30–16 = register address, bits 15–0 = value**.

### Channel settings: register = (DSP channel << 8) | parameter

Confirmed by `captures/08-settings-all.pcap`: 1,751 OSC changes across all 29 TotalMix strips.
1,705 produced exactly the predicted writes. The other 46 were selector values that landed on the
option already selected, so nothing was sent. No other addresses, no control transfers. Full
table in `maps/channel-settings.md`.

| DSP channel | TotalMix strip |
|-------------|----------------|
| `0x00`–`0x03` | Inputs Mic 1, Mic 2, AN 3, AN 4 |
| `0x04`/`05`, `06`/`07` | Inputs AN 5/6, AN 7/8 |
| `0x08`/`09` | Input SPDIF |
| `0x0a`–`0x11` | Inputs ADAT 1–8 (AS 1/2 … ADAT 7/8) |
| `0x12`/`13` … `0x18`/`19` | Outputs AN 1/2, AN 3/4, AN 5/6, PH 7/8 ("Main") |
| `0x1a`/`1b` | Output SPDIF |
| `0x1c`–`0x23` | Outputs ADAT 1–8 |

A stereo strip writes the same parameter to both of its channels. Playback strips have no channel
settings in hardware: changing their EQ/dynamics over OSC sends nothing.

Parameter offsets (shown for channel 0, Mic 1). Offsets `0x00`–`0x1f` are still unmapped. That's
presumably gain, 48V, pad, Hi-Z, phase and the like, which are audible and weren't probed.

| Register | Setting | Value |
|----------|---------|-------|
| `0x0020` | Low cut on | 0 / 1 |
| `0x0021` | Low cut frequency | Hz (20–500) |
| `0x0022` | Low cut slope | 0–3 = 6 / 12 / 18 / 24 dB/oct |
| `0x0040` | EQ on | 0 / 1 |
| `0x0041` | Band 1 type | 0 Bell, 1 Shelf |
| `0x0042` / `43` / `44` | Band 1 gain / freq / Q | signed 0.1 dB (±200) / Hz (20–20000) / Q × 10 (7–50) |
| `0x0045` / `46` / `47` | Band 2 gain / freq / Q | same |
| `0x0048` | Band 3 type | 0 Bell, 1 Shelf, 2 Low Pass |
| `0x0049` / `4a` / `4b` | Band 3 gain / freq / Q | same as band 1 |
| `0x0060` | Dynamics on | 0 / 1 |
| `0x0061` | Dynamics gain | signed 0.1 dB (±300) |
| `0x0062` / `63` | Attack / release | ms (0–200 / 100–999) |
| `0x0064` / `65` | Compressor threshold / ratio | signed 0.1 dB (−600–0) / ratio × 10 (10–100) |
| `0x0066` / `67` | Expander threshold / ratio | signed 0.1 dB (−990 to −300) / ratio × 10 |
| `0x0080` | Auto-level on (presumed, never toggled: audible) | |
| `0x0081` / `82` / `83` | Auto-level max gain / headroom / rise time | 0.1 dB (0–180) / 0.1 dB (30–120) / 0.1 s (1–99) |

So the hardware DSP takes plain engineering units; all the dB/Hz curve mapping lives in TotalMix.

### Audible per-channel registers (first look: `captures/05-audible-first.pcap`, `maps/audible-first.md`)

Inputs (channel 0 = Mic 1 shown):

| Offset | Setting | Value |
|--------|---------|-------|
| `0x00` | Mute | 0 / 1 |
| `0x01` | FX (reverb/echo) send level | signed 0.1 dB, −650 = −∞ |
| `0x05` | M/S processing | 0 / 1 |
| `0x06` | Phase invert | 0 / 1 |
| `0x07` | Preamp gain | 0.1 dB, 0–650 (Mic 1: 0–65 dB) |
| `0x0a` | AutoSet | 0 / 1 |
| `0x80` | Auto-level on | 0 / 1 |

Outputs (channel `0x12` = AN 1 shown):

| Offset | Setting | Value |
|--------|---------|-------|
| `0x00` | Output fader | signed 0.1 dB, −650 = −∞ … +60 |
| `0x01` | Balance | −100 … +100 |
| `0x02` | Mute | 0 / 1 |
| `0x03` | FX return level | signed 0.1 dB |
| `0x07` | Phase invert | 0 / 1 |
| `0x08` | Reference level | 0 / 1. One change writes AN 1–6 together (`0x12`–`0x17`): shared setting |
| `0x80` | Auto-level on | 0 / 1 |

Across all strips (`captures/05-audible-{all,rest}.pcap`, `maps/audible-inputs-1.md`, `maps/audible-rest.md`):

- Preamp gain (`0x07`) exists on Mic 1/2 and AN 3/4 only; Hi-Z is offset `0x09` on AN 3/4 only;
  the UCX has no pad. Crossfeed sends nothing on any output, including the phones: probably not
  in the UCX's hardware.
- Input reference level on the line inputs is offset `0x07` on channels 4–7 (the same offset is
  preamp gain on the mic inputs), two options 0 / 1, and one setting for AN 5–8 together: a change
  on either strip writes all four channels (`captures/05-reflevel-AN56.pcap`, `…-AN78.pcap`).
- Input phase and M/S write the channel register *and* flip the matching matrix coefficients
  (AN 7 → AN 1 became `0x5a71` = −0.293). So the channel register presumably acts on the recording
  path and the matrix on the monitor path. Input mute likewise writes `0x00` and zeroes its matrix entries.
- Playback strips have no channel registers: phase, mute, width, M/S and FX send are all matrix
  coefficients (FX send = outputs 30/31).
- Main output mute also sets bit 2 (`0x0004`) of global register `0x3f9e`. That's the register the
  ~1 s keep-alive rewrites (`0x3f98:ffff`, `0x3f9e:0000`), so `0x3f9e` looks like global flags
  (main mute; probably dim, talkback, etc.).

### Input fader/pan table: register (output << 8) | (0xe0 + input)

For every output channel (`0x12`–`0x23`) there are 18 entries, one per hardware input. Each entry
holds two fields written to the same address, told apart by bit 3 of the value:

- bit 3 clear: fader level in 0.1 dB, shifted left 4 (`0xf960` = −106 → −10.6 dB, `0xd760` = −65 dB = −∞)
- bit 3 set: pan −100 … +100, shifted left 4 (`0xf9c8` = −100, `0x0648` = +100, `0x0008` = 0)

TotalMix writes this table alongside the matrix whenever an input's fader, pan, mute or solo
changes (playback faders have no table). The DSP gains are in the matrix; this table is presumably
the unit's own record of the mix (standalone / class-compliant use). A Linux mixer should write
both, like TotalMix does.

### Selecting a submix over OSC

`/setSubmix <n>` with n = 0-based mono output index (0 = AN 1/2, 2 = AN 3/4, 6 = PH 7/8 …) switches
TotalMix's submix; `/1/labelSubmix` reports it back. No TotalMix option needed. Selecting output
strips (`/2/track±` in the output row, `/1/select`) does not change it.

**Loopback** is not a register: it's vendor request 21 sent for all 30 indices (`wIndex` 0–29),
`wValue` 1 for the looped-back channels, 0 for the rest. The power-on init's 30 × req 21 with
value 0 is "all loopback off". Index i is output i + 1: loopback on output AN 1/2 set indices 0
and 1 (`maps/audible-first.md`), and on Linux indices 2/3 put outputs AN 3/4 into recorded
channels 3/4.

### Mixer matrix: register `0x4000 + output × 0x40 + source`

Verified across every submix (`captures/06-matrix-all.pcap`, `maps/matrix.md`): one fader move
per input and playback strip in each of the 9 output submixes, 180 moves. All 180 wrote exactly
the predicted matrix entries (and, for inputs, the fader/pan table entries). The only other writes
were the inputs' FX-send register (offset `0x01`), so the FX send is post-fader.

- Source `0x00`–`0x11` = hardware inputs (same numbering as the DSP channels), `0x20`–`0x31` =
  playback channels 1–18.
- Output 0–17 = hardware outputs (AN 1 … ADAT 8), 30/31 = the FX (reverb/echo) send bus.
- Value: bit 15 picks the scale, bits 14–0 are a signed 15-bit number.
  Bit 15 clear: gain = n / 32768 (used below 0.5). Bit 15 set: gain = n / 4096 (0.5 … ~4).
  So `0x9000` = 1.0 (0 dB), `0x9fec` = 1.995 (+6 dB), `0xf000` = −1.0 (phase-inverted unity),
  `0x258f` = 0.293 (−10.6 dB, the existing AN 7/8 → AN 1/2 route).
- TotalMix does pan, balance, width, M/S and playback phase by writing matrix coefficients:
  mono inputs panned centre get −3 dB to each side; width 0 writes 0.5 to all four L/R cross-points.
  A stereo strip's balance is linear: AN 7/8 → AN 1/2 at R50 halved the left gain (`4006`
  `258f` → `12c7`, −6 dB) and left the right as it was, and the fader table's entry for AN 7 in
  AN 1's row went from −10.6 to −16.7 dB with its pan unchanged (`maps/audible-rest.md`). The
  table holds each input channel's level after the balance; a mute writes −∞ (`d760`) there.

### The power-on state push, decoded with this model

Of its 1,108 words: **684 are the full matrix**, 18 are known channel settings, 2 are the keep-alive
pair, and **404 are offsets `0xe0`–`0xf1` on each output channel `0x12`–`0x23`**: 18 entries per
output, one per input source (`0xe0 + input`), one or two words each: the input fader/pan table
described above (level and pan fields).

The non-zero matrix entries are the expected default routing: playback N → output N at
unity for all 18, all playback channels → PH 7/8, and inputs AN 7/8 → AN 1/2 at −10.6 dB.

The state push (session 01) partly fits: `0x22e0:d760`, `0x22e0:0008` would be channel `0x22`
(output ADAT 7), offset `0xe0`, which is outside the mapped blocks. `0x44xx` is above the 36
channels, maybe the mixer matrix. Not decoded yet; the audible probes (faders, pan) should tell.

## Mapping parameters with OSC

TotalMix's OSC remote is enabled (Options > Settings > OSC: controller 1, in 7001, out 9001,
host 127.0.0.1). `tools/osc.py` sends and listens; `tools/osc_probe.py` changes parameters while
capturing, labels each step, and restores what it touched. Things learned:

- Page 1 = mixer strips of the selected row (8 per bank: fader, pan, mute, solo, phantom, mic gain),
  page 2 = settings of the selected channel (gain, 48V, pad, Hi-Z, EQ, low cut, dynamics,
  auto-level, reverb send…), page 3 = reverb/echo, snapshots 1–8, mute/solo/fader groups.
- TotalMix sends a page's full contents only when the remote switches to it (`read_page` hops).
- Buttons are toggles: sending 1.0 presses, 0.0 is ignored. Continuous controls have a
  `<address>Val` display string; buttons don't.
- OSC can recall snapshots but not store them, so recalling is avoided (it would overwrite the mix).

## Power-on init (`captures/01-init-a.pcap`, 2026-10-23)

UCX power-cycled during the capture; it reappeared at USB address 14 at about +26.0 s. Times below
are relative to capture start. All vendor writes are `bmRequestType 0x40`, no data stage.

1. **+26.089 req 22** `wValue 0`, `wIndex` 0x00–0x1d and 0x20–0x3d: 60 writes, two banks of 30
   (bank in wIndex bit 5). The Digiface boot sends 22 per input with `wValue 0x400` ("loopback off").
2. **+26.093 req 27** ×4. A 24-bit value split as `wIndex` high byte : `wValue`, with the index in the
   `wIndex` low byte: [0] `0x00c350` (= 50000), [1] `0xd28db8`, [2] `0xd38234`, [3] `0xf87cff`.
   The UC project labels 27 "set sample rate, packet 1 of 2". Possibly clock/DDS coefficients.
3. **+26.093 req 16**, which looks like the Digiface's register write (`wValue` = bits, `wIndex` = mask):
   `0x0119/0x05ff`, `0x0000/0x3000` ×2, `0x0800/0x0800`, `0x0000/0x0800`, `0x2000/0x3000`.
4. **+26.093 req 23** `0x3f00/0x0fc0` once (UC project: 23 = +48V / pad / Hi-Z).
5. **+26.20–26.28 req 16**: `0x0000/0x3000` once, then `0x0800/0x0800` five times.
6. **+26.216** status polling starts (28, 30, 31, 17 every ~16 ms), replies identical to before.
7. **+26.548 req 21** `wValue 0`, `wIndex` 0x00–0x1d: 30 writes (Digiface: 21 `0x9000` per output = unity
   gain; UC project: 21 = loopback on output).
8. **+26.549** interface 1 register traffic starts.
9. **+28.57–28.93** 17 × 252 B + 148 B on `0x0c`, about 1,100 words: probably TotalMix pushing the full
   mixer state. Ends with `0x3f98ffff 0x3f9e0000`, which then repeats alone about once a second
   (keep-alive?).

Interrupt IN `0x8b` had 8 URBs pending that never completed in 90 s. MIDI IN `0x86` was idle.

**Run B** (`captures/01-init-b.pcap`, second power-cycle, TotalMix settings unchanged): the 107
non-poll control writes and the 1,108-word state push match run A word for word. So the
sequence is deterministic. Both runs had the same saved state, though, so this doesn't yet say
which words are fixed init and which are your settings. Cheaper than more power-cycles: change one
TotalMix setting, trigger the push again (session 11: snapshot recall / TotalMix restart), and diff
the 1,108 words. That should map each parameter to its word.

Open: what bits 0x0800 and 0x3000 of the req-16 register do. Bits 12–13 step through 0 → 2 here and
2 → 0 → 1 → 0 at stop; the Digiface puts its speed mode (2/3/4) at bit 12. Session 03 (sample rates)
should separate speed from run state.

## Level meters: bulk IN `0x85` (interface 1)

TotalMix reads `0x85` only while its window is visible (`captures/02-totalmix-visible.pcap`); no
command enables it, the host just submits reads. Packets are 244 bytes, about 240 per second, five
types in rotation, tagged by the last 4 bytes:

| Tag | Meters |
|-----|--------|
| `0x11111111` | Hardware inputs (one variant) |
| `0x55555555` | Hardware inputs (other variant; identical here, presumably pre/post EQ+dynamics) |
| `0x22222222` | Playback channels |
| `0x33333333` | Outputs before the output fader (submix sum) |
| `0x66666666` | Outputs after the output fader (what reaches the converter) |

Layout: bytes 0–159 = 20 × u64 **power** (one per channel, slots 18/19 unused or FX), bytes
160–231 = 9 × (u32, u32) **peaks** (even channel low half, odd channel high half), bytes 232–239 zero,
bytes 240–243 the tag. Calibrated with the inaudible pattern stream (`captures/02-meters-pattern.pcap`,
playback n = constant 24-bit n+1): power = 512 × s24², peak = 16 × |s24|. So

- power dBFS = 10·log10(value / 2^55); the floor value 31 means "below range"
- peak dBFS  = 20·log10(value / 2^27)

Checks: playback n read exactly 512·(n+1)²; PH 7/8 pre-fader read the power of all odd / all even
playback channels summed (the routing); AN 1/2 post-fader dropped to the floor (fader at −23 dB)
and PH 7/8 post-fader to 0 (fader −∞); input noise reads ~17 LSB RMS, matching the raw samples.

**On Linux** (`ucx-stream meters`, `captures/26-meters-linux.log`, 2026-10-24): claiming
interface 1 and reading `0x85` is all it takes, with nothing else sent to the unit and no
TotalMix state push. The unit answers every read at once with the next packet in the rotation
(about 25,000 a second from a tight loop of synchronous 512-byte reads), so the rate is the
host's; `meters` paces them like TotalMix, 240 a second. Peaks are held between reads: the first
read after a long pause returned old peaks up to 0 dBFS on inputs and a ramp on playback. Checked
against the selftest's code (playback 3/4, 128…255 LSB) streaming meanwhile from another process,
which stayed clean: playback 3/4 peaked at −90.3 dBFS (255 LSB) with RMS −92.7 (the code's
−92.6), outputs 3/4 post-fader the same (faders at 0 dB), and every other playback channel at the
floor. The input meters show the hardware inputs, not the loopback (AN 3/4 read input noise,
about −113 dBFS RMS). With nothing playing, main outs AN 1/2 post-fader read a constant 1 LSB.

The idle poll on `0x0c` (`0x3f00|N`, N = 0…15, answered on `0x8d` by `0x3f00:1a00`, `0x3f01:ffff` or
`0x3f03:0000`, `0x3f02:<varies>`, `0x3f10:7070`, `0x3f50:0000`) is something else, still unknown.

## Writing the mixer from Linux (`ucx-stream mixer-check`, `captures/27-*.log`, 2026-10-24)

The first writes from Linux, where nothing can be heard: only the submixes of the digital
outputs (SPDIF, ADAT; nothing plugged in) and ADAT 1/2's output level, every word checked
against that before it went out, the words from `ucx_mixer`'s model. The source was the
selftest's code on playback 3/4 (below −90 dBFS) and Mic 1's input noise. The gains were read
as the ratio of output to source power on the meters.

- **Writing works as TotalMix does it**: claim interface 1, send the words on bulk OUT `0x0c`
  (little-endian, parity bit, bursts of at most 63 words). Nothing else is needed first: no
  poll, no state push.
- **Read bulk IN `0x8d` while writing.** The unit sends status words there all the time, about
  80–120 a second: `3f00:1a00` most, `3f01:ffff`, `3f02:<varies>`, `3f03:0000`, `3f21:7070`,
  `3f61:0000`. That's the same family as the replies to TotalMix's idle poll, which saw
  `3f10:7070`, `3f50:0000` where Linux streaming gets `3f21`, `3f61`. Unread, they fill a queue
  of about 454 words. With it full, single writes still went in, and so did about 150 words in
  short runs. But a 720-word run stalled: bulk OUT timed out partway. With a thread reading
  `0x8d` throughout and bursts 20 ms apart (TotalMix's push: 17 bursts in 0.36 s), 778 words
  went in, the 722-word restore among them. Once, during the run after the stall, `0x8d` also
  carried single words at output channels' offsets `0xa0`/`0xa1` (`d760`, `0000`) and `0xa9`,
  `0xad`, `0xc1`, `0xe1`, `0xe5`, `0xe9`, `0xed` (all `ff9c`). Not seen again, not explained.
- **Every gain landed as written** (`27-mixer-writes-linux.log`, 29 checks):
  - Matrix gains: 0, −6.0, −6.1 and +6.0 dB within 0.05 dB. −6.0 and −6.1 dB are the two sides
    of the encoding's scale switch at 0.5.
  - −20 dB read −20.2. That's not the gain: the output is the source's −112.7 dBFS rounded to
    24 bits (about 19 LSB), which costs about 0.2 dB.
  - A stereo strip's balance at 50 put its left side at −3.04 dB. At −100 its right side was
    silent.
  - A mono input centred gave −3.09/−3.09 dB on Mic 1's noise (its power wanders). Hard left
    gave 0.00 dB and a silent right side.
  - The cross terms were silent, and the submixes were independent (ADAT 1/2's didn't reach
    SPDIF).
  - Output level (channel register `0x00`) −10.0 dB read −10.07 on ADAT 1/2's post- against
    pre-fader meter. It was at 0.0 dB before, and it was put back.
  - SPDIF's own processing (its channel settings registers): the dynamics with both ratios at
    1:1 are its gain alone, and −10 / +10 dB read −10.11 / +10.01. Off read 0.00, even with the
    gain left at +10. EQ on with every band at 0 dB read 0.00. A +12 dB band at 1 kHz read
    +0.33, and a 500 Hz 24 dB/oct low cut −18.4. Both are right for the code: it's a
    128-sample sawtooth from 128 to 255 LSB, about 96 % DC by power, so a boost near its
    3rd harmonic lifts only the rest (+0.35 dB worked out), and a low cut takes the DC. All
    put back to the settings the probes had found (`ucx_mixer::Settings::TOTALMIX`: everything
    off, the same on all 20 strips probed, `re/maps/channel-settings.md`).
  - A stereo strip's width −100 (only the digital outputs' share of its words sent) swapped its
    sides: playback 4 reached SPDIF L at +0.01 dB, playback 3 SPDIF R at −0.00. The model splits
    each side into a straight share (1 + w)/2 and a crossed one (1 − w)/2, which is TotalMix's
    0.5 on all four cross-points at width 0. Phase is the gains' sign, which power meters can't
    show.
- **The meters' power has ballistics**: after a drop it falls about 24 dB a second, linear in dB
  (0 → −20 dB takes 1.4 s; from −57 dB it reads silent). After a rise it settles to within
  0.1 dB in about 0.7 s (`27-mixer-meter-ballistics.log`). So a measurement waits 3 s after a
  change. The gains themselves seem to change at once: nothing in the readings points to a
  ramp in the DSP, though the meters would hide a short one.

## Global monitor registers (`captures/07-global.pcap`, OSC page 1)

| Register | Meaning | Values |
|----------|---------|--------|
| `0x3d01` | Global mute enable | 1 on (default), 0 off |
| `0x3d03` | Main Dim | 1 on; Dim also rewrites the Main output fader |
| `0x3f9e` | Main monitor flags | bit 1 (`0x02`) Mono, bit 2 (`0x04`) Main mute |
| `0x3f98` | Main monitor mask? | `0xffff` normally, `0x9bff` with Mono (bits 10, 13, 14 cleared) |

The "keep-alive" (`0x3f98:ffff`, `0x3f9e:0000` about once a second) is really TotalMix re-sending
the main monitor state. Speaker B, External In, Talkback, Mute FX, solo enable, Trim and Speaker B
link sent nothing in this setup (nothing assigned to them, or they're TotalMix-only).

## Sample rate, clock source and device options

Driven through RME's own interfaces: the ASIO driver for the rate (`tools/asio_ctl.py`,
`tools/rate_sweep.py`), the settings dialog's controls for clock and options (`tools/rme_settings.py`).
Captures `03-rates`, `04-settings`, `04-settings-combos`. Everything restored (44.1 kHz, Internal).

**Request 16 is a masked control-register write**: `wValue` = bits, `wIndex` = which bits to change.

| Bits | Mask | Meaning |
|------|------|---------|
| 1–2 | `0x0006` | Clock source preference: 0 Internal, 1 SPDIF coax, 2 Optical, 3 Word |
| 3 | `0x0008` | Word clock input termination |
| 4–5 | `0x0030` | Base rate family: 0 = 32 kHz, 1 = 44.1 kHz, 2 = 48 kHz |
| 6 | `0x0040` | Word clock output "single speed" |
| 7 | `0x0080` | SPDIF output professional format |
| 8 | `0x0100` | 1 = EQ+D *not* applied to the recording path (dialog's "EQ+D for Record" off) |
| 10 | `0x0400` | Optical output format: 1 = SPDIF, 0 = ADAT |
| 11 | `0x0800` | ? (set repeatedly after init and after streams restart) |
| 12–13 | `0x3000` | ? (stepped 0 → 2 at init, 2 → 0 → 1 → 0 at driver stop) |
| 15 | `0x8000` | Set at every stream start |

The settings dialog writes bits 0–10 together (mask `0x05cf`). The power-on init's
`0x0119/0x05ff` = Termination + 44.1 kHz family + bit 8 + bit 0 (bit 0 unknown). Request 23
(mask `0x0fc0`) mirrors bit 8 (`0x3c00` ↔ `0x3d00`) and carries bits 9–11 of something else.
TMS sends nothing: it's handled in the driver.

**Speed (1× / 2× / 4×) is not a register.** The driver selects interface 5 alt setting 1 / 2 / 3
(SET INTERFACE) when the rate changes speed band, and the UCX follows. A rate change inside a
band only rewrites the base-rate bits. For Linux: select alt 1/2/3 of the streaming interface
and write bits 4–5. To check: whether interface 0's iso alt settings drive the speed the same way.

**Status word (request 17), top byte = rate index**: 0 = 32k, 1 = 44.1k, 2 = 48k, 4 = 64k,
5 = 88.2k, 6 = 96k, 8 = 128k, 9 = 176.4k, 10 = 192k, i.e. (speed << 2) | base. That's the same
rate table as the mainline Digiface driver. It follows the alt setting at once, even with no
stream running. The other bytes (`0xff 0xf0 0x00`) didn't change in any test: with no external
signal connected, lock/sync bits can't move. An optical loopback cable would let us test them.

The ASIO driver exposes one clock source, "Settings": clock selection only works through the dialog.

## Streaming (Windows path, interface 5) — `captures/12-stream-test`, `12-stream-rates`

Silent 2-in/2-out ASIO streams at each rate (`tools/asio_stream.py`):

- Start: GET_CURRENT_FRAME_NUMBER, req 16 `0x8000/0x8000`, req 29 `0/0`, req 20 `0000/000c`,
  then interrupt transfers on `0x01` (out) and `0x82` (in), 73,728 bytes per URB at 1× (128 × 576).
- Stop: req 19 `0000/000c`, ABORT_PIPE `0x82`, then `0x01`.
- **Requests 19 / 20 mute / unmute playback channels by mask**: 32 bits, `wValue` the high half,
  `wIndex` the low half, playback channel n (0-based) at bit n + 2. The 2-channel stream sent
  `0000/000c` (channels 1–2); the 18-channel pattern stream sent req 20 `000f/fffc` (channels
  1–18, `12-stream-pattern`). Channels outside the mask stay silent (checked on interface 0
  below). (UC project: 19/20 = mute / unmute outputs, `ffff/c000` on the UC.)
- Interrupt IN `0x8b` delivers a 4-byte message every 256 frames during streaming
  (`00 NN 00 20`, NN counting), presumably a period tick for the driver's timing.
- Input samples are 32-bit little-endian, 24 bits left-justified (low byte zero).
- The same start sequence shows up whenever Windows' audio engine reopens the device.

**Sample layout** (`captures/12-stream-pattern.pcap`: all 18 ASIO outputs, output n carrying the
constant 24-bit value n+1, inaudible):

- Both directions are a plain interleaved stream of 18-slot frames, 4 bytes per slot (24-bit
  left-justified), no headers, trailers or counters anywhere in the URB.
- Slot order = ASIO channel order = DSP channel order: Analog 1–8, SPDIF coax L/R, ADAT 1–8
  (ADAT 1/2 doubles as optical SPDIF L/R).
- On the way out, every 576-byte microframe is full (8 frames) even at 48 kHz, where 6 per
  microframe would do. So the Windows path pushes data in bursts and the device must pace it,
  presumably by NAKing the interrupt endpoint, with the `0x8b` ticks reporting its position.
  The iso path (interface 0) is paced per microframe instead and has a different sample format:
  see the next section.
- Captured with the service's default 1 MB USBPcap buffer, which drops most full-size URBs and cuts
  each at 65,535 bytes. 11 out / 4 in URBs survived, which is plenty for the layout. For complete
  stream captures, add a `-b` buffer option to `tools/capture.py` (service restart, so UAC).

## Isochronous streaming, interface 0 (Linux, 2026-10-23)

Measured with `linux/iso_probe.py` (libusb, no kernel driver on the unit), logs in
`captures/20-iso-in-*.log` (one per rate, IN only, meters read alongside), `21-*` and `22-*`
(OUT attempts that didn't play) and `23-*` (OUT that plays, checked through the unit's
loopback). Descriptors read on Linux are byte-identical to `descriptors/`. Parsing and building
the packets: `ucx_protocol::iso`.

- **The unit streams as soon as interface 0 goes to alt 1 / 2 / 3.** No power-on writes and no
  stream-start requests (16 `0x8000`, 29, 20) are needed: IN data arrives from the third
  microframe on, with or without them. The status word follows the alt setting and the base-rate
  bits exactly as with interface 5, at all nine rates.
- One packet per microframe, also at 4× (where a microframe holds up to two 604-byte
  transactions). The first two packets after SET_INTERFACE are a bare 4-byte header with 0 frames.
  The first packets read can also be stale when IN starts a few ms after SET_INTERFACE (as in
  `ucx-stream`, which sends the rate and stream-start requests in between): a packet with frames
  at position 0, or an overflow (`-EOVERFLOW`), and then the position jumps ahead by 1,200–1,900
  frames to where the unit's counter really is (it counts from the alt setting on). From there it
  runs on. So a driver should take the frame rate from after the last jump.
- **IN packet = 4-byte header + N frames of planar float32**: N samples of channel 1, then N of
  channel 2, …, 18 / 14 / 12 channels at 1× / 2× / 4×, same channel order as interface 5 (analog
  1–8, SPDIF, ADAT). N is 4 at 32 kHz, 5/6 at 44.1, 6 at 48, 8 at 64, 11/12 at 88.2, 11–13 at 96,
  16 at 128, 22/23 at 176.4, 23–25 at 192; the frames per second match the rate.
- **Samples are IEEE float32, little-endian, 1.0 = full scale** (24 significant bits), unlike
  interface 5's 24-bit integers. Input noise reads ~2.6e-6 (≈ 20 LSB of 24 bit); SPDIF and ADAT
  without a signal are exactly 0.0. Read planar, each channel has its own noise level and they
  match the DSP's input meters (bulk `0x85`, tag `0x11111111`) channel for channel: stream
  18 19 17 17 38 34 17 17 LSB rms, meters 18 19 17 17 37 34 16 17 (`20-iso-in-44k1.log`).
  Read interleaved, they blur together.

Header, little-endian u32:

| Bits | Meaning |
|------|---------|
| 0–4 | N, the number of frames in this packet |
| 5–16 | Position of the packet's first frame: frames counted modulo 3072 (wraps at 3 × 1024) |
| 17–20 | Always 0 |
| 21–31 | Timestamp, 60 MHz ticks modulo 2048 |

The timestamp fits `(c + 7500·k − (P + N) · 60 MHz / rate) mod 2048` for every packet at every
rate within ±2.3 ticks, with k the microframe number and P + N the position just after this
packet's frames. 7500 ticks = 125 µs, so it is the sample clock's phase against the USB frame
clock in 60 MHz units (presumably the ULPI PHY clock). Its drift measures the sample clock against
USB time: −1.9 ppm for the 32 kHz family, +8.0 ppm for 44.1, +13.4 ppm for 48, the same at 1×, 2×
and 4× (one oscillator per base rate). A driver can stream without it (the frame counts give
implicit feedback), but it's a precise rate measurement for free.

After a change of speed band the analog inputs show a transient that starts near −52 dBFS and
decays by ~5 dB per 50 ms to the noise floor by about 600 ms (also on the DSP meters, so it's the
converters settling, not the stream).

### OUT (playback)

**OUT packet = N frames interleaved, each frame = one u32 lead word + one float32 per channel**
(19 / 15 / 13 words per frame at 1× / 2× / 4×; the descriptor maxima 532 / 780 / 1304 are 7 / 13 /
25 such frames, the last plus 4 spare bytes). That's the layout the Fireface UC project shows for
its macOS driver (76-byte blocks of `????` + 18 floats). Same float scaling and channel order as IN:
a float32 pattern of (n + 1) / 2²³ on playback channel n reads exactly n + 1 LSB rms and peak on
all 18 playback meters (`23-iso-out-float-44k1.log`).

- **Playback needs request 20 with the channel mask** (`000f/fffc` for 18 channels; the probe
  first sent the Windows 2-channel `0000/000c` and only playback 1–2 woke up, `22-*-start.log`).
  Request 16 `0x8000` and 29 were sent too; whether they're needed wasn't separated. Without
  request 20 nothing reaches the playback meters whatever the layout (`21-*`, `22-*`).
- **The first frame's lead word is the frame time the packet plays at.** The unit plays out of a
  **128-frame ring at its own frame clock** (the clock the IN header's position counts): a packet
  whose first lead word is p lands at ring slot p mod 128 and plays at the next frame time ≡ p
  (mod 128). Only the low 7 bits matter (offsets of 0, 128, 256, 384 behave the same), and only
  the first frame's word: zeros in the other frames play the same (`…-first-word-only.log`).
  - Checked through loopback (below) with a ramp pattern, each sample = channel × 128 + frame
    position mod 128: with lead word = position, every recorded sample of the looped-back channels
    follows its predecessor, at 1×, 2× and 4× (`23-iso-out-ramp-44k1.log`, `-96k`, `-192k`).
  - Lead word 0 everywhere: each packet overwrites the same ring slots, so the recording shows one
    packet's frames every 128 frames and silence in between (`…-lead-word-0.log`; the burst period
    is 128 at 44.1, 96 and 192 kHz). Position << 5 (the IN header's encoding) scatters the frames
    (`…-lead-word-pos5.log`).
  - So each packet has to reach the unit less than one ring (128 frames: 2.9 ms at 44.1 kHz,
    0.67 ms at 192 kHz) before its frame time, and a little before it: measured against the unit's
    own clock under "OUT timing" below. The probe's own numbers (at 44.1 kHz, 32–127 frames ahead
    of its frame count played cleanly) are off by its unknown start offset. Some of its 2× and 4×
    runs glitched (`23-iso-out-ramp-96k-glitches.log`), but that run also lost IN packets (3
    overflows), so the probe was falling behind as well.
- Round trip through the loopback: a frame recorded 4–5 frames after its frame time, so the DSP
  path itself adds next to nothing (5 frames at 32 / 44.1 / 48 and 128 / 176.4 / 192 kHz, 4 at
  64 / 88.2 / 96 kHz: `25-ucx-stream-selftest.log`).
- **Positions the unit gets no frames for play silence.** With every 100th OUT packet left empty
  (`ucx-stream selftest --gap-every 100`), all 1,095 / 2,628 / 5,256 frames left out at 44.1 /
  96 / 192 kHz came back silent, and every frame around them on time (`25-ucx-stream-gaps.log`).
  So the unit clears a ring slot once it has played it: a late or missing OUT packet is a
  dropout of its frames, never a replay of what was in the ring 128 frames earlier.

### OUT timing (`linux/out_timing.py`, `captures/24-out-timing-*.log`, 2026-10-24)

How far ahead of the unit's clock a packet's lead word has to be, with the unit's position taken
in the microframe the packet goes out in.

- **Which microframe a URB went out in.** usbfs doesn't say (`processcompl()` in
  `drivers/usb/core/devio.c` copies back status and lengths, not `start_frame`), so
  `out_timing.py` streams through usbfs directly and notes when each URB is reaped. A URB comes
  back shortly after its last microframe, never before, so the earliest reaps against each
  stream's own packet count line the two streams up: "OUT packet k goes out with IN packet
  k + s". 90% of reaps come within 26 µs of the earliest. s comes out 0.03–0.35 above an integer
  and stays there over the run (taken again per 100 ms). The fraction is larger at higher rates,
  presumably because bigger OUT packets complete later in the microframe.
- Submitted without `ISO_ASAP`, both streams start on a USB frame boundary
  (`xhci_get_isoc_start_frame` in `drivers/usb/host/xhci-ring.c`): s was 400 or 408 microframes
  in all 9 such runs, multiples of 8. With `ISO_ASAP`, as libusb submits, it was 402 in all 3
  runs. The reap-time offset gives the same window either way.
- **Reference: A, the IN header position just after the frames of the IN packet that goes out
  in the same microframe** as the OUT packet. The sweep sets the first lead word L = A + D for D
  across the ring, 25 ms per step, with packet sizes following so the frames run on without
  gaps. Playback 3/4 carry each frame's own position (two 7-bit digits, below −90 dBFS), so the
  loopback recording shows exactly which frame played when.

| Rate | Frames per packet | Whole packets on time for L − A (3 runs each) | Lower edge as time |
|------|------------------|-----------------------------------------------|--------------------|
| 44.1 kHz | 5–6 | 2 to 120–122 | 45 µs |
| 96 kHz | 11–13 | 2–4 to 116–118 | 21–42 µs |
| 192 kHz | 23–25 | 6–8 to 110–112 (also with `ISO_ASAP`: 7–8 to 110–112) | 31–42 µs |

- On time means every frame was recorded exactly the loopback delay after its own position
  (5 frames at 44.1 and 192 kHz, 4 at 96 kHz), for every packet with that L − A.
- With the first frame less than about 2 frames (1×) to 8 frames (4×) ahead, i.e. under about
  40 µs, the packet plays a ring late (128 frames after its position). With the last frame more
  than a ring ahead, it plays a ring early. Around both edges some frames are never recorded at
  all, and which outcome a frame gets depends on its packet's lead word, not only on its own
  position.
- **So the unit takes a packet until about 40 µs before its first frame plays**, and the
  usable range of L − A is about 2–8 to 128 minus the packet's frames. L − A = 60 is mid-range
  at every rate (`ucx_protocol::iso::OUT_LEAD`): ±50 frames of slack is ±1.1 ms at 44.1 kHz
  but only ±2 microframes at 192 kHz. So a driver has to know which microframe each OUT packet
  goes out in, to within a microframe or so. The kernel has it (`urb->start_frame`, exact
  without `URB_ISO_ASAP`); userspace can take it from reap times as above.

### Streaming both ways from userspace (`crates/ucx-stream`, `captures/25-*.log`, 2026-10-24)

`ucx-stream` streams IN and OUT through usbfs with the OUT lead words locked to the unit's clock
as above (offset from reap times, lead word = predicted A + `OUT_LEAD`, packet sizes following
the unit's frame counts). Its `selftest` plays the position code of `out_timing.py` on playback
3/4 and checks every frame that comes back through the loopback.

- **Clean at all nine rates**: every frame of the code on time, no OUT packet errors, no lead
  word put back on target (5 s per rate, `25-ucx-stream-selftest.log`; the one IN error in some
  runs is the overflow right after SET_INTERFACE). 60 s at 192 kHz: 11,469,939 frames on time
  (`25-ucx-stream-192k-60s.log`); 300 s at 48 kHz: 14,387,497 (`25-ucx-stream-48k-300s.log`).
  Handling a wakeup took at most 50–200 µs, at SCHED_FIFO.
- Except the logged 32 kHz run: in one second of it 64 IN packets (one whole queue, 8 ms)
  never came and the position jumped twice, while the streaming thread was never more than
  0.2 ms late, so the host held the stream up for about 8 ms. The engine re-anchored, put the
  lead words back on target once, and 317 frames came back wrong (61 silent, 256 a ring late)
  before everything ran on time again. An earlier 32 kHz run, unlogged, had no such stall.
- **URBs in flight have to cover about 4 ms per direction** (on the Intel xHCI it was tested on). With 16
  microframes or fewer queued (2 × 8, 4 × 4, 4 × 2 packets × URBs: 4 of 4 runs), IN loses about
  one packet in six (`-EOVERFLOW`, and URBs that complete late, so the position jumps) and OUT
  goes off target. With 24 (3 × 8, 2 × 12) 8 of 9 runs were clean and one lost its queue (the
  offset moved and streaming started over); with 32 or more (2 × 16, 4 × 8, 1 × 32, 2 × 24, 8 × 8)
  all 12 were (`25-ucx-stream-queue-depth.log`). Presumably the xHCI driver's scheduling
  threshold: a URB queued less than about two frames ahead gets no explicit frame number
  (`xhci_get_isoc_frame_id`) and is scheduled "as soon as possible".

**Loopback** (request 21, index i = output i + 1, see "Selecting a submix over OSC") copies an output into
the recording of the same channel number, sample for sample, which makes it a way to see
exactly what the unit plays. The probe turns it on for outputs 3/4 and back off for all 30
indices (the power-on state) afterwards.

**Meter readout, seen while reading continuously**: with four bulk reads always pending, `0x85`
answers about 8,700 packets per second per tag (TotalMix reads about 48). The power values are
smoothed (they take a while to settle after a level change), and a packet's peak is the peak since
the last packet, so most packets carry peak 0 and a few the full value. The first reads also
return packets queued before them, from earlier streams.

## Driver stop (`captures/01-driver-stop.pcap`)

Five req-16 writes, nothing else: `0x0000/0x0800`, `0x2000/0x3000`, `0x0000/0x3000`, `0x1000/0x3000`,
`0x0000/0x3000`.

Capture note: `pnputil /restart-device` re-enumerates the UCX at a new address, and USBPcap then records
nothing from it, even with `-A`. A power-cycle with `capture.py --follow` works.

## Prior art

- **RME Digiface USB** (`2a39:3f8c` / `3fa0`) is in mainline `snd-usb-audio` (patches from Sep 2024):
  `quirks-table.h` `QUIRK_RME_DIGIFACE` claims only interface 0, iso, S32_LE / 24-bit, implicit
  feedback from the IN endpoint, three rate bands. `quirks.c` has a boot quirk and a rate quirk;
  `mixer_quirks.c` reads status with vendor request 17. The matrix mixer is left to userspace (libusb).
  - Boot: req 16 `wValue 0x2410 wIndex 0x7fff` (mixer off, internal clock, 48k), req 18,
    req 22 per input (loopback off), req 21 `0x9000` per output (unity gain).
  - Rate: req 16, `wValue = (idx << 3) | (speed << 12)`, `wIndex 0x7078` (mask?).
- **Fireface UC** reverse engineering: <https://github.com/agfline/RME-Fireface-UC-Drivers>
  (no working driver; documents vendor requests 0x10–0x20 from the macOS kext, init sequence).
- **FireWire**: `snd-fireface` (ff-protocol-latter.c) already supports the UCX over FireWire.
