//! Optional [`chrono`](https://docs.rs/chrono) interop (Cargo feature **`chrono`**).

use crate::GregorianDate;
use crate::GregorianDateError;
use crate::ShahanshahiDate;
use crate::ShahanshahiDateError;
use ::chrono::{Datelike, NaiveDate};

impl GregorianDate {
    /// Parses a [`NaiveDate`](chrono::NaiveDate) civil YMD using the same rules as [`try_new`](GregorianDate::try_new).
    pub fn try_from_chrono_naive_date(date: NaiveDate) -> Result<Self, GregorianDateError> {
        GregorianDate::try_new(date.year(), date.month() as u8, date.day() as u8)
    }

    /// Maps this Gregorian date to `chrono`’s plain civil type (always succeeds for valid [`GregorianDate`]).
    pub fn to_chrono_naive_date(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year(), u32::from(self.month()), u32::from(self.day()))
            .expect("GregorianDate always maps to a valid chrono NaiveDate")
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
    pub fn to_chrono_naive_date(self) -> NaiveDate {
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
