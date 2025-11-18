//! Модуль содержит функционал, связанный со списком операций в формате `csv` [CsvRecords].

use super::{Parse, Print};
use crate::Transaction;
use crate::json;
use crate::parse::ParseError;
use crate::parsers::csv_parser;
use crate::printers::csv_printer;

use std::io::{self, Read, Write};

/// Список банковских операций, представленных в формате `csv`.
#[derive(Debug)]
pub struct Records(Vec<Transaction>);

impl Records {
    /// Список отдельных транзакций
    pub fn list(&self) -> &Vec<Transaction> {
        &self.0
    }

    /// Деконструирует структуру на список транзакций
    pub fn into_parts(self) -> Vec<Transaction> {
        self.0
    }
}

impl From<Vec<Transaction>> for Records {
    fn from(value: Vec<Transaction>) -> Self {
        Self(value)
    }
}

impl From<json::Records> for Records {
    fn from(value: json::Records) -> Self {
        value
            .into_parts()
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<Transaction>>()
            .into()
    }
}

impl Parse<Records> for Records {
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(csv_parser::parse(reader)?)
    }
}

impl Print for Records {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}

impl Print for &Records {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}
