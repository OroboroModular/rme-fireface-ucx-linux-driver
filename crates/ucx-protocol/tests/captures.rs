//! Checks the protocol model against data exported from real captures (tools/export_fixtures.py).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use ucx_protocol::control::{power_on_sequence, VendorWrite};
use ucx_protocol::iso::{self, InPacket};
use ucx_protocol::meters::{power_dbfs, MeterKind, MeterPacket};
use ucx_protocol::regs::{fader_table_reg, global, matrix_reg, Source};
use ucx_protocol::values::{FaderTableField, MatrixGain};
use ucx_protocol::Word;

fn fixture(name: &str) -> Vec<String> {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", name].iter().collect();
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap()
}

fn state_push() -> Vec<Word> {
    fixture("state_push.txt")
        .iter()
        .map(|l| {
            let raw = hex(l);
            let w = Word::decode(raw).expect("odd parity");
            assert_eq!(w.encode(), raw, "re-encoding must give the captured word");
            w
        })
        .collect()
}

#[test]
fn state_push_is_matrix_fader_table_and_settings() {
    let words = state_push();
    assert_eq!(words.len(), 1108);
    let matrix = words.iter().filter(|w| (0x4000..0x4800).contains(&w.addr)).count();
    let table = words.iter().filter(|w| (w.addr >> 8) >= 0x12 && (w.addr >> 8) <= 0x23 && (w.addr & 0xff) >= 0xe0).count();
    let global = words.iter().filter(|w| w.addr == global::MONITOR_MASK || w.addr == global::MONITOR_FLAGS).count();
    assert_eq!((matrix, table, global), (684, 404, 2));
    assert_eq!(words.len() - matrix - table - global, 18, "known channel settings");
}

#[test]
fn state_push_routing_matches_totalmix() {
    let m: HashMap<u16, MatrixGain> = state_push()
        .into_iter()
        .filter(|w| (0x4000..0x4800).contains(&w.addr))
        .map(|w| (w.addr, MatrixGain(w.value)))
        .collect();
    // each playback channel on its own output at unity
    for n in 0..18u8 {
        assert_eq!(m[&matrix_reg(n, Source::Playback(n))], MatrixGain::UNITY, "playback {n}");
    }
    // inputs AN 7/8 -> AN 1/2 at -10.6 dB
    for (out, input) in [(0u8, 6u8), (1, 7)] {
        assert!((m[&matrix_reg(out, Source::Input(input))].db() + 10.6).abs() < 0.1);
    }
    // and nothing from Mic 1 anywhere
    for out in 0..18u8 {
        assert_eq!(m[&matrix_reg(out, Source::Input(0))], MatrixGain::ZERO);
    }
}

#[test]
fn state_push_fader_table_holds_the_an78_route() {
    let fields: Vec<FaderTableField> = state_push()
        .into_iter()
        .filter(|w| w.addr == fader_table_reg(0, 6))
        .map(|w| FaderTableField::decode(w.value))
        .collect();
    assert!(fields.contains(&FaderTableField::Level(-106)), "{fields:?}");
    assert!(fields.contains(&FaderTableField::Pan(-100)), "{fields:?}");
}

#[test]
fn power_on_sequence_matches_capture() {
    let captured: Vec<VendorWrite> = fixture("power_on.txt")
        .iter()
        .map(|l| {
            let f: Vec<u32> = l.split_whitespace().map(hex).collect();
            VendorWrite { request: f[0] as u8, value: f[1] as u16, index: f[2] as u16 }
        })
        .collect();
    assert_eq!(power_on_sequence(), captured);
}

#[test]
fn meters_from_pattern_stream() {
    let packets: Vec<MeterPacket> = fixture("meters.txt")
        .iter()
        .map(|l| {
            let b: Vec<u8> = (0..l.len()).step_by(2).map(|i| u8::from_str_radix(&l[i..i + 2], 16).unwrap()).collect();
            MeterPacket::parse(&b).unwrap()
        })
        .collect();
    assert_eq!(packets.len(), 5);
    let playback = packets.iter().find(|p| p.kind == MeterKind::Playback).unwrap();
    for n in 0..18 {
        let v = (n + 1) as u64; // playback n carried the 24-bit value n + 1
        // power is averaged over a block, so allow a little; the peak is exact
        let expected = 512 * v * v;
        assert!(playback.power[n].abs_diff(expected) * 200 <= expected, "power {n}: {}", playback.power[n]);
        assert_eq!(playback.peak[n] as u64, 16 * v, "peak {n}");
    }
    // a 24-bit value of 1 is -138.5 dBFS
    assert!((power_dbfs(512) + 138.47).abs() < 0.01);
}

#[test]
fn iso_in_packets_from_linux() {
    let raw: Vec<Vec<u8>> = fixture("iso_in_44k1.txt")
        .iter()
        .map(|l| (0..l.len()).step_by(2).map(|i| u8::from_str_radix(&l[i..i + 2], 16).unwrap()).collect())
        .collect();
    let packets: Vec<InPacket> = raw.iter().map(|b| InPacket::parse(b, iso::channels(1)).unwrap()).collect();
    assert_eq!(packets.len(), 12);
    // 44.1 kHz: 5 and 6 frames by turns, and each header continues the last one's position,
    // across the wrap at 3072
    assert!(packets.iter().all(|p| p.frames() == 5 || p.frames() == 6));
    for (a, b) in packets.iter().zip(&packets[1..]) {
        assert_eq!(a.header.next_position(), b.header.position);
    }
    assert!(packets.iter().any(|p| p.header.position < 6), "fixture should cross the wrap");
    for p in &packets {
        // analog inputs: noise around -110 dBFS; SPDIF and ADAT, with nothing connected: exactly 0
        for c in 0..8 {
            assert!(p.channel(c).all(|x| x.abs() < 1e-4), "channel {c}");
        }
        assert!((0..8).any(|c| p.channel(c).any(|x| x != 0.0)));
        for c in 8..18 {
            assert!(p.channel(c).all(|x| x == 0.0), "channel {c}");
        }
    }
}

#[test]
fn matrix_probes_match_totalmix_display() {
    for l in fixture("matrix_probes.txt") {
        let (db, value) = l.split_once(' ').unwrap();
        let db: f64 = db.parse().unwrap();
        let g = MatrixGain(hex(value) as u16);
        assert!((g.db() - db).abs() < 0.06, "{l}: decodes to {:.2} dB", g.db());
        assert!((MatrixGain::from_db(db).db() - db).abs() < 0.01, "{l}");
    }
}
