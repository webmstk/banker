use super::{CsvRecord, CsvRecords};
use crate::Parse;
use crate::parsers::ParseError;
use crate::parsers::json_parser;

use serde::Deserialize;

use std::io::BufRead;

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
    fn parse(reader: impl BufRead) -> Result<Self, ParseError> {
        Ok(json_parser::parse(reader)?)
    }
}

#[derive(Debug, Deserialize)]
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
