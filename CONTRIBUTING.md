# Contributing

Thanks for helping improve Persian calendar tooling for software.

## Attribution

This project does **not** use automated-tool attribution in Git history:

- Do **not** add `Co-authored-by:` trailers for Cursor, Copilot, or similar tools.
- Do **not** add `Made-with:` (or “Made with Cursor”) **commit trailers** or equivalent `git commit --trailer …` lines for tooling.
- Do **not** add “Made with Cursor” (or equivalent) branding to README, documentation, **or GitHub PR/issue descriptions** (including footers injected by an editor or agent).

**Cursor users:** disable Agent commit attribution so commits stay clean — **Cursor Settings → Agent → Attribution** (turn off tool trailers / co-author injection for this project or globally, per your preference).

Project direction and rationale are summarized in [`docs/VISION.md`](./docs/VISION.md). Engineering policy, CI meaning, and semver rules are in [`docs/ENGINEERING.md`](./docs/ENGINEERING.md). **Security vulnerabilities** are reported privately per [`SECURITY.md`](./SECURITY.md) (not via public issues).

1. **Spec first** — Calendar behavior is defined in [`SPEC.md`](./SPEC.md). Propose rule changes there (with citations) before or alongside code.
2. **Golden dates** — New conversions or edge cases should add rows to [`data/reference-dates.json`](./data/reference-dates.json) with a **primary source** in the `source` field.
3. **Rust** — `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, and `cargo test --workspace` should pass before you open a PR.

## Git workflow (branches, commits, pull requests)

Agreeing on this *before* milestone work scales cleanly: everyone reads the same playbook, history stays searchable, and reviews stay small.

### Branches

- Branch off **`main`** (or the current integration branch your team uses).
- Use short, lowercase slugs with a **type prefix**:
  - `feat/` — new behavior or public API
  - `fix/` — bug fixes
  - `docs/` — documentation only
  - `chore/` — tooling, CI, refactors with no user-visible change
  - `test/` — tests and fixtures only
- Examples: `feat/jdn-conversion`, `docs/spec-cross-links`, `fix/leap-year-boundary`.

Avoid long-lived personal branches; rebase or merge `main` regularly so PRs do not rot.

### Commits

- **One logical change per commit** when practical; squash on merge is fine if the PR narrative is clear.
- **Subject line:** imperative mood, ~72 characters, no trailing period. Optional **Conventional Commits** prefix for consistency:
  - `feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`
- **Body:** use when context is not obvious — *what* changed and *why* (not how line-by-line). Link issues with `Fixes #123` or `See #123` when relevant.
- Do **not** add tooling attribution trailers; see [Attribution](#attribution) above.

### Pull requests

- **Title:** same spirit as the commit subject — scannable and specific.
- **Description** should answer:
  - **What** does this PR do?
  - **Why** (milestone goal, spec section, issue link)?
  - **How to verify** (commands run, notable edge cases)?
- **Link tracking:** reference the GitHub issue and/or milestone (e.g. v0.1 — spec-backed MVP) in the PR description so work stays tied to the roadmap.
- **Scope:** prefer smaller PRs over “everything” dumps; stack follow-ups if needed.
- **Before requesting review:** `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` pass locally (or explain CI-only exceptions in the PR).

Maintainers may squash-merge to keep `main` linear; use the PR title/description as the merge commit message when helpful.

**Templates:** New PRs and choosable issue types use [`.github/pull_request_template.md`](./.github/pull_request_template.md) and [`.github/ISSUE_TEMPLATE/`](./.github/ISSUE_TEMPLATE/). Replace the HTML comments with real content before submitting; do not leave tool-marketing footers in the description.

## Labels

Pick **one primary type** where it fits, then add narrow tags (e.g. `spec` + `tests`) as needed. Default GitHub labels (`bug`, `enhancement`, `documentation`, `question`, `good first issue`, `help wanted`, `duplicate`, `invalid`, `wontfix`) stay available for triage.

| Label | Use when |
|-------|----------|
| `spec` | Rules in `SPEC.md`, primary sources, or rows in `data/reference-dates.json` |
| `tests` | Test suite, fixtures, or golden-date coverage |
| `chore` | Tooling, CI, or repo maintenance without user-visible calendar behavior |
| `performance` | Speed, allocations, or binary-size work |
| `blocked` | Waiting on an external decision, spec clarity, or another dependency |

Questions or historical sources are welcome in issues.
