# Melliran, Shahanshahi, and standards-first work

This note captures **why this repository exists** and how it fits the broader goal of **open Persian technology** that software teams and organizations can adopt with confidence.

## Mission (Melliran)

[Melliran](https://github.com/melliran) is a non-profit open-source initiative focused on the future of **Persian technology, knowledge, and digital commons**. The aim is to publish **reusable building blocks**—libraries, data, and clear specifications—that NGOs, companies, archives, and civic projects can integrate without reinventing fragile calendar logic or unclear semantics.

## Standards before software

For calendars (and similar domains), **credibility comes from traceability**:

1. **Written spec** — [`SPEC.md`](../SPEC.md) states exactly which rules this crate implements (epoch, leap rule, month lengths, era of applicability, proleptic policy).
2. **Golden dates** — [`data/reference-dates.json`](../data/reference-dates.json) holds Y/M/D pairs with **independent citations**, driving tests so behavior cannot drift quietly.

Only after those are anchored does it make sense to grow the Rust API, `chrono`/`serde` integration, CLI, WASM, and so on. That order protects downstream users in finance, logistics, archives, research, and heritage systems—sectors that need **stable, explainable outputs**.

## What this repository is today

| Piece | Role |
|--------|------|
| [`SPEC.md`](../SPEC.md) | Versioned calendar contract for implementers (see `SPEC_VERSION` in crate / spec header) |
| [`data/`](../data/) | Schema + vetted reference rows for tests (rows still to be populated) |
| [`crates/shahanshahi/`](../crates/shahanshahi/) | Library crate; `SPEC_VERSION` tracks [`SPEC.md`](../SPEC.md) |
| [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) | `fmt`, `clippy`, `test` on `main` and PRs |

**Status:** pre-implementation skeleton—intentionally **spec-first**.

## Repository

[github.com/melliran/shahanshahi](https://github.com/melliran/shahanshahi)

## Contributor attribution (project policy)

- **Do not** add `Co-authored-by:` trailers for automated assistants (e.g. Cursor) or similar tooling.
- **Do not** add `Made-with:` / “Made with Cursor” **commit trailers** (or equivalent `git commit --trailer` usage).
- **Do not** add “Made with Cursor” (or equivalent) branding to README or published docs.

Human contributors retain normal Git attribution. See [`CONTRIBUTING.md`](../CONTRIBUTING.md).

---

*Last updated: 2026-03-19 — snapshot of project direction agreed in maintainer chat.*
