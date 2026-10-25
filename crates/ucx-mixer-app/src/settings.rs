//! The window's state between runs: the mix, the submix shown and the theme, as JSON in
//! `$XDG_CONFIG_HOME/ucx-mixer/state.json` (`~/.config/ucx-mixer/state.json`).
//!
//! The mix saved is the model's, sent to the unit or not: the unit can't be read back, so it
//! holds what was last sent (Send whole mix sends the rest).

use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ucx_mixer::Mixer;

use crate::snapshots::Snapshot;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Saved {
    pub mix: Mixer,
    pub submix: usize,
    pub theme: String,
    #[serde(default)]
    pub snapshots: Vec<Option<Snapshot>>,
}

/// Where it's kept (none without a home folder).
pub fn path() -> Option<PathBuf> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|d| !d.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(config.join("ucx-mixer").join("state.json"))
}

/// What was saved last, if anything could be read.
pub fn load() -> Option<Saved> {
    let path = path()?;
    let text = fs::read_to_string(&path).ok()?;
    match serde_json::from_str(&text) {
        Ok(saved) => Some(saved),
        Err(e) => {
            eprintln!("ucx-mixer: {} isn't a saved state ({e}); starting from TotalMix's", path.display());
            None
        }
    }
}

/// Saves `saved`: to a new file first, then in its place, so a crash leaves the old one whole.
pub fn save(saved: &Saved) -> io::Result<()> {
    let path = path().ok_or_else(|| io::Error::other("no home folder"))?;
    fs::create_dir_all(path.parent().unwrap())?;
    let new = path.with_extension("json.new");
    fs::write(&new, serde_json::to_string_pretty(saved).map_err(io::Error::other)?)?;
    fs::rename(&new, &path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_state_round_trips_through_its_file() {
        let dir = std::env::temp_dir().join(format!("ucx-mixer-test-{}", std::process::id()));
        // (this test's own config home: the process's environment, set before anything reads it)
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        assert_eq!(path(), Some(dir.join("ucx-mixer/state.json")));
        let mut mix = Mixer::totalmix();
        mix.set_send(4, ucx_mixer::Source::Input(1), ucx_mixer::Send { level: -35, pan: 20 });
        let snap = Snapshot { name: "One".into(), mix: Mixer::silent() };
        let saved = Saved { mix, submix: 4, theme: "Paper".into(), snapshots: vec![None, Some(snap)] };
        save(&saved).unwrap();
        assert_eq!(load(), Some(saved));
        fs::write(path().unwrap(), "{ not json").unwrap();
        assert_eq!(load(), None);
        fs::remove_dir_all(&dir).unwrap();
    }
}
