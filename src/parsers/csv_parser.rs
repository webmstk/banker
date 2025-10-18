use super::ParseError;
use crate::{CsvRecord, CsvRecords};

use std::io::BufRead;

pub fn parse(reader: impl BufRead) -> Result<CsvRecords, ParseError> {
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
