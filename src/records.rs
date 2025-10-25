mod csv_record;
mod json_record;

pub use csv_record::{CsvRecord, CsvRecords};
pub use json_record::{JsonRecord, JsonRecords};

use crate::parsers::ParseError;

use std::io::{self, Read, Write};

pub trait Parse<T>: Sized {
    fn parse(reader: impl Read) -> Result<Self, ParseError>;
}

pub trait Print {
    fn print(&self, writer: impl Write) -> Result<(), io::Error>;
}
