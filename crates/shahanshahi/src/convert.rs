//! Shahanshahi ↔ Gregorian via **one** internal scale: **Rata Die** (see [`crate::rata_die`]).
//!
//! Anchor (SPEC.md + `data/reference-dates.json`): **1 Farvardin 2535** Shahanshahi ≡ **1976-03-21** Gregorian.
#![cfg_attr(not(test), allow(dead_code))]

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
fn offset_from_anchor(year: i32, month: u8, day: u8) -> i64 {
    let mut y = ANCHOR_SH_YEAR;
    let mut n = 0i64;
    while y != year {
        if y < year {
            n += i64::from(shahanshahi_year_length_days(y));
            y += 1;
        } else {
            y -= 1;
            n -= i64::from(shahanshahi_year_length_days(y));
        }
    }
    for m in 1..month {
        n += i64::from(days_in_shahanshahi_month(y, m));
    }
    n + i64::from(day) - 1
}

fn shahanshahi_ymd_from_offset(mut n: i64) -> (i32, u8, u8) {
    let mut y = ANCHOR_SH_YEAR;
    while n < 0 {
        y -= 1;
        n += i64::from(shahanshahi_year_length_days(y));
    }
    while n >= i64::from(shahanshahi_year_length_days(y)) {
        let len = i64::from(shahanshahi_year_length_days(y));
        n -= len;
        y += 1;
    }
    let mut m = 1u8;
    loop {
        let dim = i64::from(days_in_shahanshahi_month(y, m));
        if n < dim {
            return (y, m, (n + 1) as u8);
        }
        n -= dim;
        m += 1;
    }
}

#[inline]
pub(crate) fn shahanshahi_to_gregorian(year: i32, month: u8, day: u8) -> GregorianDate {
    let rd = i64::from(ANCHOR_RD) + offset_from_anchor(year, month, day);
    let (gy, gm, gd) = rata_die_to_gregorian(rd);
    GregorianDate::from_ymd_unchecked(gy, gm, gd)
}

#[inline]
pub(crate) fn gregorian_to_shahanshahi_ymd(date: GregorianDate) -> (i32, u8, u8) {
    let rd = gregorian_to_rata_die(date.year(), date.month(), date.day());
    shahanshahi_ymd_from_offset(rd - i64::from(ANCHOR_RD))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_maps_to_1976_03_21_and_back() {
        let g = shahanshahi_to_gregorian(2535, 1, 1);
        assert_eq!((g.year(), g.month(), g.day()), (1976, 3, 21));
        let (y, m, d) = gregorian_to_shahanshahi_ymd(g);
        assert_eq!((y, m, d), (2535, 1, 1));
    }
}
