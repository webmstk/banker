use super::{JsonRecord, JsonRecords};
use crate::Parse;
use crate::parsers::ParseError;
use crate::parsers::csv_parser;

use serde::Deserialize;

use std::io::BufRead;

#[derive(Debug)]
pub struct CsvRecords(Vec<CsvRecord>);

impl CsvRecords {
    pub fn list(&self) -> &Vec<CsvRecord> {
        &self.0
    }

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
    fn parse(reader: impl BufRead) -> Result<Self, ParseError> {
        Ok(csv_parser::parse(reader)?)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CsvRecord {
    pub name: String,
    pub balance: i32,
}

impl From<JsonRecord> for CsvRecord {
    fn from(value: JsonRecord) -> Self {
        Self {
            name: value.name,
            balance: value.balance,
        }
    }
}
