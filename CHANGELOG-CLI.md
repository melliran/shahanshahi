# Changelog — shahanshahi-cli

All notable changes to the **`shahanshahi-cli`** crate (the `shahanshahi` binary) are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
for the **CLI contract** (flags, stdin/stdout formats), as described in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md).

The library crate [`shahanshahi`](./crates/shahanshahi) uses [`CHANGELOG.md`](./CHANGELOG.md).

## [Unreleased]

### Added

- `shahanshahi convert` with text, JSON, and CSV I/O; `--from` / `--to`; `--proleptic` ([`docs/CLI.md`](./docs/CLI.md), [#6](https://github.com/melliran/shahanshahi/issues/6)).
