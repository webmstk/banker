//! Модуль предоставляет функционал для парсинга `csv` в структуру [CsvRecords].

use super::ParseError;
use crate::{CsvRecord, CsvRecords};

use std::io::Read;

/// Парсит в [CsvRecords] структуру, реализующую трейт [Read].
pub fn parse(reader: impl Read) -> Result<CsvRecords, ParseError> {
    let mut reader = csv::Reader::from_reader(reader);

    let mut records: Vec<CsvRecord> = Vec::new();
    for record in reader.deserialize() {
        records.push(record?);
    }

    Ok(records.into())
}

impl From<csv::Error> for ParseError {
    fn from(value: csv::Error) -> Self {
        ParseError::CsvParseError(value)
    }
}
