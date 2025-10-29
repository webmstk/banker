//! Модуль описывает ошибки библиотеки.
use crate::parsers::ParseError;

use std::error::Error;
use std::io;

/// Перечисление ошибок, которые могу возникнуть в программе.
#[derive(Debug, strum_macros::Display)]
pub enum BankError {
    /// Ошибка парсинга данных.
    #[strum(to_string = "не получилось распарсить вашу фигню: {0}")]
    ParseError(ParseError),
    /// Ошибка записи данных.
    #[strum(to_string = "не получилось сохранить результат: {0}")]
    PrintError(io::Error),
}

impl Error for BankError {}

impl From<ParseError> for BankError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<io::Error> for BankError {
    fn from(value: io::Error) -> Self {
        Self::PrintError(value)
    }
}
