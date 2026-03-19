# shahanshahi

[![rustfmt](https://github.com/melliran/shahanshahi/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/rustfmt.yml)
[![clippy](https://github.com/melliran/shahanshahi/actions/workflows/clippy.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/clippy.yml)
[![test](https://github.com/melliran/shahanshahi/actions/workflows/test.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/test.yml)
[![crate package](https://github.com/melliran/shahanshahi/actions/workflows/crate-package.yml/badge.svg)](https://github.com/melliran/shahanshahi/actions/workflows/crate-package.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.74-31748f?logo=rust&logoColor=white)](https://github.com/melliran/shahanshahi/blob/main/Cargo.toml)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE-MIT)

Rust library for the **Shahanshahi (Imperial Iranian)** civil calendar — developed in the spirit of [Melliran](https://github.com/melliran): open Persian technology and shared digital infrastructure so teams and organizations can adopt **spec-backed** Persian calendar tooling without fragile one-offs.

**Why spec-first:** see [`docs/VISION.md`](./docs/VISION.md).

**How we build and version:** see [`docs/ENGINEERING.md`](./docs/ENGINEERING.md). Badges above map to CI: **rustfmt** (format), **clippy** (lint), **test**, **crate package** (`cargo publish --dry-run` — release-ready packaging, not a deployed server).

**Status: pre-implementation.** The crate publishes a workspace skeleton only. Arithmetic and conversions ship after [`SPEC.md`](./SPEC.md) and golden dates in [`data/reference-dates.json`](./data/reference-dates.json) are filled from primary sources.

## Repository layout

| Path | Purpose |
|------|--------|
| `SPEC.md` | Authoritative rules this code implements |
| `data/reference-dates.json` | Vetted Y/M/D pairs + citations for tests |
| `docs/ENGINEERING.md` | How we build, version, and release the library |
| `CHANGELOG.md` | Release history (Keep a Changelog) |
| `SECURITY.md` | How to report vulnerabilities privately |
| `crates/shahanshahi` | Library crate (API to grow with the spec) |

## Building

From the repository root:

```bash
cargo build -p shahanshahi
cargo test -p shahanshahi
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
