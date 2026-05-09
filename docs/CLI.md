# Command-line tool (`shahanshahi-cli`)

The workspace crate [`crates/shahanshahi-cli`](../crates/shahanshahi-cli) installs two binaries: **`shahanshahi`** (full name) and **`shcal`** (short alias). Both expose the same calendar arithmetic as the [`shahanshahi`](../crates/shahanshahi) library (no duplicate conversion logic).

The CLI crate is versioned **independently** of the library (starts at **0.1.0**); its public contract is flags and I/O formats, not the Rust API.

## Releases and changelog

- History: [`CHANGELOG-CLI.md`](../CHANGELOG-CLI.md).
- While [CLI tooling](https://github.com/melliran/shahanshahi/milestone/2) is open, install from the repo (`cargo install --path` or `cargo run`) and add notes under **`[Unreleased]`** in that changelog.
- When the milestone closes, the plan is **`shahanshahi-cli` 0.1.0** on crates.io together with **`shahanshahi` 0.2.1** (library changelog mentions the CLI). [`release-plz.toml`](../release-plz.toml) lists only the library until you add **`shahanshahi-cli`** for release automation. Full steps: [`docs/ENGINEERING.md`](./ENGINEERING.md) → *Multi-crate releases*.

## Install

From a clone of this repository:

```bash
cargo install --path crates/shahanshahi-cli
```

During development you can run without installing:

```bash
cargo run -p shahanshahi-cli -- <ARGS>
```

Until the first **`shahanshahi-cli`** publish from the registry, prefer `cargo install --path` or `cargo run` (see *Releases and changelog* above).

## Command overview

```text
shahanshahi convert [OPTIONS] [DATE]
```

- **`DATE`** — optional. A single civil date as **`YYYY-MM-DD`** (Gregorian or Shahanshahi components depending on `--from`).  
  Omit **`DATE`**, or pass **`-`**, to read **batch input from stdin** (see [Formats](#formats)).
- Subcommands may grow over time; today only **`convert`** exists.

### Options

| Flag | Meaning |
|------|---------|
| `--from <calendar>` | Source calendar: `gregorian` (aliases `g`, `greg`) or `shahanshahi` (`sh`, `s`). |
| `--to <calendar>` | Target calendar (same values). |
| `-f`, `--format <fmt>` | I/O format: `text` (default), `json`, or `csv`. |
| `--proleptic` | Allow Shahanshahi dates outside the default **legal civil era** (same semantics as the library’s `proleptic` feature; see [`SPEC.md`](../SPEC.md)). |

If only one of **`--from`** / **`--to`** is set, the other is inferred. If neither is set, the default direction is **Gregorian → Shahanshahi**.

You cannot set **`--from`** and **`--to`** to the same calendar.

## Formats

The **`-f` / `--format`** value applies to both input and output **in batch mode** (stdin). For a **single** positional **`DATE`**, the argument is always parsed as **`YYYY-MM-DD`** text; **`--format`** controls **output only**.

### `text` (default)

- **Single argument:** one line printed: `YYYY-MM-DD`.
- **Stdin:** one input date per line; empty lines are skipped. One output line per non-empty input line.

Years may be negative for proleptic Gregorian-style input (e.g. `-0559-01-01`). Month and day may omit leading zeros (`2535-1-1`).

### `json`

- **Single argument:** one JSON **object** on stdout:  
  `{"year": <i32>, "month": <u8>, "day": <u8>}`  
  (target calendar components).
- **Stdin:** a JSON **array** of such objects. Stdout is a JSON **array** of converted objects, same length and order.

### `csv`

Uses the **`csv`** crate: first row is a header, then data rows.

- **Header:** `year,month,day`
- **Single argument:** stdout includes the header row plus one data row.
- **Stdin:** the reader consumes the header and subsequent rows; stdout rows are written with the same header.

All numeric fields are integers consistent with the active `--from` / `--to` calendars.

## Exit status

- **`0`** — success.
- **`1`** — conversion or parse error (message on stderr).

## Shell completions

Generate and install a completion script with the `completions` subcommand. Run the command under the binary name you use daily — each binary generates completions for itself:

```bash
# bash
shahanshahi completions bash > ~/.local/share/bash-completion/completions/shahanshahi
# or, if you use shcal:
shcal completions bash > ~/.local/share/bash-completion/completions/shcal

# zsh (add the directory to $fpath if not already there)
shahanshahi completions zsh > ~/.config/zsh/completions/_shahanshahi

# fish
shahanshahi completions fish > ~/.config/fish/completions/shahanshahi.fish

# PowerShell
shahanshahi completions powershell > shahanshahi.ps1
# then add `. ./shahanshahi.ps1` to your $PROFILE
```

After installing, restart your shell (or source the file) to activate tab-completion for subcommands, flags, and calendar names.

## Examples

```bash
# Default: Gregorian → Shahanshahi, text
shahanshahi convert 1976-03-21
# 2535-01-01

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

# Outside legal era (requires --proleptic for G→SH into labelled Shahanshahi)
shahanshahi convert --proleptic 1996-03-20
# 2555-01-01
```

## See also

- [`SPEC.md`](../SPEC.md) — era bounds and proleptic policy.
- [`README.md`](../README.md) — workspace overview and library quickstart.
