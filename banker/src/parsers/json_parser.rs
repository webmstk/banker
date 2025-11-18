//! Модуль предоставляет функционал для парсинга `json` в структуру [JsonRecords].

use crate::json::{JsonRecord, Records};
use crate::parse::ParseError;

use std::io::Read;

/// Парсит в [JsonRecords] структуру, реализующую трейт [Read].
pub fn parse(reader: impl Read) -> Result<Records, ParseError> {
    let records: Vec<JsonRecord> = serde_json::from_reader(reader)?;
    Ok(records.into())
}
