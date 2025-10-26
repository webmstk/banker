use clap::{Parser, ValueEnum};

use std::fmt::Display;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    /// csv
    Csv,
    /// json
    Json,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Format::*;
        match self {
            Csv => write!(f, "csv"),
            Json => write!(f, "json"),
        }
    }
}

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about = "Конвертер финансовых операций между разными финансовыми форматами")]
#[command(after_help = "Alternative usage: bconv [OPTIONS] <samples/data.csv\n\
    Note: --input flag has priority over stdout")]
pub struct Cli {
    /// Путь к исходному файлу
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Формат исходного содержимого
    #[arg(value_enum, long)]
    pub in_format: Option<Format>,

    /// Формат результата
    #[arg(value_enum, long)]
    pub out_format: Option<Format>,

    /// Путь к файлу для сохранения результата
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
