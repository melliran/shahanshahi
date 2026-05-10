mod cli;
mod convert;
mod format;

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io::{self, Write};

use cli::{Calendar, Cli, Command, CompletionsArgs, ConvertArgs};

pub fn run() -> Result<()> {
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
    // Use argv[0] so `shcal completions bash` generates "shcal" completions, not "shahanshahi".
    let bin_name = std::env::args()
        .next()
        .and_then(|s| {
            std::path::Path::new(&s)
                .file_stem()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "shahanshahi".to_string());
    run_completions_into(args, &bin_name, &mut io::stdout())
}

fn run_completions_into(args: CompletionsArgs, bin_name: &str, out: &mut dyn Write) -> Result<()> {
    let mut cmd = Cli::command();
    generate(args.shell, &mut cmd, bin_name, out);
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap_complete::Shell;

    fn completions_for(shell: Shell) -> String {
        completions_for_bin(shell, "shahanshahi")
    }

    fn completions_for_bin(shell: Shell, bin_name: &str) -> String {
        let args = cli::CompletionsArgs { shell };
        let mut buf = Vec::new();
        run_completions_into(args, bin_name, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn completions_bash_non_empty() {
        let out = completions_for(Shell::Bash);
        assert!(!out.is_empty());
        assert!(out.contains("shahanshahi"));
        assert!(out.contains("convert"));
    }

    #[test]
    fn completions_zsh_non_empty() {
        let out = completions_for(Shell::Zsh);
        assert!(!out.is_empty());
        assert!(out.contains("shahanshahi"));
        assert!(out.contains("convert"));
    }

    #[test]
    fn completions_fish_non_empty() {
        let out = completions_for(Shell::Fish);
        assert!(!out.is_empty());
        assert!(out.contains("shahanshahi"));
        assert!(out.contains("convert"));
    }

    #[test]
    fn completions_powershell_non_empty() {
        let out = completions_for(Shell::PowerShell);
        assert!(!out.is_empty());
        assert!(out.contains("shahanshahi"));
        assert!(out.contains("convert"));
    }

    #[test]
    fn completions_shcal_bin_name() {
        let out = completions_for_bin(Shell::Bash, "shcal");
        assert!(!out.is_empty());
        assert!(out.contains("shcal"));
        assert!(out.contains("convert"));
    }
}
