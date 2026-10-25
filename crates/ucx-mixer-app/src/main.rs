//! `ucx-mixer`: a mixer for the Fireface UCX on Linux, in the Oroboro look (`vendor/oroboro_egui`).
//! TotalMix's strips and submixes (`ucx_mixer`), the unit's level meters live, and changes sent
//! to the unit as far as the top bar allows (`view.rs`, `unit.rs`).
//!
//!     ucx-mixer                         the window
//!     ucx-mixer --picture FILE.png [--after SECONDS]
//!                                       no window: the meters read for a while (3 s), the window
//!                                       drawn into a picture and the levels printed
//!     ucx-mixer --check-send            no window: the send path tried on the digital outputs
//!                                       (needs `ucx-stream selftest --seconds 60` meanwhile)
//!
//! The mix, the submix shown and the theme are kept between runs (`settings.rs`).

mod processing;
mod settings;
mod snapshots;
mod unit;
mod view;
mod widgets;

use std::process::ExitCode;
use std::thread;
use std::time::{Duration, Instant};

use oroboro_egui::egui;
use oroboro_egui::snapshot::Snapshot;
use oroboro_egui::theme::{self, Theme};
use ucx_mixer::{Mixer, Send, Settings, Source};

use settings::Saved;
use unit::{Levels, Reach, Status, Unit};
use view::MixerUi;

/// Saves this long after the last change.
const SAVE_AFTER: Duration = Duration::from_secs(1);

/// The window's state as saved last time, or TotalMix's mix; and the theme to wear.
fn start() -> (MixerUi, Theme) {
    match settings::load() {
        Some(saved) => {
            let origin = format!("as this window left it ({})", settings::path().unwrap().display());
            let theme = Theme::built_in_named(&saved.theme).unwrap_or_else(theme::chosen);
            // (what the unit keeps itself may have changed since: unknown again)
            let mix = saved.mix.clamped().without_unit_levels();
            let mut view = MixerUi::with(mix, saved.submix, origin);
            for (slot, snapshot) in view.snapshots.iter_mut().zip(saved.snapshots) {
                *slot = snapshot;
            }
            (view, theme)
        }
        None => (MixerUi::new(), theme::chosen()),
    }
}

fn saved(view: &MixerUi) -> Saved {
    let snapshots = view.snapshots.clone();
    Saved { mix: view.mixer.clone(), submix: view.submix, theme: theme::current().name, snapshots }
}

struct App {
    view: MixerUi,
    unit: Unit,
    last: Instant,
    saved: Saved,
    changed: Option<Instant>,
}

impl App {
    fn new(ctx: &egui::Context, view: MixerUi) -> App {
        let ctx = ctx.clone();
        let saved = saved(&view);
        App { view, unit: Unit::start(move || ctx.request_repaint()), last: Instant::now(), saved, changed: None }
    }

