mod csv_parser;
mod json_parser;

pub use csv_parser::CsvParser;
pub use json_parser::JsonParser;

use std::error::Error;
use std::fmt;
use std::io::BufRead;

pub trait Parse<T> {
    fn parse(reader: impl BufRead) -> Result<Vec<T>, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {
    CsvParseError(csv::Error),
    JsonParseError(serde_json::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseError::*;

        match self {
            CsvParseError(error) => {
                write!(f, "{}", error.to_string())
            }
            JsonParseError(error) => {
                write!(f, "{}", error.to_string())
            }
        }
    }
}

impl Error for ParseError {}
