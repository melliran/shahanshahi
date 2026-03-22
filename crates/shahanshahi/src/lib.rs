//! Shahanshahi (Imperial Iranian) calendar.
//!
//! This crate is **spec-first**: [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md)
//! defines calendar rules ([`SPEC_VERSION`]). Golden rows in `data/reference-dates.json` anchor tests;
//! public APIs may still evolve before `1.0.0` (see SemVer policy in the repo changelog).
//!
//! # Types
//!
//! - [`ShahanshahiDate`] — civil **year / month / day** with 1925 month lengths and **Mode A** leap
//!   (33-year arithmetic on the underlying Hijri Shamsi year). By default, only dates inside the
//!   documented **legal Shahanshahi civil era** are accepted; see that type’s docs for bounds and errors.
//! - [`GregorianDate`] — proleptic Gregorian anchor calendar for **bidirectional** conversion via
//!   **Rata Die** ([`ShahanshahiDate::to_gregorian`], [`ShahanshahiDate::try_from_gregorian`]).
//!
//! # Features
//!
//! - **`std`** (default) — [`std::error::Error`] for date errors; disable with `default-features = false`
//!   for `#![no_std]` builds (core-only algorithms and types).
//! - **`proleptic`** — [`ShahanshahiDate::try_new_proleptic`] validates the same calendar grid without
//!   enforcing the legal era (SPEC.md § Proleptic use).
//! - **`serde`** — `serde` `Serialize` / `Deserialize` on [`ShahanshahiDate`] and [`GregorianDate`]
//!   (optional dependency with `default-features = false`).
//! - **`chrono`** — `chrono::NaiveDate` conversions on [`GregorianDate`] and [`ShahanshahiDate`]
//!   (implies **`std`**).
//! - **`time`** — `time::Date` conversions on [`GregorianDate`] and [`ShahanshahiDate`] (implies **`std`**).

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

mod convert;
mod date;
mod gregorian;
mod leap;
mod rata_die;

#[cfg(feature = "chrono")]
mod chrono_compat;
#[cfg(feature = "chrono")]
pub use chrono_compat::ChronoNaiveDateOutOfRange;
#[cfg(feature = "time")]
mod time_compat;

pub use date::{ShahanshahiDate, ShahanshahiDateError};
pub use gregorian::{GregorianDate, GregorianDateError};
pub use leap::{
    is_shahanshahi_leap_arithmetic, is_solar_hijri_leap_arithmetic,
    shahanshahi_to_hijri_shamsi_year,
};

/// Tracks which written specification version this build targets.
///
/// Keep in sync with the **Document version** line at the top of [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md).
pub const SPEC_VERSION: &str = "2";
