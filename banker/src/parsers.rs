//! Модуль для парсинга данных.

pub mod csv_parser;
pub mod json_parser;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    CsvParseError(#[from] csv::Error),
    #[error("{0}")]
    JsonParseError(#[from] serde_json::Error),
}
