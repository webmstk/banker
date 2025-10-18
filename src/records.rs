mod csv_record;
mod json_record;

use std::io::BufRead;

pub use csv_record::{CsvRecord, CsvRecords};
pub use json_record::{JsonRecord, JsonRecords};

use crate::parsers::ParseError;

pub trait Parse<T>: Sized {
    fn parse(reader: impl BufRead) -> Result<Self, ParseError>;
}
