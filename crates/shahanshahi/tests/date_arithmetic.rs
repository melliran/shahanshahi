//! Day arithmetic (`checked_add_days` / `checked_sub_days`, issue #80) checked against an
//! independent Gregorian day count, so a bug in the crate's own Rata Die cannot hide itself.

mod common;

use common::days_from_civil;
use shahanshahi::{GregorianDate, ShahanshahiDate};

fn day_number(date: ShahanshahiDate) -> i64 {
    let gregorian = date.to_gregorian();
    days_from_civil(gregorian.year(), gregorian.month(), gregorian.day())
}

fn era_start() -> ShahanshahiDate {
    ShahanshahiDate::try_new(2535, 1, 1).unwrap()
}

/// Every date of the legal era, built by stepping one day at a time.
fn all_legal_era_dates() -> Vec<ShahanshahiDate> {
    let mut dates = vec![era_start()];
    while let Some(next) = dates.last().unwrap().checked_add_days(1) {
        dates.push(next);
    }
    dates
}

#[test]
fn stepping_covers_legal_era_with_consecutive_gregorian_days() {
    let dates = all_legal_era_dates();
    let last = *dates.last().unwrap();
    assert_eq!(last, ShahanshahiDate::try_new(2537, 6, 10).unwrap());
    for pair in dates.windows(2) {
        assert_eq!(
            day_number(pair[1]) - day_number(pair[0]),
            1,
            "{} → {}",
            pair[0],
            pair[1]
        );
        assert!(pair[0] < pair[1]);
    }
    // The i-th stepped date is exactly i Gregorian days after the anchor, and agrees with
    // Gregorian → Shahanshahi conversion of that day.
    let start_day = day_number(era_start());
    for (i, &date) in dates.iter().enumerate() {
        assert_eq!(day_number(date) - start_day, i as i64);
        let gregorian: GregorianDate = date.to_gregorian();
        assert_eq!(ShahanshahiDate::try_from_gregorian(gregorian), Ok(date));
    }
}

#[test]
fn add_then_sub_round_trips_and_matches_day_count() {
    let dates = all_legal_era_dates();
    let len = dates.len() as i32;
    for (i, &from) in dates.iter().enumerate().step_by(7) {
        for n in (-len..=len).step_by(13) {
            let target_index = i as i32 + n;
            let result = from.checked_add_days(n);
            if (0..len).contains(&target_index) {
                let to = result.unwrap_or_else(|| panic!("{from} + {n} should be in era"));
                assert_eq!(to, dates[target_index as usize], "{from} + {n}");
                assert_eq!(day_number(to) - day_number(from), i64::from(n));
                assert_eq!(to.checked_sub_days(n), Some(from), "{to} - {n}");
                assert_eq!(from.checked_sub_days(-n), Some(to));
            } else {
                assert_eq!(result, None, "{from} + {n} should leave the era");
            }
        }
    }
}

#[cfg(feature = "proleptic")]
#[test]
fn proleptic_add_matches_independent_day_count_across_centuries() {
    let anchor = era_start();
    let anchor_day = day_number(anchor);
    // ±200 years in uneven steps, crossing many leap Esfands.
    for n in (-73_000..=73_000).step_by(997) {
        let shifted = anchor.checked_add_days_proleptic(n).unwrap();
        assert_eq!(
            day_number(shifted) - anchor_day,
            i64::from(n),
            "anchor + {n}"
        );
        assert_eq!(shifted.checked_sub_days_proleptic(n), Some(anchor));
    }
}
