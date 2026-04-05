use anyhow::{anyhow, Context, Result};
use shahanshahi::{GregorianDate, ShahanshahiDate};

use crate::cli::Calendar;

pub struct DateTriple {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

pub fn convert(
    input: DateTriple,
    from: &Calendar,
    to: &Calendar,
    proleptic: bool,
) -> Result<DateTriple> {
    match (from, to) {
        (Calendar::Gregorian, Calendar::Shahanshahi) => {
            let g = GregorianDate::try_new(input.year, input.month, input.day)
                .map_err(|e| anyhow!("{e}"))?;
            let sh = if proleptic {
                ShahanshahiDate::try_from_gregorian_proleptic(g)
            } else {
                ShahanshahiDate::try_from_gregorian(g)
            }
            .map_err(|e| anyhow!("{e}"))?;
            Ok(DateTriple {
                year: sh.year(),
                month: sh.month(),
                day: sh.day(),
            })
        }
        (Calendar::Shahanshahi, Calendar::Gregorian) => {
            let sh = if proleptic {
                ShahanshahiDate::try_new_proleptic(input.year, input.month, input.day)
            } else {
                ShahanshahiDate::try_new(input.year, input.month, input.day)
            }
            .map_err(|e| anyhow!("{e}"))?;
            let g = sh.to_gregorian();
            Ok(DateTriple {
                year: g.year(),
                month: g.month(),
                day: g.day(),
            })
        }
        _ => unreachable!("direction validated before calling convert"),
    }
}

pub fn parse_ymd(s: &str) -> Result<DateTriple> {
    let (negative, rest) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s),
    };

    let parts: Vec<&str> = rest.splitn(3, '-').collect();
    if parts.len() != 3 {
        return Err(anyhow!("expected YYYY-MM-DD, got: {s}"));
    }

    let mut year: i32 = parts[0].parse().context("invalid year")?;
    if negative {
        year = -year;
    }
    let month: u8 = parts[1].parse().context("invalid month")?;
    let day: u8 = parts[2].parse().context("invalid day")?;

    Ok(DateTriple { year, month, day })
}

pub fn format_ymd(d: &DateTriple) -> String {
    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
}
