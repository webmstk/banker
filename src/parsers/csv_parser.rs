use super::{Parse, ParseError};
use crate::CsvRecord;

use std::io::BufRead;

#[derive(Debug)]
pub struct CsvParser {}

impl Parse<CsvRecord> for CsvParser {
    fn parse(input: impl BufRead) -> Result<Vec<CsvRecord>, ParseError> {
        let mut reader = csv::Reader::from_reader(input);

        let mut records = Vec::new();
        for record in reader.deserialize() {
            records.push(record?);
        }

        Ok(records)
    }
}

impl From<csv::Error> for ParseError {
    fn from(value: csv::Error) -> Self {
        ParseError::CsvParseError(value)
    }
}
