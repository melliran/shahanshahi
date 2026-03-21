//! Solar Hijri leap arithmetic — **Mode A** per [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md) (33-year rule).

/// Solar Hijri leap year per the 33-year arithmetic rule (Heydari-Malayeri / Borkowski range).
#[inline]
pub fn is_solar_hijri_leap_arithmetic(y_h: i32) -> bool {
    let r = y_h.rem_euclid(33);
    matches!(r, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
}

/// Underlying Solar Hijri year from Shahanshahi year (proleptic offset).
#[inline]
pub fn shahanshahi_to_hijri_shamsi_year(y_s: i32) -> i32 {
    y_s - 1180
}

/// Shahanshahi leap year using Mode A (same solar year as underlying Hijri Shamsi).
#[inline]
pub fn is_shahanshahi_leap_arithmetic(y_s: i32) -> bool {
    is_solar_hijri_leap_arithmetic(shahanshahi_to_hijri_shamsi_year(y_s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_window_1354_1357() {
        assert!(is_solar_hijri_leap_arithmetic(1354));
        assert!(!is_solar_hijri_leap_arithmetic(1355));
        assert!(!is_solar_hijri_leap_arithmetic(1356));
        assert!(!is_solar_hijri_leap_arithmetic(1357));
    }

    #[test]
    fn offset_maps_shahanshahi_to_hijri_year() {
        assert_eq!(shahanshahi_to_hijri_shamsi_year(2535), 1355);
        assert_eq!(
            is_shahanshahi_leap_arithmetic(2534),
            is_solar_hijri_leap_arithmetic(1354)
        );
    }
}
