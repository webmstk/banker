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
    pub name: String,
    pub balance: i32,
    pub bank_name: Option<String>,
}

impl From<CsvRecord> for JsonRecord {
    fn from(value: CsvRecord) -> Self {
        Self {
            name: value.name,
            balance: value.balance,
            bank_name: None,
        }
    }
}
