use super::{Parse, ParseError};
use crate::JsonRecord;

use std::io::BufRead;

#[derive(Debug)]
pub struct JsonParser {}

impl Parse<JsonRecord> for JsonParser {
    fn parse(reader: impl BufRead) -> Result<Vec<JsonRecord>, ParseError> {
        Ok(serde_json::from_reader(reader)?)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::JsonParseError(value)
    }
}
