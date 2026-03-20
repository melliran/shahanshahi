# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
as described in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md).

## [Unreleased]

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
