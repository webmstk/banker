use super::ParseError;
use crate::{JsonRecord, JsonRecords};

use std::io::BufRead;

pub fn parse(reader: impl BufRead) -> Result<JsonRecords, ParseError> {
    let records: Vec<JsonRecord> = serde_json::from_reader(reader)?;
    Ok(records.into())
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::JsonParseError(value)
    }
}
