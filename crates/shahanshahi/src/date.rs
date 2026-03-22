//! Civil Shahanshahi date (year, month, day) with validation per [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md).

use crate::convert;
use crate::gregorian::{GregorianDate, GregorianDateError};
use crate::leap::days_in_shahanshahi_month;
use core::fmt;

/// First civil day of the default **legal Shahanshahi era** (inclusive), per SPEC.md § Era of applicability.
pub const LEGAL_ERA_START_YEAR: i32 = 2535;
pub const LEGAL_ERA_START_MONTH: u8 = 1;
pub const LEGAL_ERA_START_DAY: u8 = 1;

/// Conservative inclusive **last** civil day in this crate’s default policy: **10 Shahrivar 2537**, as cited in
/// SPEC.md until the repealing instrument pins the exact last Shahanshahi-labelled day.
pub const LEGAL_ERA_END_YEAR: i32 = 2537;
pub const LEGAL_ERA_END_MONTH: u8 = 6;
pub const LEGAL_ERA_END_DAY: u8 = 10;

/// A valid civil date in the Shahanshahi (Imperial Iranian) calendar: **1 = Farvardin … 12 = Esfand**,
/// with month lengths from the 1925 solar civil law (SPEC.md § Months). Leap **Esfand** uses **Mode A**
/// (33-year arithmetic) on the underlying Hijri Shamsi year `Y_H = Y_S − 1180`.
///
/// # Legal era (default constructor)
///
/// [`ShahanshahiDate::try_new`] accepts only dates in the **inclusive** window
/// [`LEGAL_ERA_START_YEAR`]/[`LEGAL_ERA_START_MONTH`]/[`LEGAL_ERA_START_DAY`]
/// through [`LEGAL_ERA_END_YEAR`]/[`LEGAL_ERA_END_MONTH`]/[`LEGAL_ERA_END_DAY`], matching SPEC.md’s
/// default crate policy (reject civil dates outside the documented Pahlavi Shahanshahi numbering window).
///
/// # Errors
///
/// Invalid month, impossible day-of-month, or (for `try_new`) a calendar-valid date outside that window
/// yield [`ShahanshahiDateError`].
///
/// # Proleptic labelling
///
/// With the **`proleptic`** Cargo feature, [`ShahanshahiDate::try_new_proleptic`] validates the same
/// month grid and leap rule but **does not** enforce the legal era. Such dates are not historically
/// authorized as civil Shahanshahi outside the era (SPEC.md § Proleptic use).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShahanshahiDate {
    year: i32,
    month: u8,
    day: u8,
}

/// Why a [`ShahanshahiDate`] could not be constructed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShahanshahiDateError {
    /// `month` is not in `1..=12`.
    MonthOutOfRange { month: u8 },
    /// `day` is not valid for this year and month (after leap resolution for Esfand).
    DayOutOfRange {
        year: i32,
        month: u8,
        day: u8,
        max_day: u8,
    },
    /// Calendar-valid date outside the default legal era ([`ShahanshahiDate::try_new`] only).
    OutOfLegalEra { year: i32, month: u8, day: u8 },
    /// Proleptic Gregorian components could not be built (e.g. chrono / `time` interop).
    InvalidGregorianDate(GregorianDateError),
}

impl fmt::Display for ShahanshahiDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonthOutOfRange { month } => {
                write!(f, "month {month} is not in the range 1..=12 (Farvardin..=Esfand)")
            }
            Self::DayOutOfRange {
                year,
                month,
                day,
                max_day,
            } => write!(
                f,
                "day {day} is invalid for Shahanshahi {year}-{month:02} (max day {max_day})",
            ),
            Self::OutOfLegalEra {
                year,
                month,
                day,
            } => write!(
                f,
                "date {year}-{month:02}-{day:02} is outside the default legal Shahanshahi civil era \
                 (inclusive {sy}-{sm:02}-{sd:02} ..= {ey}-{em:02}-{ed:02}; see SPEC.md and crate docs)",
                sy = LEGAL_ERA_START_YEAR,
                sm = LEGAL_ERA_START_MONTH,
                sd = LEGAL_ERA_START_DAY,
                ey = LEGAL_ERA_END_YEAR,
                em = LEGAL_ERA_END_MONTH,
                ed = LEGAL_ERA_END_DAY,
            ),
            Self::InvalidGregorianDate(err) => {
                write!(f, "invalid Gregorian date from interop: {err}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ShahanshahiDateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidGregorianDate(err) => Some(err),
            _ => None,
        }
    }
}

