//! Proleptic **Gregorian** civil date — anchor calendar for Shahanshahi conversions (SPEC.md).

use core::fmt;

/// A valid **proleptic Gregorian** civil date (`year`, `month` 1..=12, `day` consistent with month/leap rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GregorianDate {
    year: i32,
    month: u8,
    day: u8,
}

/// Why a [`GregorianDate`] could not be constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GregorianDateError {
    /// `month` is not in `1..=12`.
    MonthOutOfRange { month: u8 },
    /// `day` is not valid for this year and month (including February in leap years).
    DayOutOfRange {
        year: i32,
        month: u8,
        day: u8,
        max_day: u8,
    },
}

impl fmt::Display for GregorianDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonthOutOfRange { month } => {
                write!(
                    f,
                    "month {month} is not in the range 1..=12 for Gregorian dates"
                )
            }
            Self::DayOutOfRange {
                year,
                month,
                day,
                max_day,
            } => write!(
                f,
                "day {day} is invalid for Gregorian {year}-{month:02} (max day {max_day})",
            ),
        }
    }
}

impl std::error::Error for GregorianDateError {}

impl GregorianDate {
    /// Constructs a Gregorian date, rejecting impossible month/day combinations (proleptic calendar).
    pub fn try_new(year: i32, month: u8, day: u8) -> Result<Self, GregorianDateError> {
        if !(1..=12).contains(&month) {
            return Err(GregorianDateError::MonthOutOfRange { month });
        }
        let max_day = days_in_gregorian_month(year, month);
        if !(1..=max_day).contains(&day) {
            return Err(GregorianDateError::DayOutOfRange {
                year,
                month,
                day,
                max_day,
            });
        }
        Ok(Self { year, month, day })
    }

    /// Used when the YMD is already produced by a trusted inverse (`rata_die` → Gregorian).
    #[inline]
    pub(crate) const fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
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
}

#[inline]
fn is_gregorian_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_gregorian_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_gregorian_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feb_29_on_leap() {
        GregorianDate::try_new(2000, 2, 29).unwrap();
    }

    #[test]
    fn feb_29_common_year_rejected() {
        assert!(matches!(
            GregorianDate::try_new(1900, 2, 29),
            Err(GregorianDateError::DayOutOfRange { max_day: 28, .. })
        ));
    }
}
