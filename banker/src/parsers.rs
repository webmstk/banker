pub mod csv_parser;
pub mod json_parser;

use std::error::Error;
use std::fmt;

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
