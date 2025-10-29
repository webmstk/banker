#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod records;

mod parsers;
mod printers;

use error::BankError;
use records::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};
use records::{Parse, Print};

use std::io::{Read, Write};

/// Парсит данные в обобщённый тип `Т`. В качестве таких типов могут
/// выступать структуры из модуля [records].
///
/// # Пример
///
/// ```
/// use banker::parse;
/// use banker::records::CsvRecords;
/// use std::io::Cursor;
///
/// let input = Cursor::new(
///     "from_client,from_bank,to_client,to_bank,transaction,amount,date\n\
///     Alice,bank_a,Bob,bank_b,123,500.05,24-01-2025\n",
/// );
///
/// let records: CsvRecords = parse(input).unwrap();
/// let record = records.list().first().unwrap();
///
/// assert_eq!(record.from_client, "Alice");
/// assert_eq!(record.to_client, "Bob");
/// assert_eq!(record.amount, 500.05);
pub fn parse<T>(reader: impl Read) -> Result<T, BankError>
where
    T: Parse<T>,
{
    log::trace!("parsing from reader to `T`");
    Ok(T::parse(reader)?)
}

/// Конвертирует записи в другой формат. Структуры из модуля [records]
/// можно конвертировать друг в друга.
///
/// # Пример
///
/// ```
/// use banker::convert_to;
/// use banker::records::{CsvRecord, CsvRecords, JsonRecords};
///
/// let record = CsvRecord {
///     from_client: "Alice".into(),
///     from_bank: "bank_a".into(),
///     to_client: "Bob".into(),
///     to_bank: "bank_b".into(),
///     transaction: "123".into(),
///     amount: 500.05,
///     date: "24-01-2025".into(),
/// };
/// let csv: CsvRecords = vec![record].into();
///
/// let json: JsonRecords = convert_to(csv);
pub fn convert_to<T1, T2>(records: T1) -> T2
where
    T1: Into<T2>,
{
    log::trace!("converting `T1` into `T2`");
    records.into()
}

/// Записывает банковские операции в требуемом формате в источник вывода.
///
/// # Пример
///
/// ```
/// use banker::print;
/// use banker::records::{CsvRecord, CsvRecords};
///
/// let record = CsvRecord {
///     from_client: "Alice".into(),
///     from_bank: "bank_a".into(),
///     to_client: "Bob".into(),
///     to_bank: "bank_b".into(),
///     transaction: "123".into(),
///     amount: 500.05,
///     date: "24-01-2025".into(),
/// };
/// let records: CsvRecords = vec![record].into();
///
/// let mut buffer = Vec::new();
/// print(&mut buffer, &records).unwrap();
///
/// let expected = "from_client,from_bank,to_client,to_bank,transaction,amount,date\n\
///     Alice,bank_a,Bob,bank_b,123,500.05,24-01-2025\n"
///     .to_string();
///
/// assert_eq!(buffer, expected.into_bytes());
pub fn print<T>(writer: impl Write, records: T) -> Result<(), BankError>
where
    T: Print,
{
    log::trace!("writing `T` to writer");
    Ok(records.print(writer)?)
}

#[cfg(test)]
mod tests;
