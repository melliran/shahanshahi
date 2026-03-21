# shahanshahi

[![rustfmt](https://github.com/melliran/shahanshahi/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/rustfmt.yml)
[![clippy](https://github.com/melliran/shahanshahi/actions/workflows/clippy.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/clippy.yml)
[![test](https://github.com/melliran/shahanshahi/actions/workflows/test.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/test.yml)
[![crate package](https://github.com/melliran/shahanshahi/actions/workflows/crate-package.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/crate-package.yml)
[![audit](https://github.com/melliran/shahanshahi/actions/workflows/audit.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/audit.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.74-31748f?logo=rust&logoColor=white)](https://github.com/melliran/shahanshahi/blob/main/Cargo.toml)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE-MIT)

Rust library for the **Shahanshahi (Imperial Iranian)** civil calendar — developed in the spirit of [Melliran](https://github.com/melliran): open Persian technology and shared digital infrastructure so teams and organizations can adopt **spec-backed** Persian calendar tooling without fragile one-offs.

**Why spec-first:** see [`docs/VISION.md`](./docs/VISION.md).

**How we build and version:** see [`docs/ENGINEERING.md`](./docs/ENGINEERING.md). Badges above map to CI: **rustfmt** (format), **clippy** (lint), **test** (workspace, all features), **crate package** (`cargo publish --dry-run` — release-ready packaging, not a deployed server), **audit** (`cargo audit` + `cargo deny`).

## What the library does today

The [`shahanshahi`](./crates/shahanshahi) crate implements **spec version 2** of [`SPEC.md`](./SPEC.md) for **deterministic, offline** use:

- **Civil dates:** [`ShahanshahiDate`](./crates/shahanshahi/src/date.rs) (1925 month grid, **Mode A** leap on the underlying Hijri Shamsi year) and [`GregorianDate`](./crates/shahanshahi/src/gregorian.rs) for **bidirectional** conversion via Rata Die.
- **Legal era by default:** construction rejects dates outside the documented Shahanshahi civil window; enable the **`proleptic`** Cargo feature for [`try_new_proleptic`](./crates/shahanshahi/src/date.rs) when you need the same grid without era enforcement (see SPEC.md).
- **Leap helpers:** public arithmetic aligned with SPEC (e.g. [`is_shahanshahi_leap_arithmetic`](./crates/shahanshahi/src/leap.rs)).
- **Golden tests:** [`data/reference-dates.json`](./data/reference-dates.json) holds vetted Shahanshahi ↔ Gregorian rows with citations; [`crates/shahanshahi/tests/reference_dates.rs`](./crates/shahanshahi/tests/reference_dates.rs) checks them in CI and asserts `spec_id` matches the crate’s [`SPEC_VERSION`](./crates/shahanshahi/src/lib.rs).

**Scope and non-goals** (summary): civil Shahanshahi as briefly enacted in Iran; **not** religious calendars, lunar Hijri, or locale strings beyond what the crate documents. **Mode B** (ephemeris) is specified but not implemented as runtime code in this crate yet.

**Pre-1.0:** the public API may still evolve; see [`docs/MIGRATING.md`](./docs/MIGRATING.md) and [`CHANGELOG.md`](./CHANGELOG.md). Crates.io package name: **`shahanshahi`** (when published). [v0.1.0 readiness](https://github.com/melliran/shahanshahi/issues/8) and [roadmap](https://github.com/melliran/shahanshahi/issues/9) are tracked on GitHub.

## Repository layout

| Path | Purpose |
|------|--------|
| `SPEC.md` | Authoritative rules this code implements |
| `data/reference-dates.json` | Vetted Y/M/D pairs + citations for tests |
| `docs/ENGINEERING.md` | How we build, version, and release the library |
| `CHANGELOG.md` | Release history (Keep a Changelog) |
| `docs/MIGRATING.md` | Notes when upgrading between published versions |
| `deny.toml` | `cargo-deny` policy (licenses, advisories, sources) |
| `release-plz.toml` | Release automation config ([release-plz](https://release-plz.dev/)) |
| `SECURITY.md` | How to report vulnerabilities privately |
| `crates/shahanshahi` | Library crate (API to grow with the spec) |
| `crates/shahanshahi/examples/` | Runnable examples (`cargo run -p shahanshahi --example …`) |

## Building

From the repository root:

```bash
cargo build -p shahanshahi
cargo test -p shahanshahi --all-features
```

### Examples

```bash
cargo run -p shahanshahi --example convert_legal_era
cargo run -p shahanshahi --example convert_proleptic --features proleptic
```

## Repository

[github.com/melliran/shahanshahi](https://github.com/melliran/shahanshahi)

## Roadmap

Tracked on GitHub: **[roadmap index — issue #9](https://github.com/melliran/shahanshahi/issues/9)** and milestone [**v0.1 — spec-backed MVP**](https://github.com/melliran/shahanshahi/milestone/1).

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## Security

To report a **security vulnerability**, use [GitHub Security](https://github.com/melliran/shahanshahi/security) — **do not** file a public issue. Details: [`SECURITY.md`](./SECURITY.md).

## License

Licensed under either of [Apache-2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT) at your option.
