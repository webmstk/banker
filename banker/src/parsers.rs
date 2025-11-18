//! Модуль для парсинга данных.

pub mod csv_parser;
pub mod json_parser;

use crate::parsers::csv_parser::ParseCsvError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    Csv(#[from] ParseCsvError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
