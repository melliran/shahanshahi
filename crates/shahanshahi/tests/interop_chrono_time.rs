//! Round-trip through `chrono` and `time` (features **`chrono`** + **`time`**).

use shahanshahi::{GregorianDate, ShahanshahiDate};

#[test]
fn chrono_legal_era_round_trip() {
    let sh = ShahanshahiDate::try_new(2535, 1, 1).unwrap();
    let n = sh.to_chrono_naive_date().unwrap();
    let sh2 = ShahanshahiDate::try_from_chrono_naive_date(n).unwrap();
    assert_eq!(sh, sh2);
}

#[test]
fn time_legal_era_round_trip() {
    let sh = ShahanshahiDate::try_new(2535, 4, 31).unwrap();
    let t = sh.to_time_date();
    let sh2 = ShahanshahiDate::try_from_time_date(t).unwrap();
    assert_eq!(sh, sh2);
}

#[test]
fn gregorian_chrono_time_agree() {
    let g = GregorianDate::try_new(1976, 7, 22).unwrap();
    let n = g.to_chrono_naive_date().unwrap();
    let t = g.to_time_date();
    assert_eq!(
        GregorianDate::try_from_chrono_naive_date(n).unwrap(),
        GregorianDate::try_from_time_date(t).unwrap(),
    );
}

#[test]
fn chrono_rejects_year_outside_naive_date_range() {
    // chrono::NaiveDate supports a bounded year range; GregorianDate does not.
    let g = GregorianDate::try_new(300_000, 1, 1).unwrap();
    assert!(g.to_chrono_naive_date().is_err());
}
