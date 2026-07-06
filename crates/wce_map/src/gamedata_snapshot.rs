//! Characterization test for `GameData` loading — arbiter for the
//! SLK parser rewrite (see `todos/12_plan_refonte_slkparser.md`).
//!
//! Compares a stable digest of the 14 SLK tables loaded by `GameData::new`
//! against the fixture `resources/test_fixtures/gamedata_snapshot.txt`.
//!
//! To regenerate the fixture (only after a documented investigation
//! of a divergence):
//! ```text
//! cargo test -p wce_map generate_gamedata_snapshot -- --ignored
//! ```

use std::fmt::Write as _;

use crate::slk_datas::SLKData;
use crate::{get_resources_path, GameData};

fn fixture_path() -> String {
    format!("{}test_fixtures/gamedata_snapshot.txt", get_resources_path())
}

/// 64-bit FNV-1a — deterministic across Rust versions,
/// unlike `DefaultHasher`.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Canonical serialization of a table: sorted headers then sorted lines.
fn canonical(table: &SLKData) -> String {
    let mut out = String::new();
    for (col, label) in table.headers() {
        write!(out, "H{col}={label};").unwrap();
    }
    out.push('\n');
    let mut ids: Vec<&String> = table.lines().keys().collect();
    ids.sort();
    for id in ids {
        write!(out, "{id}|").unwrap();
        for (col, val) in &table.lines()[id] {
            write!(out, "{col}={val};").unwrap();
        }
        out.push('\n');
    }
    out
}

fn digest_line(name: &str, table: &SLKData) -> String {
    format!(
        "{name} lines={} headers={} fnv={:016x}\n",
        table.lines().len(),
        table.headers().len(),
        fnv1a(canonical(table).as_bytes())
    )
}

/// Full dump of a known entry — makes a divergence readable
/// where the fnv only says "something changed".
fn dump_entry(out: &mut String, table_name: &str, table: &SLKData, id: &str) {
    writeln!(out, "[{table_name}:{id}]").unwrap();
    let formatted = table
        .get_formatted(&id.to_string())
        // NB: positional form required — wce_map is on edition 2018, where
        // `panic!("... {id} ...")` doesn't interpolate (lint `non_fmt_panics`).
        .unwrap_or_else(|| panic!("id '{}' missing from {}", id, table_name));
    for (key, value) in formatted {
        writeln!(out, "{key}={value}").unwrap();
    }
}

fn snapshot() -> String {
    let game_data = GameData::new(&get_resources_path()).expect("GameData::new");
    let tables: [(&str, &SLKData); 14] = [
        ("unit_data", &game_data.unit_data),
        ("unit_meta", &game_data.unit_meta),
        ("doodad_meta", &game_data.doodad_meta),
        ("destructable_meta", &game_data.destructable_meta),
        ("abilty_meta", &game_data.abilty_meta),
        ("upgrade_meta", &game_data.upgrade_meta),
        ("upgrade_effect_meta", &game_data.upgrade_effect_meta),
        ("const_meta", &game_data.const_meta),
        ("ui_const_meta", &game_data.ui_const_meta),
        ("ability_buff_meta", &game_data.ability_buff_meta),
        ("ability_data", &game_data.ability_data),
        ("upgrade_data", &game_data.upgrade_data),
        ("doodad_effect_data", &game_data.doodad_effect_data),
        ("destructable_effect_data", &game_data.destructable_effect_data),
    ];
    let mut out = String::new();
    for (name, table) in tables {
        out.push_str(&digest_line(name, table));
    }
    dump_entry(&mut out, "unit_data", &game_data.unit_data, "hfoo");
    dump_entry(&mut out, "ability_data", &game_data.ability_data, "AHbz");
    out
}

#[test]
fn gamedata_snapshot_matches_fixture() {
    let expected = std::fs::read_to_string(fixture_path()).expect(
        "missing fixture: cargo test -p wce_map generate_gamedata_snapshot -- --ignored",
    );
    pretty_assertions::assert_eq!(expected, snapshot());
}

#[test]
#[ignore = "regenerates the fixture — only after a documented investigation"]
fn generate_gamedata_snapshot() {
    std::fs::create_dir_all(format!("{}test_fixtures", get_resources_path())).unwrap();
    std::fs::write(fixture_path(), snapshot()).unwrap();
}
