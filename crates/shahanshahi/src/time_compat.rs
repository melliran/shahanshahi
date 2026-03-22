//! Optional [`time`](https://docs.rs/time) interop (Cargo feature **`time`**).

use crate::GregorianDate;
use crate::GregorianDateError;
use crate::ShahanshahiDate;
use crate::ShahanshahiDateError;
use ::time::{Date, Month};

impl GregorianDate {
    /// Parses a [`time::Date`] using the same rules as [`try_new`](GregorianDate::try_new).
    pub fn try_from_time_date(date: Date) -> Result<Self, GregorianDateError> {
        GregorianDate::try_new(date.year(), date.month() as u8, date.day())
    }

    /// Maps this Gregorian date to `time`’s civil [`Date`] (always succeeds for valid [`GregorianDate`]).
    pub fn to_time_date(self) -> Date {
        let month = Month::try_from(self.month()).expect("GregorianDate month is always 1..=12");
        Date::from_calendar_date(self.year(), month, self.day())
            .expect("GregorianDate always maps to a valid time::Date")
    }
}

impl ShahanshahiDate {
    /// Converts through [`GregorianDate`] and [`try_from_gregorian`](ShahanshahiDate::try_from_gregorian).
    pub fn try_from_time_date(date: Date) -> Result<Self, ShahanshahiDateError> {
        let g = GregorianDate::try_from_time_date(date)?;
        Self::try_from_gregorian(g)
    }

    /// [`to_gregorian`](ShahanshahiDate::to_gregorian) then [`GregorianDate::to_time_date`].
    pub fn to_time_date(self) -> Date {
        self.to_gregorian().to_time_date()
    }

    /// Like [`try_from_time_date`](Self::try_from_time_date), using proleptic Shahanshahi policy.
    /// Requires Cargo feature **`proleptic`** in addition to **`time`**.
    #[cfg(feature = "proleptic")]
    pub fn try_from_time_date_proleptic(date: Date) -> Result<Self, ShahanshahiDateError> {
        let g = GregorianDate::try_from_time_date(date)?;
        Self::try_from_gregorian_proleptic(g)
    }
}
