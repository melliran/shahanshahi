use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

use crate::cli::{Calendar, Format};
use crate::convert::{self, DateTriple};

#[derive(Serialize, Deserialize)]
struct DateRecord {
    year: i32,
    month: u8,
    day: u8,
}

impl DateRecord {
    fn from_triple(d: &DateTriple) -> Self {
        Self {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }

    fn into_triple(self) -> DateTriple {
        DateTriple {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
}

pub fn write_single(format: &Format, output: &DateTriple) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match format {
        Format::Text => {
            writeln!(out, "{}", convert::format_ymd(output))?;
        }
        Format::Json => {
            serde_json::to_writer(&mut out, &DateRecord::from_triple(output))?;
            writeln!(out)?;
        }
        Format::Csv => {
            let mut wtr = csv::Writer::from_writer(&mut out);
            wtr.serialize(DateRecord::from_triple(output))?;
            wtr.flush()?;
        }
    }

    Ok(())
}

pub fn run_batch(format: &Format, from: &Calendar, to: &Calendar, proleptic: bool) -> Result<()> {
    match format {
        Format::Text => run_text_batch(from, to, proleptic),
        Format::Json => run_json_batch(from, to, proleptic),
        Format::Csv => run_csv_batch(from, to, proleptic),
    }
}

fn run_text_batch(from: &Calendar, to: &Calendar, proleptic: bool) -> Result<()> {
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
        writeln!(out, "{}", convert::format_ymd(&output))?;
    }

    Ok(())
}

fn run_json_batch(from: &Calendar, to: &Calendar, proleptic: bool) -> Result<()> {
    let stdin = io::stdin();
    let records: Vec<DateRecord> =
        serde_json::from_reader(stdin.lock()).context("failed to parse JSON array from stdin")?;

    let mut results = Vec::with_capacity(records.len());
    for rec in records {
        let output = convert::convert(rec.into_triple(), from, to, proleptic)?;
        results.push(DateRecord::from_triple(&output));
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer(&mut out, &results)?;
    writeln!(out)?;

    Ok(())
}

fn run_csv_batch(from: &Calendar, to: &Calendar, proleptic: bool) -> Result<()> {
    let stdin = io::stdin();
    let mut rdr = csv::Reader::from_reader(stdin.lock());

    let stdout = io::stdout();
    let mut wtr = csv::Writer::from_writer(stdout.lock());

    for result in rdr.deserialize() {
        let rec: DateRecord = result.context("failed to parse CSV row")?;
        let output = convert::convert(rec.into_triple(), from, to, proleptic)?;
        wtr.serialize(DateRecord::from_triple(&output))?;
    }
    wtr.flush()?;

    Ok(())
}
