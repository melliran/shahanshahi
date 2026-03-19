# Shahanshahi calendar — implementation specification (draft)

This document **freezes** what the `shahanshahi` Rust crate implements. Until sections below are filled from **primary sources** (law, official gazette, authorized astronomical tables), the crate stays at `0.0.x` and `SPEC_VERSION` remains `0-unstable`.

## Scope

- **Intent**: Pahlavi-era **Imperial Iranian (شاهنشاهی)** civil calendar, for use as a **deterministic** converter in software (similar ecosystem role to Jalali libraries).
- **Non-goals** (unless explicitly added later): religious calendars, lunar Hijri, historical dating before the defined era of applicability, UI localization beyond what the crate documents.

## Primary references (required before 0.1.0)

Fill with full citations (title, publisher, date, page/article). Link or archive URL if available.

| Topic | Citation | Notes |
|--------|-----------|--------|
| Epoch | _TODO_ | e.g. anchor year 1 definition |
| Leap rule | _TODO_ | exact formula + examples |
| Month lengths | _TODO_ | ordinary vs leap year |
| Era of applicability | _TODO_ | first/last civil dates this spec covers |
| Month names (if in scope) | _TODO_ | mapping month index ↔ official name |

## Calendar rules (to be completed)

### Epoch

_TODO_: Define year 1 relative to a documented astronomical or historical anchor.

### Months

_TODO_: Count, names (if any), days per month in ordinary and leap years.

### Leap years

_TODO_: Rule, examples (at least one leap and one common year), boundary behavior.

### Proleptic use

_TODO_: State explicitly whether dates outside the legal era of applicability are **rejected**, **allowed with a warning**, or **computed by extending the same arithmetic** (proleptic). Default recommendation for v1: **reject** or **feature-gated proleptic** to avoid silent historical errors.

## Versioning

- **`SPEC.md`**: When rules change, add an entry to [Changelog](#changelog) with date and rationale.
- **Crate semver**: Bugfixes that align code with this spec are patch bumps. Intentional spec changes require minor/major per semver and a migration note.

## Relationship to `data/reference-dates.json`

Every row in that file must cite a **traceable source** independent of this codebase. The implementation must pass all golden tests derived from those rows.

## Changelog

| Date | Change |
|------|--------|
| _YYYY-MM-DD_ | Initial draft template. |
