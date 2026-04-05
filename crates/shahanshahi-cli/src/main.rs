mod cli;
mod convert;
mod format;

use anyhow::{bail, Result};
use clap::Parser;

use cli::{Calendar, Cli, Command, ConvertArgs};

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
    }
}

fn run_convert(args: ConvertArgs) -> Result<()> {
    let (from, to) = resolve_direction(args.from, args.to)?;

    match args.date.as_deref() {
        Some(s) if s != "-" => {
            let input = convert::parse_ymd(s)?;
            let output = convert::convert(input, &from, &to, args.proleptic)?;
            format::write_single(&args.format, &output)
        }
        _ => format::run_batch(&args.format, &from, &to, args.proleptic),
    }
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
