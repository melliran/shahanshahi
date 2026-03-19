# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
as described in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md).

## [Unreleased]

### Added

- Release automation: [release-plz](https://release-plz.dev/) workflow (draft release PRs; optional gated publish via repo variable `RELEASE_PLZ_PUBLISH`) and [`release-plz.toml`](./release-plz.toml); optional secret `RELEASE_PLZ_GITHUB_TOKEN` if the repo cannot allow Actions to open PRs.
- CI security: `cargo audit` and `cargo deny check` ([`audit` workflow](.github/workflows/audit.yml)), root [`deny.toml`](./deny.toml), and committed [`Cargo.lock`](./Cargo.lock) for deterministic scans.
- GitHub automation: Dependabot (Cargo + Actions) and path-based PR labeler (see [`docs/ENGINEERING.md`](./docs/ENGINEERING.md)).
- Security policy and reporting process ([`SECURITY.md`](./SECURITY.md)).
- Engineering and versioning policy ([`docs/ENGINEERING.md`](./docs/ENGINEERING.md)).
- Split CI into `rustfmt`, `clippy`, `test`, and `crate package` (publish dry-run) workflows.
