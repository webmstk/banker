mod csv_records;
mod json_records;

pub use csv_records::{CsvRecord, CsvRecords};
pub use json_records::{JsonRecord, JsonRecords};

use crate::parsers::ParseError;

use std::io::{self, Read, Write};

pub trait Parse<T>: Sized {
    fn parse(reader: impl Read) -> Result<Self, ParseError>;
}

pub trait Print {
    fn print(&self, writer: impl Write) -> Result<(), io::Error>;
}
