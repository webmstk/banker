use serde::Deserialize;

use super::JsonRecord;
use crate::parsers::{CsvParser, Parse, ParseError};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CsvRecord {
    pub name: String,
    pub balance: i32,
}

impl Parse<CsvRecord> for CsvRecord {
    fn parse(reader: impl std::io::BufRead) -> Result<Vec<CsvRecord>, ParseError> {
        Ok(CsvParser::parse(reader)?)
    }
}

impl From<JsonRecord> for CsvRecord {
    fn from(value: JsonRecord) -> Self {
        Self {
            name: value.name,
            balance: value.balance,
        }
    }
}
