# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Shahanshahi** is a Rust library and CLI for deterministic offline conversion between the **Shahanshahi (Imperial Iranian) civil calendar** (in use ~1976–1979) and the proleptic Gregorian calendar. Behavior is governed by `SPEC.md` (authoritative, version-pinned) and validated against a vetted golden corpus in `data/reference-dates.json`.

## Workspace Layout

Two crates:
- `crates/shahanshahi` — the core library (v0.2.0)
- `crates/shahanshahi-cli` — batch/pipe-friendly CLI binary (v0.1.0)

## Common Commands

```bash
# Build
cargo build --workspace --all-features

# Test everything
cargo test --workspace --all-features

# Run a single test
cargo test -p shahanshahi <test_name>
cargo test -p shahanshahi <test_name> -- --nocapture

# Format
cargo fmt --all

# Lint (CI gate: no warnings allowed)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Security audit
cargo audit
cargo deny check
```

CI merge gates: `cargo fmt --all -- --check`, `cargo clippy ... -D warnings`, `cargo test --workspace --all-features`.

## Architecture

### Conversion model

All conversions flow through a shared **Rata Die** (integer day count). The anchor is `1 Farvardin 2535 SH = 1976-03-21 Gregorian = RD 721434`. From there, year walks use the 33-year arithmetic leap rule (residues: 1, 5, 9, 13, 17, 22, 26, 30 mod 33 on the underlying Hijri Shamsi year, i.e. `Y_SH - 1180`).

```
GregorianDate ←→ RataDay ←→ ShahanshahiDate
```

Key modules in `crates/shahanshahi/src/`:
- `date.rs` — `ShahanshahiDate` (year/month/day, legal era bounds enforced by default)
- `gregorian.rs` — `GregorianDate`
- `rata_die.rs` — Gregorian ↔ JDN via the standard 153-term identity
- `convert.rs` — bidirectional conversion via the Rata Die anchor
- `leap.rs` — `is_shahanshahi_leap_arithmetic()`, `shahanshahi_to_hijri_shamsi_year()` (offset: `Y_S - 1180`)

### Feature flags

| Feature | Effect |
|---------|--------|
| `std` (default) | Enables `std::error::Error`; disable for `no_std` |
| `proleptic` | Unlocks `try_new_proleptic()` — same month grid, no era bounds |
| `serde` | Serialize/deserialize `ShahanshahiDate`, `GregorianDate` |
| `chrono`, `time` | Ecosystem interop (imply `std`) |

### Spec compliance

The library implements **Mode A only** (arithmetic 33-year rule). Mode B (ephemeris-based equinox check) is spec'd in `SPEC.md` but not yet implemented. `SPEC_VERSION = "2"` is embedded in `lib.rs`; the golden corpus (`data/reference-dates.json`) carries a matching `spec_id` field that the `reference_dates` test verifies.

### CLI crate

`crates/shahanshahi-cli/src/` modules: `cli.rs` (clap structs), `convert.rs` (parse + call library), `format.rs` (text/JSON/CSV output, batch stdin). See `docs/CLI.md` for usage reference.

### Invariants to preserve

- `unsafe_code = "forbid"` is a crate-level lint — no unsafe Rust.
- The golden corpus in `data/reference-dates.json` is the ground truth for all conversion correctness claims; changes to conversion logic must be validated against it.
- Breaking spec behavior requires a `SPEC.md` version bump and corresponding `SPEC_VERSION` update in `lib.rs`.

## Versioning & Release

Library and CLI are versioned independently. Releases are automated via [release-plz](https://release-plz.dev/): CI drafts a release PR (version bump + changelog), which is merged manually to trigger `cargo publish`. Library tags: `vX.Y.Z`; CLI tags: `shahanshahi-cli-vX.Y.Z`. See `docs/ENGINEERING.md` for the full release workflow.
