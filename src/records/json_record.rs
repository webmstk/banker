use serde::Deserialize;

use super::CsvRecord;
use crate::parsers::{JsonParser, Parse, ParseError};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonRecord {
    pub name: String,
    pub balance: i32,
    pub bank_name: Option<String>,
}

impl Parse<JsonRecord> for JsonRecord {
    fn parse(reader: impl std::io::BufRead) -> Result<Vec<JsonRecord>, ParseError> {
        Ok(JsonParser::parse(reader)?)
    }
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
