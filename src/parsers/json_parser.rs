use super::{Parse, ParseError};
use crate::JsonRecord;

use std::io::BufRead;

#[derive(Debug)]
pub struct JsonParser {}

impl Parse<JsonRecord> for JsonParser {
    fn parse(input: impl BufRead) -> Result<Vec<JsonRecord>, ParseError> {
        let records: Vec<JsonRecord> = serde_json::from_reader(input)?;
        Ok(records)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::JsonParseError(value)
    }
}