    fn save(&mut self) {
        let now = saved(&self.view);
        if now != self.saved {
            if let Err(e) = settings::save(&now) {
                eprintln!("ucx-mixer: saving the mix: {e}");
            }
            self.saved = now;
        }
        self.changed = None;
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if theme::sync() {
            theme::install_visuals(ui.ctx());
        }
        let now = Instant::now();
        let dt = (now - self.last).as_secs_f32().min(0.5);
        self.last = now;
        self.view.status = self.unit.status();
        let levels = self.unit.take_levels();
        self.view.meters.update(levels.as_ref(), dt);
        view::draw(ui, &mut self.view);

        // without the unit, changes are held back (nothing goes out by itself when it's back),
        // and the analog outputs have to be asked for again
        let live = matches!(self.view.status, Status::Live { .. });
        if !live && self.view.reach == Reach::All {
            self.view.reach = Reach::Digital;
        }
        self.unit.set_reach(self.view.reach);
        let words = std::mem::take(&mut self.view.outgoing);
        if live {
            self.unit.send(&words);
        } else {
            self.view.hold(words);
        }
        self.view.lost = self.unit.counts().1;
        // saved a second after the first change not saved yet
        if saved(&self.view) != self.saved {
            let since = *self.changed.get_or_insert(now);
            if now - since >= SAVE_AFTER {
                self.save();
            } else {
                ui.ctx().request_repaint_after(SAVE_AFTER);
            }
        }
        if matches!(self.view.status, Status::Live { .. }) {
            ui.ctx().request_repaint_after(Duration::from_millis(33));
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.save();
    }
}

fn window() -> ExitCode {
    let (view, theme) = start();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("UCX Mixer")
            .with_inner_size(view::SIZE)
            .with_min_inner_size(view::SIZE),
        ..Default::default()
    };
    let result = eframe::run_native(
        "ucx-mixer",
        options,
        Box::new(move |cc| {
            theme::apply(&theme);
            theme::sync();
            theme::install(&cc.egui_ctx);
            Ok(Box::new(App::new(&cc.egui_ctx, view)))
        }),
    );
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ucx-mixer: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Waits up to five seconds for the link to be live.
fn live(unit: &Unit) -> bool {
    let end = Instant::now() + Duration::from_secs(5);
    while Instant::now() < end {
        if matches!(unit.status(), Status::Live { .. }) {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

/// The loudest of each meter over `seconds`, and the levels taken every 50 ms meanwhile.
fn gather(unit: &Unit, seconds: f64) -> (Levels, Vec<Levels>) {
    let mut most = Levels::SILENT;
    let mut takes = Vec::new();
    let end = Instant::now() + Duration::from_secs_f64(seconds);
    while Instant::now() < end {
        thread::sleep(Duration::from_millis(50));
        if let Some(l) = unit.take_levels() {
            for r in 0..unit::ROWS {
                for c in 0..ucx_protocol::regs::CHANNELS {
                    most.peak[r][c] = most.peak[r][c].max(l.peak[r][c]);
                    most.rms[r][c] = most.rms[r][c].max(l.rms[r][c]);
                }
            }
            takes.push(l);
        }
    }
    (most, takes)
}

fn picture(file: &str, after: f64) -> ExitCode {
    let unit = Unit::start(|| {});
    let (mut view, theme) = start();
    if !live(&unit) {
        eprintln!("ucx-mixer: the unit isn't there: {:?}", unit.status());
    }
    view.status = unit.status();
    let (most, takes) = gather(&unit, after);
    for l in &takes {
        view.meters.update(Some(l), 0.05);
    }
    for (r, row) in ["inputs", "playback", "outputs"].iter().enumerate() {
        let levels: Vec<String> = (0..ucx_protocol::regs::CHANNELS)
            .filter(|&c| most.rms[r][c].is_finite())
            .map(|c| format!("{} {:.1}/{:.1}", c + 1, most.rms[r][c], most.peak[r][c]))
            .collect();
        eprintln!("{row} (channel rms/peak dBFS): {}", if levels.is_empty() { "-".into() } else { levels.join(", ") });
    }
    oroboro_egui::touch::set_here(Some(false));
    theme::apply_here(&theme);
    let mut snap = Snapshot::new(view::SIZE, 1.0);
    theme::install(&snap.ctx);
    let image = snap.settle(|ui| view::draw(ui, &mut view));
    match std::fs::write(file, image.to_png()) {
        Ok(()) => {
            eprintln!("ucx-mixer: drawn into {file}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ucx-mixer: {file}: {e}");
            ExitCode::FAILURE
        }
    }
}

/// The send path, as the window takes it, on the digital outputs only: playback 3/4 into the
/// SPDIF submix at −6 dB, then at 0 dB through SPDIF's dynamics at −10 dB, then off, measured on
/// the meters (the selftest's code on playback 3/4 meanwhile); an analog submix's change held
/// back; then the digital outputs' submixes and processing as TotalMix left them.
fn check_send() -> ExitCode {
    const SPDIF: usize = 4;
    const CODE: Source = Source::Playback(1);
    let (pb3, spdif_l) = (2, 8);
    let unit = Unit::start(|| {});
    if !live(&unit) {
        eprintln!("ucx-mixer: the unit isn't there: {:?}", unit.status());
        return ExitCode::FAILURE;
    }
    let mut view = MixerUi::new();
    view.reach = Reach::Digital;
    unit.set_reach(Reach::Digital);
    let flush = |view: &mut MixerUi| {
        unit.send(&view.outgoing);
        view.outgoing.clear();
    };
    // the output against the source after the meters have settled (they fall 24 dB a second)
    let measure = || {
        thread::sleep(Duration::from_millis(3500));
        let (_, takes) = gather(&unit, 1.0);
        let d: Vec<f32> = takes.iter().map(|l| l.rms[2][spdif_l] - l.rms[1][pb3]).collect();
        let finite: Vec<f32> = d.iter().copied().filter(|x| x.is_finite()).collect();
        if finite.len() < d.len() / 2 {
            f32::NEG_INFINITY
        } else {
            finite.iter().sum::<f32>() / finite.len() as f32
        }
    };
    let (most, _) = gather(&unit, 0.5);
    if most.rms[1][pb3] < -100.0 {
        eprintln!("ucx-mixer: no code on playback 3: run `ucx-stream selftest --seconds 60` meanwhile");
        return ExitCode::FAILURE;
    }
    let mut ok = true;
    let mut report = |what: &str, got: f32, pass: bool| {
        eprintln!("{} {what}: {got:+.2} dB", if pass { "ok " } else { "BAD" });
        ok &= pass;
    };

    view.change_send(SPDIF, CODE, Send { level: -60, pan: 0 });
    flush(&mut view);
    let d = measure();
    report("playback 3 -> SPDIF L at -6.0 dB", d, (d + 6.0).abs() <= 0.3);
    // SPDIF's own processing, from the card's path: dynamics at 1:1 are their gain
    view.change_send(SPDIF, CODE, Send { level: 0, pan: 0 });
    let mut processing = Settings::TOTALMIX;
    processing.dynamics.on = true;
    processing.dynamics.gain = -100;
    view.change_output_settings(SPDIF, processing);
    flush(&mut view);
    let d = measure();
    report("SPDIF out, dynamics on at 1:1, gain -10 dB", d, (d + 10.0).abs() <= 0.3);
    view.change_output_settings(SPDIF, Settings::TOTALMIX);
    view.change_send(SPDIF, CODE, Send::OFF);
    flush(&mut view);
    let d = measure();
    report("playback 3 -> SPDIF L off", d, d < -40.0);

    let held = view.held.len();
    view.change_send(0, CODE, Send { level: 0, pan: 0 });
    let kept_back = view.outgoing.is_empty() && view.held.len() > held;
    eprintln!("{} an analog submix's change held back ({} words)", if kept_back { "ok " } else { "BAD" }, view.held.len() - held);
    ok &= kept_back;
    view.outgoing.clear();

    view.mixer = Mixer::totalmix();
    view.send_whole_mix();
    flush(&mut view);
    let end = Instant::now() + Duration::from_secs(10);
    while unit.counts().2 > 0 && Instant::now() < end {
        thread::sleep(Duration::from_millis(50));
    }
    let (sent, lost, waiting) = unit.counts();
    eprintln!("{sent} words sent, {lost} lost, {waiting} still waiting; the digital outputs as TotalMix left them");
    if ok && lost == 0 && waiting == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let value = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
    if args.iter().any(|a| a == "--check-send") {
        return check_send();
    }
    if let Some(file) = value("--picture") {
        let after = value("--after").and_then(|s| s.parse().ok()).unwrap_or(3.0);
        return picture(file, after);
    }
    if let Some(a) = args.first() {
        eprintln!("ucx-mixer: unknown {a:?}\nusage: ucx-mixer [--picture FILE.png [--after SECONDS] | --check-send]");
        return ExitCode::from(2);
    }
    window()
}
