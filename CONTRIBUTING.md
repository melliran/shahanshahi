# Contributing

Thanks for helping improve Persian calendar tooling for software.

## Attribution

This project does **not** use automated-tool attribution in Git history:

- Do **not** add `Co-authored-by:` trailers for Cursor, Copilot, or similar tools.
- Do **not** add `Made-with:` (or “Made with Cursor”) **commit trailers** or equivalent `git commit --trailer …` lines for tooling.
- Do **not** add “Made with Cursor” (or equivalent) branding to README or documentation.

**Cursor users:** disable Agent commit attribution so commits stay clean — **Cursor Settings → Agent → Attribution** (turn off tool trailers / co-author injection for this project or globally, per your preference).

Project direction and rationale are summarized in [`docs/VISION.md`](./docs/VISION.md).

1. **Spec first** — Calendar behavior is defined in [`SPEC.md`](./SPEC.md). Propose rule changes there (with citations) before or alongside code.
2. **Golden dates** — New conversions or edge cases should add rows to [`data/reference-dates.json`](./data/reference-dates.json) with a **primary source** in the `source` field.
3. **Rust** — `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, and `cargo test --workspace` should pass before you open a PR.

Questions or historical sources are welcome in issues.
