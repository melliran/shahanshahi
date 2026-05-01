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

/// Returns the English transliteration of the Shahanshahi month name for a
/// 1-based month index (1 = Farvardin … 12 = Esfand), as fixed by the 1925
/// Iranian calendar law and listed in SPEC.md § Months.
pub fn month_name(month: u8) -> &'static str {
    const NAMES: [&str; 12] = [
        "Farvardin",
        "Ordibehesht",
        "Khordad",
        "Tir",
        "Mordad",
        "Shahrivar",
        "Mehr",
        "Aban",
        "Azar",
        "Dey",
        "Bahman",
        "Esfand",
    ];
    match month {
        1..=12 => NAMES[(month - 1) as usize],
        _ => "?",
    }
}

/// Formats a date as `"D MonthName YYYY"` (e.g. `"1 Farvardin 2535"`).
/// Intended for Shahanshahi output with `--month-names`.
pub fn format_named(d: &DateTriple) -> String {
    format!("{} {} {}", d.day, month_name(d.month), d.year)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positive_year() {
        let d = parse_ymd("1976-03-21").unwrap();
        assert_eq!((d.year, d.month, d.day), (1976, 3, 21));
    }

    #[test]
    fn parse_negative_year() {
        let d = parse_ymd("-0559-01-01").unwrap();
        assert_eq!((d.year, d.month, d.day), (-559, 1, 1));
    }

    #[test]
    fn parse_no_leading_zeros() {
        let d = parse_ymd("2535-1-1").unwrap();
        assert_eq!((d.year, d.month, d.day), (2535, 1, 1));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert!(parse_ymd("not-a-date").is_err());
        assert!(parse_ymd("1976/03/21").is_err());
        assert!(parse_ymd("1976-03").is_err());
    }

    #[test]
    fn format_round_trip() {
        let d = DateTriple {
            year: 2535,
            month: 1,
            day: 1,
        };
        assert_eq!(format_ymd(&d), "2535-01-01");
    }

    #[test]
    fn convert_gregorian_to_shahanshahi() {
        let input = parse_ymd("1976-03-21").unwrap();
        let output = convert(input, &Calendar::Gregorian, &Calendar::Shahanshahi, false).unwrap();
        assert_eq!((output.year, output.month, output.day), (2535, 1, 1));
    }

    #[test]
    fn convert_shahanshahi_to_gregorian() {
        let input = parse_ymd("2535-01-01").unwrap();
        let output = convert(input, &Calendar::Shahanshahi, &Calendar::Gregorian, false).unwrap();
        assert_eq!((output.year, output.month, output.day), (1976, 3, 21));
    }

    #[test]
    fn convert_rejects_out_of_era_without_proleptic() {
        let input = parse_ymd("1996-03-20").unwrap();
        assert!(convert(input, &Calendar::Gregorian, &Calendar::Shahanshahi, false).is_err());
    }

    #[test]
    fn convert_accepts_out_of_era_with_proleptic() {
        let input = parse_ymd("1996-03-20").unwrap();
        let output = convert(input, &Calendar::Gregorian, &Calendar::Shahanshahi, true).unwrap();
        assert_eq!((output.year, output.month, output.day), (2555, 1, 1));
    }

    #[test]
    fn month_name_all_twelve() {
        let expected = [
            "Farvardin",
            "Ordibehesht",
            "Khordad",
            "Tir",
            "Mordad",
            "Shahrivar",
            "Mehr",
            "Aban",
            "Azar",
            "Dey",
            "Bahman",
            "Esfand",
        ];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(month_name(i as u8 + 1), *name);
        }
    }

    #[test]
    fn month_name_out_of_range_returns_fallback() {
        assert_eq!(month_name(0), "?");
        assert_eq!(month_name(13), "?");
    }

    #[test]
    fn format_named_produces_day_name_year() {
        let d = DateTriple {
            year: 2535,
            month: 1,
            day: 1,
        };
        assert_eq!(format_named(&d), "1 Farvardin 2535");
    }

    #[test]
    fn format_named_last_month() {
        let d = DateTriple {
            year: 2535,
            month: 12,
            day: 29,
        };
        assert_eq!(format_named(&d), "29 Esfand 2535");
    }
}
