//! Модуль описывает ошибки библиотеки.

// Пришлось разрешить из-за impl_context!
#![allow(missing_docs)]

pub use crate::parse::ParseError;

use thiserror::Error;
use thiserror_context::{Context, impl_context};

use std::io;

/// Перечисление ошибок, которые могут возникнуть в программе.
#[derive(Error, Debug)]
pub enum BankErrorInner {
    /// Ошибка парсинга данных.
    #[error("не получилось распарсить вашу фигню: {0}")]
    ParseError(#[from] ParseError),
    /// Ошибка записи данных.
    #[error("не получилось сохранить результат: {0}")]
    PrintError(#[from] io::Error),
}

impl_context!(BankError(BankErrorInner));
