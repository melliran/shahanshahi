# Migrating

Notes for upgrading between **published** versions of the `shahanshahi` crate. Until **1.0.0**, minor releases in the **0.* line** may change or extend the public API; check [`CHANGELOG.md`](../CHANGELOG.md) for each release.

## From repository / `0.0.0` to first crates.io release (e.g. `0.1.0`)

`0.0.0` was a **workspace placeholder** on crates.io (if reserved) or pre-publish only; treat the first real release as the baseline **documented API** in rustdoc and [`SPEC.md`](../SPEC.md).

- **SemVer:** In **0.*.***, Cargo allows breaking changes between minor versions. Pin a specific version range that matches your risk tolerance.
- **Features:** Default builds enforce the **legal Shahanshahi era** on [`ShahanshahiDate`](../crates/shahanshahi/src/date.rs). Opt in to **`proleptic`** only if you need dates outside that window under the same civil grid. Optional **`serde`**, **`chrono`**, and **`time`** integrate without pulling into the default dependency graph; disable default features (`default-features = false`) for **`no_std`** builds (no `std::error::Error` on date errors in that mode).
- **Errors:** [`ShahanshahiDateError`](../crates/shahanshahi/src/date.rs) includes **`InvalidGregorianDate`** when interop cannot form a valid proleptic Gregorian YMD (e.g. invalid `chrono` / `time` inputs). Update exhaustive `match` arms if you match this enum structurally.
- **Spec alignment:** [`SPEC_VERSION`](../crates/shahanshahi/src/lib.rs) tracks the written spec; golden rows in [`data/reference-dates.json`](../data/reference-dates.json) must match that id for tests to pass.

For release automation and when to enable automated crates.io publish, see [Release process](./ENGINEERING.md#release-process-cratesio) and [v0.1.0 readiness](./ENGINEERING.md#v010-readiness).
