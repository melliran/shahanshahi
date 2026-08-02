# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
as described in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md).

## [Unreleased]

## [0.4.0] - 2026-08-02

### Added

- [`Locale`](./crates/shahanshahi/src/format.rs) enum (`Locale::En`, `Locale::Fa`) — controls whether format output uses ASCII romanized names and Western digits, or Persian script names and Persian numerals (U+06F0–U+06F9) ([#117](https://github.com/melliran/shahanshahi/pull/117))
- [`ShahanshahiDate::format(fmt)`](./crates/shahanshahi/src/date.rs) — strftime-style formatting with specifiers `%Y`, `%m`, `%d`, `%B` (full month name), `%b` (short month name), `%A` (weekday name), `%%`; defaults to `Locale::En` ([#117](https://github.com/melliran/shahanshahi/pull/117))
- [`ShahanshahiDate::format_localized(fmt, locale)`](./crates/shahanshahi/src/date.rs) — same as `format()` but accepts a `Locale`; `Locale::Fa` outputs Persian script names and converts all digits to Persian numerals ([#117](https://github.com/melliran/shahanshahi/pull/117))
- [`to_persian_numerals(s)`](./crates/shahanshahi/src/format.rs) — public utility that replaces every ASCII digit in a string with its Persian numeral equivalent ([#117](https://github.com/melliran/shahanshahi/pull/117))

## [0.3.0] - 2026-07-05

### Added

- [`Weekday`](./crates/shahanshahi/src/weekday.rs) enum (Saturday-first, matching the Iranian civil week) with `name_ascii()` (e.g. `"Shanbeh"`), `name_persian()` (e.g. `"شنبه"`), `number_from_saturday()` (0 = Saturday … 6 = Friday), and `impl Display` ([#102](https://github.com/melliran/shahanshahi/pull/102))
- [`ShahanshahiDate::weekday()`](./crates/shahanshahi/src/date.rs) — returns the day of the week for a Shahanshahi date; derived from the Rata Die already computed during conversion, so no extra algorithm cost ([#102](https://github.com/melliran/shahanshahi/pull/102))
- [`GregorianDate::weekday()`](./crates/shahanshahi/src/gregorian.rs) — same for Gregorian dates ([#102](https://github.com/melliran/shahanshahi/pull/102))

## [0.2.1] - 2026-06-27

### Added

- `impl std::fmt::Display` for [`ShahanshahiDate`] and [`GregorianDate`]: formats as `YYYY-MM-DD` (e.g. `2535-01-01`, `1976-03-21`); negative years format as `-YYYY-MM-DD`. Unlocks `.to_string()` and `format!("{}", date)` ([#93](https://github.com/melliran/shahanshahi/pull/93))
- [`shahanshahi_month_name(month)`](./crates/shahanshahi/src/months.rs) — ASCII romanization of the Shahanshahi month name for a 1-based index (1 = `"Farvardin"` … 12 = `"Esfand"`), returns `Option<&'static str>` ([#95](https://github.com/melliran/shahanshahi/pull/95))
- [`shahanshahi_month_name_persian(month)`](./crates/shahanshahi/src/months.rs) — Persian script month name (1 = `"فروردین"` … 12 = `"اسفند"`), returns `Option<&'static str>` ([#95](https://github.com/melliran/shahanshahi/pull/95))

## [0.2.0] - 2026-03-22

### Added

- Optional Cargo features **`serde`**, **`chrono`**, and **`time`**: `Serialize`/`Deserialize` on civil date types; `NaiveDate` / `time::Date` conversion helpers; default **`std`** feature with `default-features = false` for `#![no_std]` builds ([issue #5](https://github.com/melliran/shahanshahi/issues/5))
- [`ShahanshahiDateError::InvalidGregorianDate`](./crates/shahanshahi/src/date.rs) and `From<GregorianDateError>` for interop validation failures
- [`ChronoNaiveDateOutOfRange`](./crates/shahanshahi/src/chrono_compat.rs) when a valid [`GregorianDate`](./crates/shahanshahi/src/gregorian.rs) lies outside `chrono::NaiveDate`’s year range

### Changed

- **`chrono` feature:** [`GregorianDate::to_chrono_naive_date`](./crates/shahanshahi/src/chrono_compat.rs) and [`ShahanshahiDate::to_chrono_naive_date`](./crates/shahanshahi/src/chrono_compat.rs) return `Result<_, ChronoNaiveDateOutOfRange>` instead of panicking for out-of-range years ([#38](https://github.com/melliran/shahanshahi/pull/38))

## [0.1.0] - 2026-03-21

First crates.io release: spec-backed Shahanshahi civil dates (legal era by default), Gregorian conversion (Mode A), golden tests, and runnable examples.

### Added

- [`crates/shahanshahi/examples/`](./crates/shahanshahi/examples/) — `convert_legal_era` and `convert_proleptic` (`--features proleptic`) runnable examples; CI builds all examples with `--all-features` ([issue #32](https://github.com/melliran/shahanshahi/issues/32))
- [`docs/MIGRATING.md`](./docs/MIGRATING.md) — upgrade notes for first publish and **0.* semver**; [`docs/ENGINEERING.md`](./docs/ENGINEERING.md) — v0.1.0 readiness checklist and **`RELEASE_PLZ_PUBLISH`** guidance ([issue #8](https://github.com/melliran/shahanshahi/issues/8))
- [`README.md`](./README.md) — current library scope, legal era vs **`proleptic`**, golden tests, and links to migration / roadmap
- [`crates/shahanshahi/src/date.rs`](./crates/shahanshahi/src/date.rs) — [`ShahanshahiDate`](./crates/shahanshahi/src/date.rs) + [`ShahanshahiDateError`](./crates/shahanshahi/src/date.rs): civil Y/M/D validation (1925 month grid, Mode A leap), default **legal era** bounds per SPEC.md, optional **`proleptic`** feature for [`try_new_proleptic`](./crates/shahanshahi/src/date.rs) ([issue #3](https://github.com/melliran/shahanshahi/issues/3))
- [`crates/shahanshahi/src/leap.rs`](./crates/shahanshahi/src/leap.rs) — public Mode A helpers (`is_solar_hijri_leap_arithmetic`, `shahanshahi_to_hijri_shamsi_year`, `is_shahanshahi_leap_arithmetic`) matching SPEC.md
- [`data/reference-dates.json`](./data/reference-dates.json) — golden Shahanshahi ↔ Gregorian rows with **Wikipedia *Solar Hijri calendar*** comparison table + **IAS 1925 law** transcription + derived month-grid dates ([issue #2](https://github.com/melliran/shahanshahi/issues/2))
- [`crates/shahanshahi/tests/reference_dates.rs`](./crates/shahanshahi/tests/reference_dates.rs) — loads corpus, checks `spec_id` vs [`SPEC_VERSION`](./crates/shahanshahi/src/lib.rs)

### Changed

- [`SPEC.md`](./SPEC.md) — **spec version 2**: astronomical references (Heydari‑Malayeri, Akrami, Wikipedia *Equinox*), operational Nowruz model, **Mode A** (33‑year arithmetic + Rust) and **Mode B** (JPL ephemeris + illustrative Rust); [`SPEC_VERSION`](./crates/shahanshahi/src/lib.rs) and [`data/reference-dates.json`](./data/reference-dates.json) `spec_id` → **`2`**

## [0.0.0] - 2026-03-19

### Added

- Cargo workspace and `shahanshahi` library skeleton
- README, [`CONTRIBUTING.md`](./CONTRIBUTING.md), and [`docs/VISION.md`](./docs/VISION.md); README links to roadmap issue and v0.1 milestone
- [`docs/ENGINEERING.md`](./docs/ENGINEERING.md) — build, versioning, CI, releases, and automation
- [`SECURITY.md`](./SECURITY.md) and GitHub issue / PR templates
- CI: `rustfmt`, `clippy`, `test`, and `crate package` workflows; README badges
- `cargo audit` and `cargo deny` ([`deny.toml`](./deny.toml)); committed [`Cargo.lock`](./Cargo.lock) for deterministic scans
- Dependabot (Cargo + Actions) and path-based PR labeler ([`.github/labeler.yml`](./.github/labeler.yml))
- [`release-plz.toml`](./release-plz.toml) and release-plz workflow (draft release PRs; gated publish via `RELEASE_PLZ_PUBLISH`; optional `RELEASE_PLZ_GITHUB_TOKEN`)
