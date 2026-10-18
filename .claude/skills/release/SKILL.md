---
name: release
description: Release workflow for the shahanshahi workspace (library, CLI, WASM crates) — versioning scheme, release-plz automation, and tagging. Use when cutting a release, publishing to crates.io/npm, or bumping crate versions.
---

Library and CLI are versioned independently. Releases use [release-plz](https://release-plz.dev/), **manually triggered** (`gh workflow run release-plz.yml --ref main`) — it never runs on push. It drafts a release PR (version bump + changelog); after merging it, publish by hand (`RELEASE_PLZ_PUBLISH=false`). Library tags: `vX.Y.Z`; CLI tags: `shahanshahi-cli-vX.Y.Z`. See `docs/ENGINEERING.md` for the full release workflow.
