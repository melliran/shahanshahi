# Calendar implementation code style

This note applies to **calendar arithmetic** in the `shahanshahi` crate (and any future calendar modules): conversions, leap rules, day counting, and ports of published formulas.

## Naming

- **Prefer descriptive locals** over single-letter names (`g`, `dg`, `da`, `n`, `dim`, …) when implementing multi-step algorithms, unless the letter is a **universal symbol** in a one-liner (e.g. a well-known remainder in a single expression).
- **Reserve `year` / `month` / `day`** for values that are actual **civil** calendar components in the target calendar. If a variable is an intermediate “packed” month index (as in some JDN inverses) or a shifted March-based month, name it accordingly (`month_packed`, `shifted_month`, …).
- **Public API parameters** may keep short **domain abbreviations** when they match SPEC.md (`y_h` = Hijri Shamsi year, `y_s` = Shahanshahi year). Document those abbreviations in the function’s doc comment.

## Documentation

- For any **ported formula** (JDN ↔ Gregorian, similar blocks from papers or FAQs), add a short **`///` or `//` comment** that names the algorithm family (e.g. Fliegel & Van Flandern / standard JDN decomposition) and the **phases** of the computation (400-year / century / 4-year / month-day recovery).
- If the implementation intentionally matches an external source **line-for-line**, say so and link or cite in `SPEC.md` / commit message / `source` field on golden rows—not only in code comments.

## When terse names are acceptable

- **Trivial loops** or **one obvious accumulator** where a longer name adds noise.
- **Test-only** scratch variables with very small scope.

## Review checklist

When adding or changing calendar logic:

1. Would a new contributor recognize each binding **without** opening Wikipedia?
2. Are civil `month`/`day` values clearly distinguished from **intermediate** indices?
3. Is there a **one-paragraph** pointer to what mathematical object each phase computes?

See also: [`CONTRIBUTING.md`](../CONTRIBUTING.md), [`SPEC.md`](../SPEC.md).
