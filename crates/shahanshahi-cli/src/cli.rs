use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "shahanshahi",
    version,
    about = "Shahanshahi (Imperial Iranian) ↔ Gregorian date conversion"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Convert dates between Shahanshahi and Gregorian calendars.
    Convert(ConvertArgs),
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Source calendar [default: inferred from --to, or gregorian].
    #[arg(long, value_enum)]
    pub from: Option<Calendar>,

    /// Target calendar [default: inferred from --from, or shahanshahi].
    #[arg(long, value_enum)]
    pub to: Option<Calendar>,

    /// I/O format.
    #[arg(long, short, default_value = "text", value_enum)]
    pub format: Format,

    /// Allow dates outside the legal Shahanshahi civil era.
    #[arg(long)]
    pub proleptic: bool,

    /// Date in YYYY-MM-DD format (omit or pass "-" for stdin).
    pub date: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Calendar {
    /// Proleptic Gregorian calendar.
    #[value(alias = "g", alias = "greg")]
    Gregorian,

    /// Shahanshahi (Imperial Iranian) calendar.
    #[value(alias = "sh", alias = "s")]
    Shahanshahi,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// One YYYY-MM-DD per line.
    Text,

    /// JSON (array for batch, object for single).
    Json,

    /// CSV with year,month,day header.
    Csv,
}
