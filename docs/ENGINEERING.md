# Engineering, quality gates, and versioning

This document describes how we build and ship **shahanshahi** as a Rust library. It complements [`VISION.md`](./VISION.md) (why the project exists) and [`CONTRIBUTING.md`](../CONTRIBUTING.md) (day-to-day contribution rules).

## What we are building

- **Shape:** A **Rust library crate** (`crates/shahanshahi`) consumed via Cargo / crates.io — not a hosted “production service” with its own runtime SLA.
- **Approach:** **Spec-first.** Behavior is defined in [`SPEC.md`](../SPEC.md) and checked against vetted data in [`data/reference-dates.json`](../data/reference-dates.json). Code follows the spec; the spec changes when primary sources justify it.
- **Safety / style:** `unsafe` is forbidden at the crate level (`[lints.rust] unsafe_code = "forbid"`). Formatting and Clippy are **merge gates** (see below).

“**Production**” here means **dependable published artifacts**: a semver version on crates.io that downstream projects can pin, plus a clear record of what changed.

## Quality gates (CI)

GitHub Actions run on every push and pull request to `main`:

| Workflow | What it proves |
|----------|----------------|
| [rustfmt](../.github/workflows/rustfmt.yml) | `cargo fmt --check` — consistent formatting. |
| [clippy](../.github/workflows/clippy.yml) | `cargo clippy … -D warnings` — no Clippy warnings. |
| [test](../.github/workflows/test.yml) | `cargo test --workspace` — tests pass. |
| [crate package](../.github/workflows/crate-package.yml) | `cargo publish -p shahanshahi --dry-run` — the crate **packages and builds** as crates.io would. |

Locally, match CI before opening a PR: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

**Future hooks (optional, not required today):** `cargo deny`, `cargo semver-checks` after 1.0, MSRV matrix jobs, or `minimal-versions` builds — add them when the API and dependency graph justify the cost.

## Versioning rules

We follow **[Semantic Versioning 2.0.0](https://semver.org/)** as interpreted by the **Rust / Cargo ecosystem**:

| Version | Meaning for this crate |
|---------|-------------------------|
| **0.*.* (current)** | **API may break** between minor releases while the spec and public surface are still settling. Prefer documenting notable breaks in [`CHANGELOG.md`](../CHANGELOG.md). Patches are for fixes and non-breaking additions when possible. |
| **1.0.0 (future)** | **Stable API** commitment: follow SemVer strictly for the public API. Breaking changes require a new **major** version. |

**Source of truth:** The workspace version in the root [`Cargo.toml`](../Cargo.toml) (`[workspace.package] version`) is the crate version. **Do not** diverge the published version from that field.

**MSRV:** `rust-version` in `Cargo.toml` is the **minimum supported Rust version**. **Raising MSRV** is a **semver-visible** change: treat it as at least a **minor** bump in `0.x` (and a **minor** bump post-1.0), and record it in the changelog.

**Git tags:** When publishing to crates.io, use an **annotated tag** `vX.Y.Z` whose numbers **match** `Cargo.toml` at that commit (e.g. `v0.1.0` ↔ `version = "0.1.0"`).

## Release process (crates.io)

When maintainers are ready to publish (not yet required while the API is a skeleton):

1. Land all changes on `main` with CI green.
2. Update [`CHANGELOG.md`](../CHANGELOG.md) (move items under `[Unreleased]` into a dated `X.Y.Z` section).
3. Bump `version` in the root `Cargo.toml`.
4. Tag `vX.Y.Z` and push the tag.
5. Run `cargo publish -p shahanshahi` (maintainer credentials).
6. Create a **GitHub Release** (same tag) with the changelog section as the description.

Pre-release tags (`0.2.0-alpha.1`) are allowed if we need testers before a stable `0.y.z`.

## Summary

| Topic | Rule |
|-------|------|
| Spec vs code | Spec + golden dates lead; code implements. |
| CI | rustfmt, clippy (`-D warnings`), test, packaging dry-run on each PR. |
| Version | Root `Cargo.toml` `version`; SemVer; `0.x` allows API evolution with changelog discipline. |
| MSRV | Documented in `Cargo.toml`; bump ⇒ at least minor semver bump. |
| Tags | `vX.Y.Z` matches crate version at release. |

Questions belong in GitHub issues (see [issue templates](../.github/ISSUE_TEMPLATE/)).
