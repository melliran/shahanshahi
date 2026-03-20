//! Golden corpus at workspace `data/reference-dates.json` (issue #2).

use serde::Deserialize;
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
