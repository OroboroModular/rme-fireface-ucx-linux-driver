# Fireface UCX on Linux, without class-compliant mode

Run an RME Fireface UCX on Linux over USB in its normal (vendor) mode, with a TotalMix-style
mixer app. RME doesn't support Linux; the protocol is reverse engineered from
USB captures of the unit talking to RME's Windows driver and TotalMix FX, plus RME's own public
interfaces (ASIO, TotalMix's OSC remote, the settings dialog). No RME code is decompiled or copied.

## Status (2026-10-25)

- **Kernel driver: working.** [`linux/kernel`](linux/kernel) makes the unit an ALSA card,
  `snd-fireface-ucx`: 18 / 14 / 12 channels each way at all nine rates (32 to 192 kHz), checked
  frame by frame through the unit's loopback, and played through PipeWire with no errors, gaps or
  resyncs. The unit's two MIDI I/Os are on the same card ("Fireface UCX Port 1" and "Port 2";
  output checked, input not yet). Builds out of tree or with DKMS, against kernel 7.0.
- **Userspace driver: working.** [`crates/ucx-stream`](crates/ucx-stream) does the same through
  usbfs, without root, as the PipeWire sink and source "Fireface UCX". One of the two drivers runs
  at a time.
- **Mixer app: working.** [`crates/ucx-mixer-app`](crates/ucx-mixer-app), `ucx-mixer`:
  TotalMix's three rows of strips with the unit's level meters live; click an output's name to see
  and set its submix, and the icon beside a strip's name for its EQ, low cut, dynamics,
  auto-level, preamp, phase and width; eight snapshots. The model
  ([`crates/ucx-mixer`](crates/ucx-mixer)) starts from the mix TotalMix left in the unit, makes the
  same register words TotalMix does, and is kept between runs. It runs next to either driver.
  Nothing is sent until **Send to unit** is on, and then only to the SPDIF and ADAT outputs;
  **Analog outputs** asks first (turn monitors and phones down). Output levels and preamp gains
  show "?" until set: the unit keeps them and they can't be read back. Phantom power isn't in the
  app.
- **Protocol: mixer and settings fully mapped.** Every TotalMix channel setting on all 29 strips,
  the full mixer matrix, the unit's own fader/pan table, global monitor flags, sample rate, clock
  source, device options, level meters, the power-on sequence and stream start/stop. See
  [`re/NOTES.md`](re/NOTES.md). The protocol library, [`crates/ucx-protocol`](crates/ucx-protocol)
  (Rust, no dependencies), builds the register words and parses meters and the iso audio
  packets; its tests replay real captures.
- **Not yet:** the reverb and echo, the main output's Dim and Mono, the input and output
  reference levels, and external clock lock. Only the Fireface UCX (USB `2a39:3fc9`) is
  supported, not the UCX II. Tested on one unit (firmware 03013100) with an Intel xHCI
  controller, kernel 7.0 and PipeWire 1.6.

## Install

Rust (`cargo`) and, for the kernel driver, the kernel's headers and DKMS. Turn monitors and
headphones down before the first sound: the card plays at full scale through the unit's mixer,
as TotalMix last left it.

```
cargo build --release

# let the logged-in user open the unit (the mixer app, ucx-stream)
sudo cp linux/70-fireface-ucx.rules /etc/udev/rules.d/
sudo udevadm control --reload && sudo udevadm trigger --attr-match=idVendor=2a39

# the kernel driver, rebuilt for every kernel and loaded whenever the unit is plugged in
make -C linux/kernel clean
sudo dkms add linux/kernel
sudo dkms install snd-fireface-ucx/0.1
sudo modprobe snd-fireface-ucx

# PipeWire: the card's nodes never become the default device by themselves
mkdir -p ~/.config/wireplumber/wireplumber.conf.d
cp linux/wireplumber/51-fireface-ucx.conf ~/.config/wireplumber/wireplumber.conf.d/
systemctl --user restart wireplumber

# the mixer app (UCX Mixer in the application menu) and ucx-stream, for this user
linux/install-user.sh
```

Then pick **Fireface UCX** in the sound settings. Updating the kernel driver, and the userspace
driver instead of it (`ucx-stream pipewire`): [`linux/DRIVER_PLAN.md`](linux/DRIVER_PLAN.md).