impl From<GregorianDateError> for ShahanshahiDateError {
    fn from(err: GregorianDateError) -> Self {
        Self::InvalidGregorianDate(err)
    }
}

impl ShahanshahiDate {
    /// Constructs a date **within the default legal Shahanshahi civil era** only.
    pub fn try_new(year: i32, month: u8, day: u8) -> Result<Self, ShahanshahiDateError> {
        Self::validate_ymd(year, month, day)?;
        if !is_within_legal_era(year, month, day) {
            return Err(ShahanshahiDateError::OutOfLegalEra { year, month, day });
        }
        Ok(Self { year, month, day })
    }

    /// Like [`try_new`](Self::try_new), but allows any year that still fits the 1925 month grid and Mode A leap rule.
    ///
    /// Enable Cargo feature **`proleptic`**. Dates outside the legal era are conventional only (SPEC.md).
    #[cfg(feature = "proleptic")]
    pub fn try_new_proleptic(year: i32, month: u8, day: u8) -> Result<Self, ShahanshahiDateError> {
        Self::validate_ymd(year, month, day)?;
        Ok(Self { year, month, day })
    }

    fn validate_ymd(year: i32, month: u8, day: u8) -> Result<(), ShahanshahiDateError> {
        if !(1..=12).contains(&month) {
            return Err(ShahanshahiDateError::MonthOutOfRange { month });
        }
        let max_day = days_in_month(year, month);
        if !(1..=max_day).contains(&day) {
            return Err(ShahanshahiDateError::DayOutOfRange {
                year,
                month,
                day,
                max_day,
            });
        }
        Ok(())
    }

    #[inline]
    pub const fn year(self) -> i32 {
        self.year
    }

    #[inline]
    pub const fn month(self) -> u8 {
        self.month
    }

    #[inline]
    pub const fn day(self) -> u8 {
        self.day
    }

    /// Converts this Shahanshahi civil date to the **proleptic Gregorian** anchor calendar (SPEC.md),
    /// using **Rata Die** as the internal scale (see `convert` module).
    ///
    /// This is **infallible** for any [`ShahanshahiDate`] (the type guarantees a valid YMD on the 1925 grid).
    #[inline]
    pub fn to_gregorian(self) -> GregorianDate {
        convert::shahanshahi_to_gregorian(self.year, self.month, self.day)
    }

    /// Parses a **proleptic Gregorian** civil date and returns the matching Shahanshahi YMD, **only if**
    /// it falls in the default **legal Shahanshahi civil era** (same window as [`try_new`](Self::try_new)).
    pub fn try_from_gregorian(date: GregorianDate) -> Result<Self, ShahanshahiDateError> {
        let (y, m, d) = convert::gregorian_to_shahanshahi_ymd(date);
        Self::try_new(y, m, d)
    }

    /// Like [`try_from_gregorian`](Self::try_from_gregorian), but accepts any Shahanshahi YMD consistent
    /// with the 1925 month grid and Mode A leap rule, without enforcing the legal era (SPEC.md § Proleptic use).
    ///
    /// Enable Cargo feature **`proleptic`**.
    #[cfg(feature = "proleptic")]
    pub fn try_from_gregorian_proleptic(date: GregorianDate) -> Result<Self, ShahanshahiDateError> {
        let (y, m, d) = convert::gregorian_to_shahanshahi_ymd(date);
        Self::try_new_proleptic(y, m, d)
    }
}

fn days_in_month(year: i32, month: u8) -> u8 {
    days_in_shahanshahi_month(year, month)
}

