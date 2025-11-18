//! Модуль содержащий структуры, представляющие данные в разных форматах.

/// Содержит внутреннюю структуру, описывающую транзакцию, которая может быть использована
/// в совместимых с ней форматоах.
pub mod base;
pub mod csv;
pub mod json;

use crate::parse::ParseError;

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