## Layout

| Path | What |
|------|------|
| `re/NOTES.md` | Everything known about the protocol, with the capture each fact comes from |
| `re/CAPTURE_PLAN.md` | How captures are made, the session list, and where we stopped |
| `re/captures/` | The USB captures (313 MB, Git LFS), their labels and logs, and the test logs the notes cite. Kept private: not in the public repository |
| `re/maps/` | Generated tables: every probed change → the register words it produced |
| `re/descriptors/` | The unit's USB descriptors |
| `tools/` | Capture and probing tools (Windows, Python, no packages needed beyond Wireshark/USBPcap) |
| `crates/ucx-protocol/` | The protocol library |
| `crates/ucx-usb/` | The unit through Linux usbfs: control, bulk and iso transfers (shared by the driver and the mixer) |
| `crates/ucx-mixer/` | The mixer as TotalMix models it: strips, submixes, and the register words that set them |
| `crates/ucx-mixer-app/` | The mixer app `ucx-mixer` (egui, the Oroboro look); its tests draw it into `target/pictures/` |
| `vendor/oroboro_egui/` | The Oroboro look, a copy of the family's crate (GPL-2.0-or-later), not edited here |
| `linux/DRIVER_PLAN.md` | What the kernel and userspace parts need, and what's still open |
| `linux/iso_probe.py` | Streams interface 0 with libusb and reports the packet format (Linux, Python, no packages) |
| `linux/usbfs.py`, `out_timing.py` | Streams through usbfs directly, timing each transfer, and measures the OUT timing window |
| `crates/ucx-stream/` | The userspace driver: streams both ways through usbfs (`selftest` checks it through the loopback), `pipewire` puts the unit into PipeWire, `meters` shows the level meters (Linux, Rust) |
| `linux/pipewire_check.py` | Checks `ucx-stream pipewire` end to end, inaudibly: a code through PipeWire, the unit and back |
| `linux/kernel/` | The kernel driver `snd-fireface-ucx`: the unit as an ALSA card (C, builds out of tree or with DKMS) |
| `linux/alsa_check.py` | Checks the kernel driver end to end, inaudibly: a code through ALSA, the unit and back |
| `linux/wireplumber/` | WirePlumber rules for the kernel driver's card: never the default by itself, or left out of PipeWire |
| `linux/70-fireface-ucx.rules` | udev rule that lets the logged-in user open the unit |
| `linux/install-user.sh` | Installs `ucx-mixer` and `ucx-stream` for this user (no root), UCX Mixer in the application menu (`ucx-mixer.desktop`), and `ucx-stream.service` to start the userspace driver with the session |

## Tools, briefly

- `tools/capture.py`: USBPcap capture of the UCX only. `--start-service` once per sitting (one UAC
  prompt), then captures need no prompt for 6 hours. `--follow` survives a power-cycle.
- `tools/osc.py`, `osc_probe.py`, `map_channel_settings.py`, `map_matrix.py`: drive TotalMix over
  its OSC remote (enabled: in 7001, out 9001, 127.0.0.1), change settings while capturing, restore
  and verify them. Phantom power is refused in code.
- `tools/asio_ctl.py`, `rate_sweep.py`, `asio_stream.py`: sample rate and (silent or inaudible
  pattern) streaming through RME's ASIO driver.
- `tools/rme_settings.py`: the Fireface USB Settings dialog's clock and option controls.
- `tools/pcap_summary.py`, `map_report.py`: turn captures into readable timelines and tables.
- `tools/export_fixtures.py`: refresh the protocol library's test data from the captures.

## License

GPL-2.0-or-later ([`LICENSE`](LICENSE)) for everything here: the crates, the kernel driver (which
has to be GPL), the scripts and tools, the notes, and the Oroboro look in `vendor/oroboro_egui` (a
copy from its own repository, under the same license). Except:

- The look's fonts, which have their own licenses beside them (`vendor/oroboro_egui/assets/*/LICENSE`:
  SIL Open Font License, Apache 2.0).
- Names: the license covers code, not the Oroboro name or brand. RME, Fireface and TotalMix are
  RME's trademarks, named here to say what this works with; the project isn't affiliated with or
  endorsed by RME.
