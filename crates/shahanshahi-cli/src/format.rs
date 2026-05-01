use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

use crate::cli::{Calendar, Format};
use crate::convert::{self, DateTriple};

// Input-only: used to parse year/month/day from JSON arrays and CSV rows.
#[derive(Deserialize)]
struct DateRecord {
    year: i32,
    month: u8,
    day: u8,
}

impl DateRecord {
    fn into_triple(self) -> DateTriple {
        DateTriple {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
}

// Output without month names.
#[derive(Serialize)]
struct DateRecordOut {
    year: i32,
    month: u8,
    day: u8,
}

impl DateRecordOut {
    fn from_triple(d: &DateTriple) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }
}

// Output with month names; Serialize only — never parsed back from input.
#[derive(Serialize)]
struct DateRecordNamed {
    year: i32,
    month: u8,
    month_name: &'static str,
    day: u8,
}

impl DateRecordNamed {
    fn from_triple(d: &DateTriple) -> Self {
        Self {
            year: d.year,
            month: d.month,
            month_name: convert::month_name(d.month),
            day: d.day,
        }
    }
}

pub fn write_single(format: &Format, output: &DateTriple, month_names: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match format {
        Format::Text => {
            let line = if month_names {
                convert::format_named(output)
            } else {
                convert::format_ymd(output)
            };
            writeln!(out, "{line}")?;
        }
        Format::Json => {
            if month_names {
                serde_json::to_writer(&mut out, &DateRecordNamed::from_triple(output))?;
            } else {
                serde_json::to_writer(&mut out, &DateRecordOut::from_triple(output))?;
            }
            writeln!(out)?;
        }
        Format::Csv => {
            let mut wtr = csv::Writer::from_writer(&mut out);
            if month_names {
                wtr.serialize(DateRecordNamed::from_triple(output))?;
            } else {
                wtr.serialize(DateRecordOut::from_triple(output))?;
            }
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn run_batch(
    format: &Format,
    from: &Calendar,
    to: &Calendar,
    proleptic: bool,
    month_names: bool,
) -> Result<()> {
    match format {
        Format::Text => run_text_batch(from, to, proleptic, month_names),
        Format::Json => run_json_batch(from, to, proleptic, month_names),
        Format::Csv => run_csv_batch(from, to, proleptic, month_names),
    }
}

fn run_text_batch(
    from: &Calendar,
    to: &Calendar,
    proleptic: bool,
    month_names: bool,
) -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.context("failed to read line from stdin")?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let input =
            convert::parse_ymd(trimmed).with_context(|| format!("parsing line: {trimmed}"))?;
        let output = convert::convert(input, from, to, proleptic)
            .with_context(|| format!("converting: {trimmed}"))?;
        let formatted = if month_names {
            convert::format_named(&output)
        } else {
            convert::format_ymd(&output)
        };
        writeln!(out, "{formatted}")?;
    }

    Ok(())
}

fn run_json_batch(
    from: &Calendar,
    to: &Calendar,
    proleptic: bool,
    month_names: bool,
) -> Result<()> {
    let stdin = io::stdin();
    let records: Vec<DateRecord> =
        serde_json::from_reader(stdin.lock()).context("failed to parse JSON array from stdin")?;

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if month_names {
        let mut results = Vec::with_capacity(records.len());
        for rec in records {
            let output = convert::convert(rec.into_triple(), from, to, proleptic)?;
            results.push(DateRecordNamed::from_triple(&output));
        }
        serde_json::to_writer(&mut out, &results)?;
    } else {
        let mut results = Vec::with_capacity(records.len());
        for rec in records {
            let output = convert::convert(rec.into_triple(), from, to, proleptic)?;
            results.push(DateRecordOut::from_triple(&output));
        }
        serde_json::to_writer(&mut out, &results)?;
    }
    writeln!(out)?;

    Ok(())
}

fn run_csv_batch(
    from: &Calendar,
    to: &Calendar,
    proleptic: bool,
    month_names: bool,
) -> Result<()> {
    let stdin = io::stdin();
    let mut rdr = csv::Reader::from_reader(stdin.lock());

    let stdout = io::stdout();
    let mut wtr = csv::Writer::from_writer(stdout.lock());

    for result in rdr.deserialize() {
        let rec: DateRecord = result.context("failed to parse CSV row")?;
        let output = convert::convert(rec.into_triple(), from, to, proleptic)?;
        if month_names {
            wtr.serialize(DateRecordNamed::from_triple(&output))?;
        } else {
            wtr.serialize(DateRecordOut::from_triple(&output))?;
        }
    }
    wtr.flush()?;

    Ok(())
}
