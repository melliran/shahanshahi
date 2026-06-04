# shahanshahi-cli

Command-line tool for deterministic offline conversion between the **Shahanshahi (Imperial Iranian) civil calendar** and the proleptic Gregorian calendar.

Installs two binaries: **`shahanshahi`** (full name) and **`shcal`** (short alias). Both expose the same conversion logic from the [`shahanshahi`](https://crates.io/crates/shahanshahi) library.

## Install

```bash
cargo install shahanshahi-cli
```

## Usage

```text
shahanshahi convert [OPTIONS] [DATE]
shahanshahi completions <SHELL>
```

`DATE` is a civil date as `YYYY-MM-DD`. Omit it (or pass `-`) to read batch input from stdin.

### Options

| Flag | Meaning |
|------|---------|
| `--from <calendar>` | Source: `gregorian` (`g`, `greg`) or `shahanshahi` (`sh`, `s`) |
| `--to <calendar>` | Target (same values) |
| `-f`, `--format <fmt>` | `text` (default), `json`, or `csv` |
| `--month-names` | Print human-readable Persian month names in text output |
| `--proleptic` | Allow Shahanshahi dates outside the legal civil era |

If only one of `--from` / `--to` is set, the other is inferred. Default direction is Gregorian → Shahanshahi.

## Examples

```bash
# Gregorian → Shahanshahi
shahanshahi convert 1976-03-21
# 2535-01-01

# Shahanshahi → Gregorian
shahanshahi convert --from sh --to g 2535-01-01
# 1976-03-21

# Human-readable month names
shahanshahi convert 1976-03-21 --month-names
# 1 Farvardin 2535

# Batch from stdin
printf "1976-03-21\n1977-03-21\n" | shahanshahi convert

# JSON batch
echo '[{"year":1976,"month":3,"day":21}]' | shahanshahi convert -f json

# CSV batch
printf "year,month,day\n1976,3,21\n" | shahanshahi convert -f csv
```

## Shell completions

```bash
# bash
shahanshahi completions bash > ~/.local/share/bash-completion/completions/shahanshahi

# zsh
shahanshahi completions zsh > ~/.config/zsh/completions/_shahanshahi

# fish
shahanshahi completions fish > ~/.config/fish/completions/shahanshahi.fish

# PowerShell
shahanshahi completions powershell > shahanshahi.ps1
```

## Output formats

**`text`** — one `YYYY-MM-DD` line per input date (default).  
**`json`** — single object `{"year":…,"month":…,"day":…}` or array in batch mode.  
**`csv`** — `year,month,day` header + data rows.

## Exit codes

- `0` — success
- `1` — parse or conversion error (message on stderr)

## See also

- [`shahanshahi`](https://crates.io/crates/shahanshahi) — the underlying library crate
- [SPEC.md](https://github.com/melliran/shahanshahi/blob/main/SPEC.md) — era bounds and conversion algorithm
