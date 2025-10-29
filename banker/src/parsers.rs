//! Модуль для парсинга данных.

pub mod csv_parser;
pub mod json_parser;

use std::error::Error;

#[derive(Debug, strum_macros::Display)]
pub enum ParseError {
    #[strum(to_string = "{0}")]
    CsvParseError(csv::Error),
    #[strum(to_string = "{0}")]
    JsonParseError(serde_json::Error),
}

impl Error for ParseError {}
