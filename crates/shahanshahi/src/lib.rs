//! Shahanshahi (Imperial Iranian) calendar.
//!
//! This crate is **spec-first**: [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md)
//! defines calendar rules (`SPEC_VERSION`). Arithmetic and conversions ship once
//! golden rows in `data/reference-dates.json` are populated and tested; public
//! APIs may still evolve before `0.1.0`.

#![forbid(unsafe_code)]

/// Tracks which written specification version this build targets.
pub const SPEC_VERSION: &str = "1";
