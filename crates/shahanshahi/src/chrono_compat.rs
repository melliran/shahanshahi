//! Optional [`chrono`](https://docs.rs/chrono) interop (Cargo feature **`chrono`**).

use crate::GregorianDate;
use crate::GregorianDateError;
use crate::ShahanshahiDate;
use crate::ShahanshahiDateError;
use ::chrono::{Datelike, NaiveDate};
use core::fmt;

/// The Gregorian civil date is valid for [`GregorianDate`] but cannot be represented as a
/// [`chrono::NaiveDate`](chrono::NaiveDate) (chrono limits the representable year range).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChronoNaiveDateOutOfRange;

impl fmt::Display for ChronoNaiveDateOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gregorian date is outside the year range supported by chrono::NaiveDate",
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChronoNaiveDateOutOfRange {}

impl GregorianDate {
    /// Parses a [`NaiveDate`](chrono::NaiveDate) civil YMD using the same rules as [`try_new`](GregorianDate::try_new).
    pub fn try_from_chrono_naive_date(date: NaiveDate) -> Result<Self, GregorianDateError> {
        GregorianDate::try_new(date.year(), date.month() as u8, date.day() as u8)
    }

    /// Maps this Gregorian date to `chrono`’s plain civil type.
    ///
    /// Returns [`Err`] when the year is outside the range supported by `chrono::NaiveDate` (even
    /// though the same triple is valid for [`GregorianDate`]).
    pub fn to_chrono_naive_date(self) -> Result<NaiveDate, ChronoNaiveDateOutOfRange> {
        NaiveDate::from_ymd_opt(self.year(), u32::from(self.month()), u32::from(self.day()))
            .ok_or(ChronoNaiveDateOutOfRange)
    }
}

impl ShahanshahiDate {
    /// Converts through [`GregorianDate`] and [`try_from_gregorian`](ShahanshahiDate::try_from_gregorian)
    /// (default **legal era** policy).
    pub fn try_from_chrono_naive_date(date: NaiveDate) -> Result<Self, ShahanshahiDateError> {
        let g = GregorianDate::try_from_chrono_naive_date(date)?;
        Self::try_from_gregorian(g)
    }

    /// [`to_gregorian`](ShahanshahiDate::to_gregorian) then [`GregorianDate::to_chrono_naive_date`].
    pub fn to_chrono_naive_date(self) -> Result<NaiveDate, ChronoNaiveDateOutOfRange> {
        self.to_gregorian().to_chrono_naive_date()
    }

    /// Like [`try_from_chrono_naive_date`](Self::try_from_chrono_naive_date), using the proleptic
    /// Shahanshahi policy. Requires Cargo feature **`proleptic`** in addition to **`chrono`**.
    #[cfg(feature = "proleptic")]
    pub fn try_from_chrono_naive_date_proleptic(
        date: NaiveDate,
    ) -> Result<Self, ShahanshahiDateError> {
        let g = GregorianDate::try_from_chrono_naive_date(date)?;
        Self::try_from_gregorian_proleptic(g)
    }
}
