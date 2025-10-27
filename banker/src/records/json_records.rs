//! Модуль содержит функционал, связанный со списком операций в формате `json` [JsonRecords].

use super::{Parse, Print};
use crate::parsers::{ParseError, json_parser};
use crate::printers::json_printer;
use crate::{CsvRecord, CsvRecords};

use serde::{Deserialize, Serialize};

use std::io::{self, Read, Write};

/// Список банковских операций, представленных в формате `json`.
#[derive(Debug)]
pub struct JsonRecords(Vec<JsonRecord>);

impl JsonRecords {
    pub fn list(&self) -> &Vec<JsonRecord> {
        &self.0
    }

    pub fn into_parts(self) -> Vec<JsonRecord> {
        self.0
    }
}

impl From<Vec<JsonRecord>> for JsonRecords {
    fn from(value: Vec<JsonRecord>) -> Self {
        Self(value)
    }
}

impl From<CsvRecords> for JsonRecords {
    fn from(value: CsvRecords) -> Self {
        value
            .into_parts()
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<JsonRecord>>()
            .into()
    }
}

impl Parse<JsonRecords> for JsonRecords {
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(json_parser::parse(reader)?)
    }
}

impl Print for &JsonRecords {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        json_printer::print(writer, self)
    }
}

/// Банковская операция, представленная в формете `json`.
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonRecord {
    pub sender: String,
    pub sender_bank: String,
    pub reciever: String,
    pub reciever_bank: String,
    pub transaction_id: String,
    pub quantity: f64,
    pub date: String,
}

impl From<CsvRecord> for JsonRecord {
    fn from(csv_record: CsvRecord) -> Self {
        Self {
            sender: csv_record.from_client,
            sender_bank: csv_record.from_bank,
            reciever: csv_record.to_client,
            reciever_bank: csv_record.to_bank,
            transaction_id: csv_record.transaction,
            quantity: csv_record.amount,
            date: csv_record.date,
        }
    }
}
