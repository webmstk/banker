//! Модуль описывает ошибки библиотеки.
use crate::parsers::ParseError;

use std::error::Error;
use std::fmt::Display;
use std::io;

/// Перечисление ошибок, которые могу возникнуть в программе.
#[derive(Debug)]
pub enum BankError {
    /// Ошибка парсинга данных.
    ParseError(ParseError),
    /// Ошибка записи данных.
    PrintError(io::Error),
}

impl Error for BankError {}

impl Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BankError::*;
        match self {
            ParseError(err) => write!(f, "не получилось распарсить вашу фигню: {}", err),
            PrintError(err) => write!(f, "не получилось сохранить результат: {}", err),
        }
    }
}

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
