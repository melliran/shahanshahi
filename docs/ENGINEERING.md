# Engineering, quality gates, and versioning

This document describes how we build and ship **shahanshahi** as a Rust library. It complements [`VISION.md`](./VISION.md) (why the project exists) and [`CONTRIBUTING.md`](../CONTRIBUTING.md) (day-to-day contribution rules).

The workspace also ships **`shahanshahi-cli`** ([`crates/shahanshahi-cli`](../crates/shahanshahi-cli)), a `shahanshahi` binary documented in [`CLI.md`](./CLI.md). It versions separately from the library. [`release-plz.toml`](../release-plz.toml) lists only **`shahanshahi`** until the [CLI tooling](https://github.com/melliran/shahanshahi/milestone/2) milestone is done and you add **`shahanshahi-cli`** for automated releases. Until then, log CLI work under **`[Unreleased]`** in [`CHANGELOG-CLI.md`](../CHANGELOG-CLI.md). See [Multi-crate releases](#multi-crate-releases-shahanshahi-and-shahanshahi-cli).

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
| [test](../.github/workflows/test.yml) | `cargo test --workspace --all-features` — tests pass (including `proleptic`). |
| [crate package](../.github/workflows/crate-package.yml) | `cargo publish -p shahanshahi --dry-run` and `cargo publish -p shahanshahi-cli --dry-run` — each crate **packages and builds** as crates.io would. |
| [audit](../.github/workflows/audit.yml) | **`cargo audit`** (RustSec) + **`cargo deny check`** (advisories, licenses, sources) on every PR and weekly on a schedule. |
| [release-plz](../.github/workflows/release-plz.yml) | On push to **`main`**: maintain a **draft release PR** (version + changelog). **Publish** job runs only if repo variable **`RELEASE_PLZ_PUBLISH`** is `true` and **`CARGO_REGISTRY_TOKEN`** is set. |

**`Cargo.lock`** is committed at the workspace root so CI and security scans are **deterministic**. Refresh it when dependencies change (`cargo update` as appropriate).

Locally, match CI before opening a PR: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features`, and when you touch dependencies or lockfile: `cargo audit` and `cargo deny check` (install via [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) or `cargo install`).

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

**Source of truth (library):** Crates that set `version.workspace = true` (today: **`shahanshahi`**) take their version from the root [`Cargo.toml`](../Cargo.toml) (`[workspace.package] version`). **Do not** diverge the published library version from that field.

**Source of truth (CLI):** **`shahanshahi-cli`** keeps its own `version` in [`crates/shahanshahi-cli/Cargo.toml`](../crates/shahanshahi-cli/Cargo.toml). Bump it on its own semver cadence; it does **not** track `[workspace.package] version`.

**MSRV:** `rust-version` in `Cargo.toml` is the **minimum supported Rust version**. **Raising MSRV** is a **semver-visible** change: treat it as at least a **minor** bump in `0.x` (and a **minor** bump post-1.0), and record it in the changelog.

**Git tags:** When publishing to crates.io, use an **annotated tag** `vX.Y.Z` whose numbers **match** `Cargo.toml` at that commit (e.g. `v0.1.0` ↔ `version = "0.1.0"`). If you publish **both** crates from one release train, use **disambiguated** tags (e.g. `shahanshahi-v0.2.1` and `shahanshahi-cli-v0.1.0`) or separate release commits per crate so tags stay unambiguous.

### Multi-crate releases (`shahanshahi` and `shahanshahi-cli`)

| Crate | Version field | Changelog |
|-------|----------------|-----------|
| **`shahanshahi`** | Root [`Cargo.toml`](../Cargo.toml) `[workspace.package] version` | [`CHANGELOG.md`](../CHANGELOG.md) |
| **`shahanshahi-cli`** | [`crates/shahanshahi-cli/Cargo.toml`](../crates/shahanshahi-cli/Cargo.toml) `version` | [`CHANGELOG-CLI.md`](../CHANGELOG-CLI.md) |

**First CLI release (after milestone [CLI tooling](https://github.com/melliran/shahanshahi/milestone/2) closes):** ship **`shahanshahi-cli` 0.1.0** and bump **`shahanshahi` to 0.2.1** so [`CHANGELOG.md`](../CHANGELOG.md) can mention the binary on crates.io. In [`crates/shahanshahi-cli/Cargo.toml`](../crates/shahanshahi-cli/Cargo.toml), set the `shahanshahi` dependency to `version = "0.2.1"` next to `path` (keep that in sync with the workspace library version whenever you publish). Add `[[package]] name = "shahanshahi-cli"` / `changelog_path = "CHANGELOG-CLI.md"` to [`release-plz.toml`](../release-plz.toml) if you want release-plz to manage the CLI; otherwise publish by hand.

**Path + version:** `cargo publish` requires a crates.io **`version`** on `shahanshahi` alongside **`path`**; CI’s `cargo publish -p shahanshahi-cli --dry-run` checks that the manifest is valid.

## Release process (crates.io)

We use **[release-plz](https://release-plz.dev/)** so versioning and changelog updates are **proposed as a PR** and **publishing** is a separate, **gated** step.

> **Important:** If the workflow fails with **`GitHub Actions is not permitted to create or approve pull requests`**, that is a **GitHub** permission problem, **not** crates.io. Adding **`CARGO_REGISTRY_TOKEN`** does **not** fix it. You must either allow Actions to open PRs (step 1 below) or set **`RELEASE_PLZ_GITHUB_TOKEN`** (step “Fix B” under [HTTP 403](#if-you-still-get-http-403-when-opening-the-release-pr)).

### One-time GitHub setup

1. **Let Actions open PRs (required for `release-plz-pr`):**  
   **Settings → Actions → General → Workflow permissions**
   - Set **Workflow permissions** to **Read and write permissions** (not “Read repository contents”).
   - Turn **on** **Allow GitHub Actions to create and approve pull requests**.  
   Official docs: [Allowing GitHub Actions to create or approve pull requests](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-github-actions-to-create-or-approve-pull-requests).  
   If this option is **missing or disabled**, your **organization** may forbid it — an org owner must change [organization Actions policies](https://docs.github.com/en/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#preventing-github-actions-from-creating-or-approving-pull-requests), or use the PAT workaround below.

2. **Secrets (crates.io only):** add **`CARGO_REGISTRY_TOKEN`** ([crates.io token](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish) with scopes `publish-new` and `publish-update`). Used **only** by the **`release-plz-release`** publish job when `RELEASE_PLZ_PUBLISH=true`. It does **not** affect opening release PRs.

3. **Variables:** add repository variable **`RELEASE_PLZ_PUBLISH`** = `true` only when you want CI to run **`release-plz release`** (crates.io + GitHub Release). Leave unset or not `true` to **skip publishing** while still opening release PRs.

#### If you still get HTTP 403 when opening the release PR

Error from GitHub’s API often looks like:  
`"message": "GitHub Actions is not permitted to create or approve pull requests."`

- **Fix A:** Confirm step (1): both **read/write** workflow permissions **and** the **create/approve pull requests** toggle (repo or org).
- **Fix B (PAT):** Create a **fine-grained personal access token** (or classic PAT) for a bot/user account with **Contents** and **Pull requests** write access to this repo. Add it as repository secret **`RELEASE_PLZ_GITHUB_TOKEN`**. The workflow uses it **instead of** `GITHUB_TOKEN` when this secret is non-empty ([release-plz token docs](https://release-plz.dev/docs/github/token)).

### Day-to-day flow

#### Why a bot PR appears

Every push to `main` triggers the **`release-plz-pr`** job. Release-plz compares the latest crates.io version with the commits since that release, infers the next semver bump from Conventional Commit prefixes, and opens (or updates) a **draft** PR on a `release-plz-*` branch with the proposed `Cargo.toml` version and [`CHANGELOG.md`](../CHANGELOG.md) edits for packages listed in [`release-plz.toml`](../release-plz.toml) (today: **`shahanshahi`** only). The PR is labelled `chore`.

This is **normal and expected** — the bot PR is a convenience, not a mandate to merge immediately. It stays in draft until a maintainer is ready to release.

#### Preferred path — merge the release-plz draft PR

1. Merge ordinary work to `main` (Conventional Commit–style messages help release-plz infer semver bumps).
2. **Wait** for release-plz to open or update its draft PR with the proposed version and changelog.
3. **Review** that PR: check the version level (patch / minor / major), changelog prose, and spec readiness.
4. **Edit if needed** — push fixup commits to the bot's branch for changelog wording or version override. Release-plz tolerates manual edits on its branch.
5. Mark the PR **ready for review** and **merge** when satisfied.
6. **Publishing** happens according to the `RELEASE_PLZ_PUBLISH` variable:
   - **Automated:** with `RELEASE_PLZ_PUBLISH=true` and a valid `CARGO_REGISTRY_TOKEN`, the **`release-plz-release`** job on `main` runs [`release-plz release`](https://release-plz.dev/docs/usage/release) (tags + GitHub Release + `cargo publish`).
   - **Manual override:** you can still `cargo publish -p shahanshahi` and create the GitHub Release yourself if you disable the variable or skip automation.

#### When to use a manual release PR instead

Sometimes the bot PR is not suitable — for example, if you need to **batch several unreleased changes** under a hand-crafted changelog, **skip a version**, or release a **pre-release** tag (`0.2.0-alpha.1`) that release-plz does not propose automatically.

In that case:

1. Create a branch (e.g. `chore/release-0.3.0`) and manually bump `version` in the root [`Cargo.toml`](../Cargo.toml) and update [`CHANGELOG.md`](../CHANGELOG.md).
2. Open a PR, review, and merge to `main` as usual.
3. **Close the superseded bot PR** — do **not** merge both. Only one version bump should land per release. Add a comment (e.g. *"Superseded by #NN"*) so the history is clear.

After merging, release-plz will detect that the version on `main` already matches (or exceeds) what it would propose and will **not** re-open a conflicting PR.

#### Handling duplicate or stale release PRs

- **One release PR at a time.** If a release-plz draft PR and a manual release PR both exist, decide which to use and **close the other** (not merge).
- If you close a bot PR that is no longer relevant, release-plz will open a fresh one on the next push to `main` if there are new unreleased commits.
- Leaving stale bot PRs open is harmless (they stay in draft) but adds noise — prefer closing them promptly with a short note.

#### Publishing and `RELEASE_PLZ_PUBLISH`

| Variable state | What happens on merge to `main` |
|----------------|---------------------------------|
| **unset / not `true`** | Release-plz opens draft PRs only. **No** crates.io publish, **no** GitHub Release. You can still publish manually (`cargo publish -p shahanshahi` + tag + GitHub Release). |
| **`true`** | The **`release-plz-release`** job runs `release-plz release`, which creates a **git tag**, a **GitHub Release**, and calls **`cargo publish`** — provided **`CARGO_REGISTRY_TOKEN`** is set. |

**First publish to crates.io** is often **manual** ([crates.io limitation](https://release-plz.dev/docs/github/quickstart); trusted publishing has similar constraints). Do the first `cargo publish` locally, then enable automation for subsequent versions.

Pre-release versions (`0.2.0-alpha.1`) are still allowed; configure via release-plz / Cargo as needed.

### v0.1.0 readiness

Tracking checklist: [issue #8](https://github.com/melliran/shahanshahi/issues/8). Use this before treating **0.1.0** as shipped:

| Item | Notes |
|------|--------|
| **Spec** | [`SPEC.md`](../SPEC.md) documents behavior the crate claims; [`SPEC_VERSION`](../crates/shahanshahi/src/lib.rs) matches the spec header; *Known gaps* are acceptable to call out explicitly. |
| **Golden corpus + CI** | [`data/reference-dates.json`](../data/reference-dates.json) and integration tests (e.g. [`reference_dates.rs`](../crates/shahanshahi/tests/reference_dates.rs)) run on every PR via [test](../.github/workflows/test.yml). |
| **README / crate docs** | [`README.md`](../README.md) and crate-level docs describe scope, legal-era default vs **`proleptic`**, and non-goals consistent with the spec. |
| **Migration note** | [`MIGRATING.md`](./MIGRATING.md) covers `0.0.0` → first publish and **0.* semver** expectations. |

**`RELEASE_PLZ_PUBLISH` — do you need to change it for 0.1.0?**

- **Getting ready** (docs, merging feature work, reviewing a draft release-plz PR): **leave the variable unset or not `true`**. Release-plz still opens/updates **draft release PRs**; nothing is published to crates.io automatically.
- **Turn it to `true` only when** you want every successful **`release-plz release`** run on `main` (after you merge the release PR) to **publish to crates.io** and create the GitHub Release **without** a manual `cargo publish`. Prerequisites: valid **`CARGO_REGISTRY_TOKEN`** secret and confidence that automation matches your release policy.
- **First crates.io publish** is often done **manually** once ([release-plz quickstart](https://release-plz.dev/docs/github/quickstart)); you can keep **`RELEASE_PLZ_PUBLISH`** off until after that, then enable it for later versions if desired.

### Manual fallback (no automation)

If release-plz is disabled or unsuitable for a one-off:

1. Land changes on `main` with CI green.
2. **Library:** update [`CHANGELOG.md`](../CHANGELOG.md) and bump `version` in the root [`Cargo.toml`](../Cargo.toml).
3. **CLI (first release):** update [`CHANGELOG-CLI.md`](../CHANGELOG-CLI.md), bump **`shahanshahi-cli`** (e.g. **0.1.0**), set `shahanshahi = { path = "...", version = "…" }` to the library version you publish, and add **`shahanshahi-cli`** to `release-plz.toml` if you want the bot to handle it.
4. Tag (see [Git tags](#versioning-rules) above); run `cargo publish -p shahanshahi` and/or `cargo publish -p shahanshahi-cli`; create the **GitHub Release(s)**.

## Summary

| Topic | Rule |
|-------|------|
| Spec vs code | Spec + golden dates lead; code implements. |
| CI | rustfmt, clippy (`-D warnings`), test, packaging dry-run, audit + deny on each PR (and weekly audit schedule). |
| Version | Library: root `Cargo.toml` `[workspace.package] version`. CLI: `crates/shahanshahi-cli/Cargo.toml`. SemVer per surface; first CLI drop pairs **`shahanshahi-cli` 0.1.0** with **`shahanshahi` 0.2.1** — see [Multi-crate releases](#multi-crate-releases-shahanshahi-and-shahanshahi-cli). |
| MSRV | Documented in `Cargo.toml`; bump ⇒ at least minor semver bump. |
| Tags | `vX.Y.Z` matches crate version at release. |
| Security | Report vulnerabilities privately per [SECURITY.md](../SECURITY.md), not public issues. |
| Automation | Dependabot, path-based PR labels, release-plz (draft release PRs + gated publish); see [Automation (GitHub)](#automation-github), [Release process](#release-process-cratesio), and [v0.1.0 readiness](#v010-readiness). |

Questions belong in GitHub issues (see [issue templates](../.github/ISSUE_TEMPLATE/)) — **except** undisclosed security problems; use [SECURITY.md](../SECURITY.md).
