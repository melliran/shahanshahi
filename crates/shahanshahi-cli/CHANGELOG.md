# Changelog

All notable changes to `shahanshahi-cli` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4](https://github.com/melliran/shahanshahi/compare/shahanshahi-cli-v0.1.3...shahanshahi-cli-v0.1.4) - 2026-08-02

### Other

- update Cargo.lock dependencies

## [0.1.3] - 2026-07-05

### Changed

- Bumped `shahanshahi` dependency to `v0.3.0`, picking up the new `Weekday` enum and `weekday()` methods on `ShahanshahiDate` and `GregorianDate`

## [0.1.2] - 2026-06-27

### Changed

- `convert::month_name` now delegates to `shahanshahi::shahanshahi_month_name()` from the library instead of maintaining its own table ([#96](https://github.com/melliran/shahanshahi/pull/96))
- Bumped `shahanshahi` dependency to `v0.2.1`

## [0.1.1] - 2026-06-05

### Added

- `README.md` for crates.io display

## [0.1.0] - 2026-06-04

First release of the `shahanshahi-cli` crate.

### Added

- Batch/pipe-friendly CLI binary (`shahanshahi` / `shcal`) for Shahanshahi ↔ Gregorian conversion ([#48](https://github.com/melliran/shahanshahi/pull/48), [#6](https://github.com/melliran/shahanshahi/issues/6))
- Human-readable Persian month names in text output ([#56](https://github.com/melliran/shahanshahi/pull/56))
- Shell completion scripts for bash, zsh, fish, and PowerShell (`shcal completions <shell>`) ([#59](https://github.com/melliran/shahanshahi/pull/59))
- `shcal` short binary alias alongside the `shahanshahi` binary ([#60](https://github.com/melliran/shahanshahi/pull/60))
