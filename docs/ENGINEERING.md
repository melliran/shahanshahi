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
| [audit](../.github/workflows/audit.yml) | **`cargo audit`** (RustSec) + **`cargo deny check`** (advisories, licenses, sources) on every PR and weekly on a schedule. |
| [release-plz](../.github/workflows/release-plz.yml) | On push to **`main`**: maintain a **draft release PR** (version + changelog). **Publish** job runs only if repo variable **`RELEASE_PLZ_PUBLISH`** is `true` and **`CARGO_REGISTRY_TOKEN`** is set. |

**`Cargo.lock`** is committed at the workspace root so CI and security scans are **deterministic**. Refresh it when dependencies change (`cargo update` as appropriate).

Locally, match CI before opening a PR: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and when you touch dependencies or lockfile: `cargo audit` and `cargo deny check` (install via [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) or `cargo install`).

**Future hooks (optional):** `cargo semver-checks` after 1.0, MSRV matrix jobs, or `minimal-versions` builds — add when the API graph justifies the cost.

## Automation (GitHub)

Lightweight automation common in open-source Rust repos:

| Mechanism | File | Purpose |
|-----------|------|--------|
| **Dependabot** | [`.github/dependabot.yml`](../.github/dependabot.yml) | Opens PRs to update **Cargo** dependencies (weekly) and **GitHub Actions** pins (monthly, grouped). PRs are tagged with `chore`. |
| **PR labeler** | [`.github/workflows/labeler.yml`](../.github/workflows/labeler.yml) + [`.github/labeler.yml`](../.github/labeler.yml) | Applies **path-based** labels (`documentation`, `spec`, `chore`, `tests`) so triage and filters stay easy. |
| **Release-plz** | [`.github/workflows/release-plz.yml`](../.github/workflows/release-plz.yml) + [`release-plz.toml`](../release-plz.toml) | See [Release process](#release-process-cratesio) below. |

The labeler uses [`pull_request_target`](https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#pull_request_target) so it can label PRs from **forks**; the job only adjusts labels (no checkout of untrusted code). The **first** PR that introduces this workflow will not self-label — that is expected.

**Optional later:** issue/stale bots, `CODEOWNERS` once owners are fixed, or welcome comments for first-time contributors — add when noise vs. value is acceptable for the team.

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

We use **[release-plz](https://release-plz.dev/)** so versioning and changelog updates are **proposed as a PR** and **publishing** is a separate, **gated** step.

### One-time GitHub setup

1. **Actions → General → Workflow permissions:** allow **read and write** and “Allow GitHub Actions to create and approve pull requests” (see [release-plz quickstart](https://release-plz.dev/docs/github/quickstart)).
2. **Secrets:** add **`CARGO_REGISTRY_TOKEN`** ([crates.io token](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish) with scopes `publish-new` and `publish-update`). The publish job reads it only when enabled below.
3. **Variables:** add repository variable **`RELEASE_PLZ_PUBLISH`** = `true` only when you want CI to run **`release-plz release`** (crates.io + GitHub Release). Leave unset or not `true` to **skip publishing** while still opening release PRs.

### Day-to-day flow

1. Merge ordinary work to `main` (Conventional Commit–style messages help release-plz infer semver bumps).
2. **Release-plz** opens or updates a **draft** PR (branch prefix `release-plz-`, label `chore`) with **`Cargo.toml` version** and [`CHANGELOG.md`](../CHANGELOG.md) edits — see [`release-plz.toml`](../release-plz.toml).
3. **Review** that PR (spec readiness, version level, changelog text). Mark it **ready for review** and **merge** when satisfied.
4. **Publishing:**
   - **Automated:** with `RELEASE_PLZ_PUBLISH=true` and a valid `CARGO_REGISTRY_TOKEN`, the **`release-plz-release`** job on `main` runs [`release-plz release`](https://release-plz.dev/docs/usage/release) (tags + GitHub Release + `cargo publish` as configured).
   - **Manual override:** you can still `cargo publish -p shahanshahi` and create the GitHub Release yourself if you disable the variable or skip automation.

**First publish to crates.io** is often **manual** ([crates.io limitation](https://release-plz.dev/docs/github/quickstart); trusted publishing has similar constraints). Do the first `cargo publish` locally, then enable automation for subsequent versions.

Pre-release versions (`0.2.0-alpha.1`) are still allowed; configure via release-plz / Cargo as needed.

### Manual fallback (no automation)

If release-plz is disabled or unsuitable for a one-off:

1. Land changes on `main` with CI green.
2. Update [`CHANGELOG.md`](../CHANGELOG.md) and bump `version` in the root [`Cargo.toml`](../Cargo.toml).
3. Tag `vX.Y.Z` and push; run `cargo publish -p shahanshahi`; create the **GitHub Release**.

## Summary

| Topic | Rule |
|-------|------|
| Spec vs code | Spec + golden dates lead; code implements. |
| CI | rustfmt, clippy (`-D warnings`), test, packaging dry-run, audit + deny on each PR (and weekly audit schedule). |
| Version | Root `Cargo.toml` `version`; SemVer; `0.x` allows API evolution with changelog discipline. |
| MSRV | Documented in `Cargo.toml`; bump ⇒ at least minor semver bump. |
| Tags | `vX.Y.Z` matches crate version at release. |
| Security | Report vulnerabilities privately per [SECURITY.md](../SECURITY.md), not public issues. |
| Automation | Dependabot, path-based PR labels, release-plz (draft release PRs + gated publish); see [Automation (GitHub)](#automation-github) and [Release process](#release-process-cratesio). |

Questions belong in GitHub issues (see [issue templates](../.github/ISSUE_TEMPLATE/)) — **except** undisclosed security problems; use [SECURITY.md](../SECURITY.md).
