//! Shahanshahi ↔ Gregorian via **one** internal scale: **Rata Die** (see [`crate::rata_die`]).
//!
//! Anchor (SPEC.md + `data/reference-dates.json`): **1 Farvardin 2535** Shahanshahi ≡ **1976-03-21** Gregorian.

use crate::gregorian::GregorianDate;
use crate::leap::{days_in_shahanshahi_month, is_shahanshahi_leap_arithmetic};
use crate::rata_die::{gregorian_to_rata_die, rata_die_to_gregorian};

/// Shahanshahi year of the normative anchor (`1 Farvardin 2535`).
const ANCHOR_SH_YEAR: i32 = 2535;

/// Rata Die of **1976-03-21** (must match [`crate::rata_die::gregorian_to_rata_die`]).
const ANCHOR_RD: i32 = 721_434;

#[inline]
fn shahanshahi_year_length_days(year: i32) -> i32 {
    if is_shahanshahi_leap_arithmetic(year) {
        366
    } else {
        365
    }
}

/// Signed day offset from **1 Farvardin `ANCHOR_SH_YEAR`** to the given Shahanshahi civil date.
///
/// Counts whole civil days after the anchor’s Farvardin 1: walk year-by-year using Mode A year
/// lengths, then add completed months in the target year, then `day - 1` within the target month.
fn offset_from_anchor(year: i32, month: u8, day: u8) -> i64 {
    let mut cursor_year = ANCHOR_SH_YEAR;
    let mut offset_days = 0i64;
    while cursor_year != year {
        if cursor_year < year {
            offset_days += i64::from(shahanshahi_year_length_days(cursor_year));
            cursor_year += 1;
        } else {
            cursor_year -= 1;
            offset_days -= i64::from(shahanshahi_year_length_days(cursor_year));
        }
    }
    for month_idx in 1..month {
        offset_days += i64::from(days_in_shahanshahi_month(cursor_year, month_idx));
    }
    offset_days + i64::from(day) - 1
}

/// Map “days after 1 Farvardin of the anchor year” (may be negative) to Shahanshahi YMD.
///
/// Normalizes the offset into `[0, year_length)`, then consumes whole months in civil order.
fn shahanshahi_ymd_from_offset(mut offset_days: i64) -> (i32, u8, u8) {
    let mut year = ANCHOR_SH_YEAR;
    while offset_days < 0 {
        year -= 1;
        offset_days += i64::from(shahanshahi_year_length_days(year));
    }
    while offset_days >= i64::from(shahanshahi_year_length_days(year)) {
        let year_len = i64::from(shahanshahi_year_length_days(year));
        offset_days -= year_len;
        year += 1;
    }
    let mut month = 1u8;
    loop {
        let days_this_month = i64::from(days_in_shahanshahi_month(year, month));
        if offset_days < days_this_month {
            return (year, month, (offset_days + 1) as u8);
        }
        offset_days -= days_this_month;
        month += 1;
    }
}

#[inline]
pub(crate) fn shahanshahi_to_gregorian(year: i32, month: u8, day: u8) -> GregorianDate {
    let rata_die = i64::from(ANCHOR_RD) + offset_from_anchor(year, month, day);
    let (g_year, g_month, g_day) = rata_die_to_gregorian(rata_die);
    GregorianDate::from_ymd_unchecked(g_year, g_month, g_day)
}

#[inline]
pub(crate) fn gregorian_to_shahanshahi_ymd(date: GregorianDate) -> (i32, u8, u8) {
    let rata_die = gregorian_to_rata_die(date.year(), date.month(), date.day());
    shahanshahi_ymd_from_offset(rata_die - i64::from(ANCHOR_RD))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ShahanshahiDate;

    #[test]
    fn anchor_round_trip_through_public_date_api() {
        let g = shahanshahi_to_gregorian(2535, 1, 1);
        assert_eq!(
            (g.year(), g.month(), g.day()),
            (1976, 3, 21),
            "SPEC / golden anchor"
        );
        let (y, m, d) = gregorian_to_shahanshahi_ymd(g);
        assert_eq!((y, m, d), (2535, 1, 1));
        let back = ShahanshahiDate::try_new(y, m, d).unwrap();
        assert_eq!(back.to_gregorian(), g);
    }
}
