mod error;
mod parsers;
mod printers;
mod records;

pub use error::BankError;
pub use records::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};
pub use records::{Parse, Print};

use std::io::{Read, Write};

pub fn parse<T>(reader: impl Read) -> Result<T, BankError>
where
    T: Parse<T>,
{
    Ok(T::parse(reader)?)
}

pub fn convert_to<T1, T2>(records: T1) -> T2
where
    T1: Into<T2>,
{
    records.into()
}

pub fn print<T>(writer: impl Write, records: T) -> Result<(), BankError>
where
    T: Print,
{
    Ok(records.print(writer)?)
}

#[cfg(test)]
mod tests;
