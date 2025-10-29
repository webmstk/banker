//! Модуль содержит функционал, связанный со списком операций в формате `csv` [CsvRecords].

use super::{Parse, Print};
use crate::parsers::{ParseError, csv_parser};
use crate::printers::csv_printer;
use crate::{JsonRecord, JsonRecords};

use serde::{Deserialize, Serialize};

use std::io::{self, Read, Write};

/// Список банковских операций, представленных в формате `csv`.
#[derive(Debug)]
pub struct CsvRecords(Vec<CsvRecord>);

impl CsvRecords {
    /// Список отдельных транзакций
    pub fn list(&self) -> &Vec<CsvRecord> {
        &self.0
    }

    /// Деконструирует структуру на список транзакций
    pub fn into_parts(self) -> Vec<CsvRecord> {
        self.0
    }
}

impl From<Vec<CsvRecord>> for CsvRecords {
    fn from(value: Vec<CsvRecord>) -> Self {
        Self(value)
    }
}

impl From<JsonRecords> for CsvRecords {
    fn from(value: JsonRecords) -> Self {
        value
            .into_parts()
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<CsvRecord>>()
            .into()
    }
}

impl Parse<CsvRecords> for CsvRecords {
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(csv_parser::parse(reader)?)
    }
}

impl Print for &CsvRecords {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}

/// Банковская операция, представленная в формете `csv`.
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CsvRecord {
    /// Отправитель денег
    pub from_client: String,
    /// Банк отправителя денег
    pub from_bank: String,
    /// Получатель денег
    pub to_client: String,
    /// Банк получателя денег
    pub to_bank: String,
    /// Идентификатор транзакции
    pub transaction: String,
    /// Количество денег
    pub amount: f64,
    /// Дата транзакции
    pub date: String,
}

impl From<JsonRecord> for CsvRecord {
    fn from(json_record: JsonRecord) -> Self {
        Self {
            from_client: json_record.sender,
            from_bank: json_record.sender_bank,
            to_client: json_record.reciever,
            to_bank: json_record.reciever_bank,
            transaction: json_record.transaction_id,
            amount: json_record.quantity,
            date: json_record.date,
        }
    }
}
