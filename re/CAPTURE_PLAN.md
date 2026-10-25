# USB capture plan (Windows, USBPcap + Wireshark)

## Overnight 2026-10-23 (after 17:45)

Done without you: sample rates (03), clock source and device options (04), stream start/stop and
sample layout (12), global monitor flags (07), level meters (TotalMix window shown briefly), and the
protocol library `crates/ucx-protocol`. Everything was restored: 44.1 kHz, clock Internal, options as
they were, submix AN 1/2, TotalMix back in the tray. The capture service was stopped at about 18:15.

Still open, all quick once the service is running again: reverb/echo parameters (OSC page 3,
inaudible while reverb is off), the second half of session 02 (TotalMix closed), and complete
stream captures (`capture.py` needs a USBPcap `-b` buffer option first). Then the Linux side,
`linux/DRIVER_PLAN.md`.

## Where we stopped (2026-10-23, 17:45)

Done: descriptors, power-on init (01, runs A/B identical), driver stop, idle with TotalMix open
(02, first half), every channel setting on all 29 strips, inaudible (08) and audible (05), the
full mixer matrix across all 9 submixes (06), and the fader/pan table and state push decode.
Every probed setting was restored and verified; TotalMix's selection is on Mic 1, submix AN 1/2.

To resume, with the UCX connected and TotalMix running: `python tools/capture.py --start-service`
(one UAC prompt on the PC). Still open:

- Sessions 03 (rates), 04 (clock and options) and the start/stop part of 12 (streaming) were done
  the evening of 2026-10-23 through ASIO (`tools/asio_ctl.py`, `rate_sweep.py`, `asio_stream.py`)
  and the settings dialog (`tools/rme_settings.py`). Still open there: external clock lock/sync
  bits (needs a cable, e.g. optical out → optical in) and request-16 bits 0, 11, 12–13.
- Global registers `0x3f98` / `0x3f9e` (main mute, dim, talkback, …) and the meter polling (02).
- Session 02 second half: idle with TotalMix closed (needs TotalMix quit by hand).

TotalMix keeps the OSC settings (in 7001, out 9001, 127.0.0.1) across restarts.

Goal: map every TotalMix / Settings-dialog action to the USB traffic it causes. We only observe
traffic between our own driver and our own unit; we don't copy anything out of RME's binaries.

## Setup

- Installed 2026-10-22: Wireshark 4.6.9 and USBPcap 1.5.4 (winget). The USBPcap installer didn't
  register its extcap with Wireshark, so `USBPcapCMD.exe` was copied to
  `%APPDATA%\Wireshark\extcap`. The USBPcap filter only attaches to USB hubs after a reboot;
  `USBPcapCMD.exe --extcap-interfaces` lists nothing until then.
- The UCX is on USBPcap2, device address 4 (2026-10-23). That root hub also carries keyboard and
  mouse, so never capture the whole hub: it would record keystrokes.
- At the start of a sitting, run `python tools/capture.py --start-service` and approve its one UAC
  prompt (USBPcap refuses non-admin users; prompts only show on the PC's own screen). For the next
  6 hours, captures need no prompt.
- Capture with `python tools/capture.py re/captures/NN-topic.pcap` (Ctrl+C to stop, or
  `--seconds N`). It finds the UCX's hub and address itself and filters to that address.
- TotalMix ignores a normal close request (it hides to the tray), so scripts can't quit it.
- Mixer parameters are driven over TotalMix's OSC remote instead of by hand:
  `python tools/osc_probe.py re/captures/NN-topic.pcap --page /2 --set "label" /2/eqEnable 1 ...`
  (see NOTES.md, "Mapping parameters with OSC"). Run it from PowerShell: Git Bash rewrites
  `/2/...` into a Windows path unless `MSYS_NO_PATHCONV=1` is set.
- `python tools/map_channel_settings.py re/captures/08-....pcap` maps every channel's inaudible
  settings (sections that are off) over OSC and restores them; `python tools/map_report.py` turns
  the capture plus its `.steps.jsonl` into a table in `re/maps/`.
- Never automate phantom power (48V can damage ribbon mics and thump speakers). Faders, gains and
  mutes are audible, so probe those only when nothing loud is connected or playing.
- Summarize with `python tools/pcap_summary.py re/captures/NN-topic.pcap --controls --marks ...`.
- One `.pcap` per session in `re/captures/`, named `NN-topic.pcap`.
- During a session, run `python tools/mark.py re/captures/NN-topic.marks` in a second terminal and type a short label
  before each action (e.g. `in1 gain 0->10`). Each label gets a wall-clock timestamp we line up
  with the capture.
- Step values with the keyboard or by typing numbers, not by dragging, so the values are known.
- In Wireshark, the useful view filter for control traffic is
  `usb.transfer_type == 2 && usb.setup.bmRequestType != 0x80` (vendor control, minus descriptor reads).

## Sessions

| # | Topic | Actions |
|---|-------|---------|
| 01 | power-on init | `capture.py ... --seconds 90 --follow`, then switch the UCX off and on within the first minute. `--follow` keeps tracking the UCX at its new address. (`--restart-device` doesn't work: USBPcap loses the device after a driver restart.) Runs A and B done (identical). |
| 02 | idle | 30 s with TotalMix closed, then 30 s with TotalMix open (shows periodic status/meter polling). |
| 03 | sample rate | In Fireface USB Settings: 44.1 → 48 → 32 → 64 → 88.2 → 96 → 128 → 176.4 → 192 → 48, 5 s apart. |
| 04 | clock | Each clock source (Internal, SPDIF coax, ADAT/optical), with and without a signal present if possible. |
| 05 | inputs | Per input: gain steps min → max on mic 1, 48V on/off on 1 and 2, Inst/Hi-Z, reference level on line inputs, stereo link, phase invert. |
| 06 | matrix | One cross-point (In 1 → Out 1/2): fader −∞, −20, −6, 0, +6 dB; pan L/C/R; mute; solo. Repeat for In 2 → Out 1/2 and In 1 → Out 3/4 to learn the addressing. Then the same for a Playback channel. |
| 07 | outputs | Output fader steps, mute, reference level, phones outputs, stereo/mono. |
| 08 | DSP | Input/output EQ on/off + one band's gain/freq/Q, low cut, dynamics on/off + one parameter, reverb/echo on/off + type + send level. |
| 09 | digital I/O | Optical mode ADAT ↔ SPDIF, SPDIF format consumer/professional, any emphasis/non-audio flags. |
| 10 | meters | Known level into In 1 (or loopback a test tone), watch meter traffic. Also pre/post-fader meter setting. |
| 11 | snapshots | Store a snapshot, change things, recall it. Load a TotalMix workspace. Also try closing and reopening TotalMix. Whatever re-sends the 1,108-word state push from session 01 becomes the way to map parameters: change one setting, push again, diff. Worth doing early. |
| 12 | streaming | Play a known test signal via WASAPI and via ASIO at 48 / 96 / 192 kHz (Windows streams on interface 5, which tells us the sample layout but not the iso path). |

## After each session

Add the findings to `NOTES.md` (request number, wValue/wIndex encoding, value scaling) and keep
the raw capture. Anything the captures can't explain goes on a list for a targeted follow-up capture.
