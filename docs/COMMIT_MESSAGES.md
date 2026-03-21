# Commit and message conventions

These conventions keep history **readable**, work well with **changelog and release tooling**, and make **reviews** easier. They are optional to adopt in full on day one, but new work should aim to match them.

## Principles

1. **[Conventional Commits](https://www.conventionalcommits.org/)** — a small grammar for the subject line (`type`, optional `scope`, `description`) and optional **footers** (`Fixes #123`, `BREAKING CHANGE: …`). This interoperates with **release automation** (e.g. semantic-release-style tooling, **release-plz**, changelog generators).
2. **Imperative, present-tense subjects** — “add”, “fix”, “document” (not “added” / “fixes”). Aligns with common Git guidance and many project style guides.
3. **One logical change per commit** when practical (or a clear narrative if the maintainer squash-merges).
4. **No machine-generated tool marketing in messages** — see [`CONTRIBUTING.md`](../CONTRIBUTING.md) attribution section and [`.cursor/rules/contributor-attribution.mdc`](../.cursor/rules/contributor-attribution.mdc).

This repository does **not** use the Linux kernel’s **Signed-off-by** / patch-email workflow unless maintainers adopt it later.

## Subject line (required)

Format (Conventional Commits):

```text
<type>[(optional-scope)][!]: <short description>
```

- **`type`** (common set for this repo):
  - `feat` — new behavior or public API
  - `fix` — bug fix
  - `docs` — documentation only
  - `test` — tests or fixtures only
  - `refactor` — internal change without intended behavior change
  - `chore` — tooling, CI, metadata, repo maintenance
  - `perf` — performance improvement
  - `build` / `ci` — build system or CI config (use when clearer than `chore`)
- **`scope`** (optional): crate or area, e.g. `feat(shahanshahi):`, `fix(ci):`, `docs(spec):`.
- **`!`** (optional): marks a **breaking change** for parsers that support it (often paired with a `BREAKING CHANGE:` footer for humans).

**Style:**

- **Imperative mood**, **~50–72 characters** for the subject line, **no trailing period**.
- **Lowercase** description after the colon (common OSS style); proper nouns (`SPEC.md`, `Rust`) may stay as needed.

**Examples:**

```text
feat(shahanshahi): expose Gregorian conversion helpers
fix(convert): correct offset across Esfand boundary
docs: link calendar code style from CONTRIBUTING
chore(cursor): add commit message convention rule
```

## Body (optional but encouraged)

Use a blank line after the subject, then:

- **What** changed at a high level (if not obvious from the subject).
- **Why** it changed (spec section, issue, regression).
- **Links:** `Fixes #42`, `See #42`, or `Closes #42` on its own line in the footer area is fine.

Wrap body text near **72 columns** for terminal readability (common convention).

## Footers

- **Issue tracking:** `Fixes #123` / `See #123` (GitHub understands these).
- **Breaking changes:** describe user impact, e.g.  
  `BREAKING CHANGE: ShahanshahiDate::foo now returns Result`
- **Do not** add `Co-authored-by:`, `Made-with:`, or other **tool-attribution** trailers in this repository.

## Pull request titles

Match the **same spirit** as a good commit subject: Conventional-Commit-style title when it helps release notes, and a body that answers **what / why / how to verify** (see [`CONTRIBUTING.md`](../CONTRIBUTING.md)).

## Further reading

- [Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0/)
- [How to Write a Git Commit Message](https://cbea.ms/git-commit/) (Chris Beams) — imperative subject and “why” in the body
- [`CONTRIBUTING.md`](../CONTRIBUTING.md) — branches, commits, PRs for this repo
