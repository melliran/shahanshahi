mod cli;
mod convert;
mod format;

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io;

use cli::{Calendar, Cli, Command, CompletionsArgs, ConvertArgs};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Convert(args) => run_convert(args),
        Command::Completions(args) => run_completions(args),
    }
}

fn run_convert(args: ConvertArgs) -> Result<()> {
    let (from, to) = resolve_direction(args.from, args.to)?;
    // Month names only apply to Shahanshahi output; silently no-op for Gregorian.
    let use_month_names = args.month_names && to == Calendar::Shahanshahi;

    match args.date.as_deref() {
        Some(s) if s != "-" => {
            let input = convert::parse_ymd(s)?;
            let output = convert::convert(input, &from, &to, args.proleptic)?;
            format::write_single(&args.format, &output, use_month_names)
        }
        _ => format::run_batch(&args.format, &from, &to, args.proleptic, use_month_names),
    }
}

fn run_completions(args: CompletionsArgs) -> Result<()> {
    let mut cmd = Cli::command();
    generate(args.shell, &mut cmd, "shahanshahi", &mut io::stdout());
    Ok(())
}

/// Infer the missing direction when only one of --from / --to is given.
/// Default (neither specified): Gregorian → Shahanshahi.
fn resolve_direction(from: Option<Calendar>, to: Option<Calendar>) -> Result<(Calendar, Calendar)> {
    match (from, to) {
        (Some(f), Some(t)) if f == t => {
            bail!("--from and --to must specify different calendars");
        }
        (Some(f), Some(t)) => Ok((f, t)),
        (Some(Calendar::Gregorian), None) | (None, None) => {
            Ok((Calendar::Gregorian, Calendar::Shahanshahi))
        }
        (Some(Calendar::Shahanshahi), None) | (None, Some(Calendar::Gregorian)) => {
            Ok((Calendar::Shahanshahi, Calendar::Gregorian))
        }
        (None, Some(Calendar::Shahanshahi)) => Ok((Calendar::Gregorian, Calendar::Shahanshahi)),
    }
}
