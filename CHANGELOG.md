# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
as described in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md).

## [Unreleased]

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
