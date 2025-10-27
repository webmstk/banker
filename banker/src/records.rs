//! Модуль содержащий структуры, представляющие данные в разных форматах.

mod csv_records;
mod json_records;

pub use csv_records::{CsvRecord, CsvRecords};
pub use json_records::{JsonRecord, JsonRecords};

use crate::parsers::ParseError;

use std::io::{self, Read, Write};

/// Трейт для парсинга данных из переданного источника в новую структуру.
pub trait Parse<T>: Sized {
    /// Парсит данные, возвращая новую структуру или ошибку.
    fn parse(reader: impl Read) -> Result<Self, ParseError>;
}

/// Трейт для печати банковских операций в источник вывода.
pub trait Print {
    /// Записывает данные в источник вывода.
    fn print(&self, writer: impl Write) -> Result<(), io::Error>;
}
