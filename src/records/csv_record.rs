use super::{Parse, Print};
use crate::parsers::{ParseError, csv_parser};
use crate::printers::csv_printer;
use crate::{JsonRecord, JsonRecords};

use serde::{Deserialize, Serialize};

use std::io::{self, Read, Write};

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
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(csv_parser::parse(reader)?)
    }
}

impl Print for &CsvRecords {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        csv_printer::print(writer, self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
