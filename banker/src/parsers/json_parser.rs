//! Модуль предоставляет функционал для парсинга `json` в структуру [JsonRecords].

use super::ParseError;
use crate::{JsonRecord, JsonRecords};

use std::io::Read;

/// Парсит в [JsonRecords] структуру, реализующую трейт [Read].
pub fn parse(reader: impl Read) -> Result<JsonRecords, ParseError> {
    let records: Vec<JsonRecord> = serde_json::from_reader(reader)?;
    Ok(records.into())
}
