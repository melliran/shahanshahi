//! Golden corpus at workspace `data/reference-dates.json` (issue #2, #4).

use serde::Deserialize;
use shahanshahi::{GregorianDate, ShahanshahiDate, ShahanshahiDateError};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ReferenceFile {
    schema_version: u32,
    spec_id: String,
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
struct Row {
    shahanshahi: Ymd,
    gregorian: Ymd,
    source: String,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Ymd {
    year: i32,
    month: u8,
    day: u8,
}

fn reference_dates_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/reference-dates.json")
}

#[test]
fn reference_dates_loads_and_matches_spec() {
    let path = reference_dates_path();
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let data: ReferenceFile = serde_json::from_str(&raw).expect("valid JSON");

    assert_eq!(data.schema_version, 1);
    assert_eq!(data.spec_id, shahanshahi::SPEC_VERSION);
    assert!(!data.rows.is_empty(), "golden rows should be populated");

    for (i, row) in data.rows.iter().enumerate() {
        assert!(
            (1..=12).contains(&row.shahanshahi.month),
            "row {i}: shahanshahi month {}",
            row.shahanshahi.month
        );
        assert!(
            (1..=12).contains(&row.gregorian.month),
            "row {i}: gregorian month {}",
            row.gregorian.month
        );
        assert!(!row.source.trim().is_empty(), "row {i}: empty source");
        assert!(
            row.source.len() > 20,
            "row {i}: source should be a substantive citation"
        );
        for (label, ymd) in [
            ("shahanshahi", &row.shahanshahi),
            ("gregorian", &row.gregorian),
        ] {
            assert!(
                (1..=31).contains(&ymd.day),
                "row {i} {label}: day {}",
                ymd.day
            );
            assert!(
                (-5000..=10_000).contains(&ymd.year),
                "row {i} {label}: year {}",
                ymd.year
            );
        }
        let _ = row.notes.as_deref();
    }
}

fn gregorian_from_row(ymd: &Ymd) -> GregorianDate {
    GregorianDate::try_new(ymd.year, ymd.month, ymd.day)
        .unwrap_or_else(|e| panic!("invalid golden Gregorian {:?}: {e}", ymd))
}

/// Rows that fit the crate default **legal Shahanshahi civil era** (`try_new`).
fn shahanshahi_try_new(row: &Row) -> Result<ShahanshahiDate, ShahanshahiDateError> {
    ShahanshahiDate::try_new(
        row.shahanshahi.year,
        row.shahanshahi.month,
        row.shahanshahi.day,
    )
}

#[test]
fn golden_legal_era_shahanshahi_to_gregorian() {
    let path = reference_dates_path();
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let data: ReferenceFile = serde_json::from_str(&raw).expect("valid JSON");

    for (i, row) in data.rows.iter().enumerate() {
        let Ok(sh) = shahanshahi_try_new(row) else {
            continue;
        };
        let g = sh.to_gregorian();
        assert_eq!(
            (g.year(), g.month(), g.day()),
            (row.gregorian.year, row.gregorian.month, row.gregorian.day),
            "row {i}: Shahanshahi → Gregorian mismatch (source: {})",
            row.source
        );
    }
}

#[test]
fn golden_legal_era_gregorian_to_shahanshahi() {
    let path = reference_dates_path();
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let data: ReferenceFile = serde_json::from_str(&raw).expect("valid JSON");

    for (i, row) in data.rows.iter().enumerate() {
        if shahanshahi_try_new(row).is_err() {
            continue;
        }
        let g = gregorian_from_row(&row.gregorian);
        let sh = ShahanshahiDate::try_from_gregorian(g).unwrap_or_else(|e| {
            panic!("row {i}: try_from_gregorian failed for legal-era golden: {e}")
        });
        assert_eq!(
            (sh.year(), sh.month(), sh.day()),
            (
                row.shahanshahi.year,
                row.shahanshahi.month,
                row.shahanshahi.day
            ),
            "row {i}: Gregorian → Shahanshahi mismatch (source: {})",
            row.source
        );
    }
}

/// Full golden corpus including **proleptic** Shahanshahi rows (SPEC.md § Proleptic use).
#[cfg(feature = "proleptic")]
#[test]
fn golden_full_corpus_round_trip() {
    let path = reference_dates_path();
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let data: ReferenceFile = serde_json::from_str(&raw).expect("valid JSON");

    for (i, row) in data.rows.iter().enumerate() {
        let sh = ShahanshahiDate::try_new_proleptic(
            row.shahanshahi.year,
            row.shahanshahi.month,
            row.shahanshahi.day,
        )
        .unwrap_or_else(|e| panic!("row {i}: invalid golden Shahanshahi: {e}"));
        let g = sh.to_gregorian();
        assert_eq!(
            (g.year(), g.month(), g.day()),
            (row.gregorian.year, row.gregorian.month, row.gregorian.day),
            "row {i}: Shahanshahi → Gregorian (proleptic) (source: {})",
            row.source
        );

        let g2 = gregorian_from_row(&row.gregorian);
        let sh2 = ShahanshahiDate::try_from_gregorian_proleptic(g2)
            .unwrap_or_else(|e| panic!("row {i}: try_from_gregorian_proleptic: {e}"));
        assert_eq!(
            (sh2.year(), sh2.month(), sh2.day()),
            (sh.year(), sh.month(), sh.day()),
            "row {i}: Gregorian → Shahanshahi round-trip (source: {})",
            row.source
        );
    }
}
