mod csv_parser;

pub use csv_parser::CsvParser;

use std::error::Error;
use std::fmt;
use std::io::BufRead;

pub trait Parse<T> {
    fn parse(reader: impl BufRead) -> Result<Vec<T>, ParseError>;
}

#[derive(Debug)]
pub enum ParseError {
    CsvParseError(csv::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::CsvParseError(error) => {
                write!(f, "{}", error.to_string())
            }
        }
    }
}

impl Error for ParseError {}