fn is_within_legal_era(year: i32, month: u8, day: u8) -> bool {
    let candidate = (year, i32::from(month), i32::from(day));
    let start = (
        LEGAL_ERA_START_YEAR,
        i32::from(LEGAL_ERA_START_MONTH),
        i32::from(LEGAL_ERA_START_DAY),
    );
    let end = (
        LEGAL_ERA_END_YEAR,
        i32::from(LEGAL_ERA_END_MONTH),
        i32::from(LEGAL_ERA_END_DAY),
    );
    candidate >= start && candidate <= end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn era_anchor_ok() {
        let d = ShahanshahiDate::try_new(2535, 1, 1).unwrap();
        assert_eq!((d.year(), d.month(), d.day()), (2535, 1, 1));
    }

    #[test]
    fn era_last_day_ok() {
        ShahanshahiDate::try_new(2537, 6, 10).unwrap();
    }

    #[test]
    fn month_out_of_range() {
        assert!(matches!(
            ShahanshahiDate::try_new(2535, 0, 1),
            Err(ShahanshahiDateError::MonthOutOfRange { .. })
        ));
        assert!(matches!(
            ShahanshahiDate::try_new(2535, 13, 1),
            Err(ShahanshahiDateError::MonthOutOfRange { .. })
        ));
    }

    #[test]
    fn day_out_of_range_mehr() {
        assert!(matches!(
            ShahanshahiDate::try_new(2535, 7, 31),
            Err(ShahanshahiDateError::DayOutOfRange { max_day: 30, .. })
        ));
    }

    #[test]
    fn esfand_29_in_common_year() {
        ShahanshahiDate::try_new(2535, 12, 29).unwrap();
        assert!(matches!(
            ShahanshahiDate::try_new(2535, 12, 30),
            Err(ShahanshahiDateError::DayOutOfRange { max_day: 29, .. })
        ));
    }

    #[cfg(feature = "proleptic")]
    #[test]
    fn proleptic_leap_esfand_30() {
        ShahanshahiDate::try_new_proleptic(2534, 12, 30).unwrap();
    }

    #[test]
    fn before_era_rejected() {
        assert!(matches!(
            ShahanshahiDate::try_new(2534, 12, 29),
            Err(ShahanshahiDateError::OutOfLegalEra { .. })
        ));
    }

    #[test]
    fn after_era_rejected() {
        assert!(matches!(
            ShahanshahiDate::try_new(2537, 6, 11),
            Err(ShahanshahiDateError::OutOfLegalEra { .. })
        ));
        assert!(matches!(
            ShahanshahiDate::try_new(2537, 7, 1),
            Err(ShahanshahiDateError::OutOfLegalEra { .. })
        ));
    }

    #[test]
    fn ordering() {
        let a = ShahanshahiDate::try_new(2535, 1, 1).unwrap();
        let b = ShahanshahiDate::try_new(2535, 12, 29).unwrap();
        assert!(a < b);
    }

    #[test]
    fn to_gregorian_anchor_matches_spec() {
        use crate::GregorianDate;
        let sh = ShahanshahiDate::try_new(2535, 1, 1).unwrap();
        let g = sh.to_gregorian();
        assert_eq!(GregorianDate::try_new(1976, 3, 21).unwrap(), g);
    }

    #[test]
    fn try_from_gregorian_round_trip_legal_era() {
        use crate::GregorianDate;
        let g = GregorianDate::try_new(1976, 7, 22).unwrap();
        let sh = ShahanshahiDate::try_from_gregorian(g).unwrap();
        assert_eq!((sh.year(), sh.month(), sh.day()), (2535, 4, 31));
        assert_eq!(sh.to_gregorian(), g);
    }

    #[test]
    fn try_from_gregorian_rejects_proleptic_only_date() {
        use crate::GregorianDate;
        let g = GregorianDate::try_new(1996, 3, 20).unwrap();
        assert!(matches!(
            ShahanshahiDate::try_from_gregorian(g),
            Err(ShahanshahiDateError::OutOfLegalEra { .. })
        ));
    }

    #[cfg(feature = "proleptic")]
    #[test]
    fn try_from_gregorian_proleptic_accepts_outside_legal_era() {
        use crate::GregorianDate;
        // Golden row: last day of 2554 (= 1374 SH, common in Mode A) ≡ 1996-03-19 Gregorian.
        let g = GregorianDate::try_new(1996, 3, 19).unwrap();
        let sh = ShahanshahiDate::try_from_gregorian_proleptic(g).unwrap();
        assert_eq!((sh.year(), sh.month(), sh.day()), (2554, 12, 29));
    }
}
