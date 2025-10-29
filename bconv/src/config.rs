use crate::cli::{self, Cli};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Display;
use std::io::{IsTerminal, stdin};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Csv,
    Json,
}

impl From<cli::Format> for Format {
    fn from(cli_format: cli::Format) -> Self {
        use cli::Format::*;
        match cli_format {
            Csv => Format::Csv,
            Json => Format::Json,
        }
    }
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

#[derive(Debug)]
pub struct Config {
    pub input_path: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub in_format: Format,
    pub out_format: Format,
    pub log_level: log::LevelFilter,
}

#[derive(Debug)]
pub enum ConfigError {
    InFormatUndefined,
    InputRequired,
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ConfigError::*;
        match self {
            InFormatUndefined => write!(f, "не получилось определить формат :("),
            InputRequired => write!(f, "отсутствуют входящие данные"),
        }
    }
}

impl TryFrom<Cli> for Config {
    type Error = ConfigError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        if let None = cli.input {
            if stdin().is_terminal() {
                return Err(ConfigError::InputRequired);
            }
        };

        let in_format = cli
            .in_format
            .map(|f| f.into())
            .or_else(|| get_format_from_extension(cli.input.as_ref()))
            .ok_or(ConfigError::InFormatUndefined)?;

        let out_format = cli
            .out_format
            .map(|f| f.into())
            .unwrap_or(in_format.clone());

        let log_level = if cli.verbose {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Error
        };

        Ok(Self {
            input_path: cli.input,
            output_path: cli.output,
            in_format,
            out_format,
            log_level,
        })
    }
}

type Extension<'a> = &'a str;

impl TryFrom<Extension<'_>> for Format {
    type Error = ();

    fn try_from(value: Extension<'_>) -> Result<Self, Self::Error> {
        match value {
            "csv" => Ok(Format::Csv),
            "json" => Ok(Format::Json),
            _ => Err(()),
        }
    }
}

fn get_format_from_extension(path: Option<&PathBuf>) -> Option<Format> {
    let ext = path?.extension()?;
    OsStr::to_str(ext)?.try_into().ok()
}
