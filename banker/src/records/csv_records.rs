//! Модуль содержит функционал, связанный со списком операций в формате `csv` [CsvRecords].

use super::{Parse, Print};
use crate::parsers::{ParseError, csv_parser};
use crate::printers::csv_printer;
use crate::records::{JsonRecords, Transaction};

use std::io::{self, Read, Write};

/// Список банковских операций, представленных в формате `csv`.
#[derive(Debug)]
pub struct CsvRecords(Vec<Transaction>);

impl CsvRecords {
    /// Список отдельных транзакций
    pub fn list(&self) -> &Vec<Transaction> {
        &self.0
    }

    /// Деконструирует структуру на список транзакций
    pub fn into_parts(self) -> Vec<Transaction> {
        self.0
    }
}

impl From<Vec<Transaction>> for CsvRecords {
    fn from(value: Vec<Transaction>) -> Self {
        Self(value)
    }
}

impl From<JsonRecords> for CsvRecords {
    fn from(value: JsonRecords) -> Self {
        value
            .into_parts()
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<Transaction>>()
            .into()
    }
}

impl Parse<CsvRecords> for CsvRecords {
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(csv_parser::parse(reader)?)
    }
}

impl Print for CsvRecords {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}

impl Print for &CsvRecords {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}
