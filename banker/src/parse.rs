//! Модуль для публичного экспорта ошибок парсинга.

pub use crate::parsers::csv_parser::{CsvError, CsvErrorKind};

use thiserror::Error;

/// Общее перечеслиние возможных ошибок парсинга разных форматов.
#[derive(Error, Debug)]
pub enum ParseError {
    /// csv формат.
    #[error("{0}")]
    Csv(#[from] CsvError),
    /// json формат.
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
