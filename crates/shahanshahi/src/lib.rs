//! Shahanshahi (Imperial Iranian) calendar.
//!
//! This crate is **spec-first**: arithmetic and conversions are implemented only
//! after [`SPEC.md`](https://github.com/melliran/shahanshahi/blob/main/SPEC.md)
//! and the golden dates under `data/` are locked. Until then, APIs may change
//! without notice.

#![forbid(unsafe_code)]

/// Tracks which written specification version this build targets.
pub const SPEC_VERSION: &str = "0-unstable";
