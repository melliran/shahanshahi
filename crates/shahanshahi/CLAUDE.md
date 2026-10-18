# Calendar algorithm code style (`crates/shahanshahi/**/*.rs`)

- Use **descriptive local variable names** over cryptic single letters in multi-step algorithms.
- Use `year` / `month` / `day` only for real civil components; name intermediates explicitly (`shifted_month`, `month_packed`, `offset_days`, `rata_die`, …).
- For ported formulas (e.g. JDN ↔ Gregorian), add a brief comment naming the algorithm family and its phases.
- Public params may use SPEC abbreviations (`y_h`, `y_s`) if documented on the function.
- Full rationale: `docs/CALENDAR_CODE_STYLE.md`.
