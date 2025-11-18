#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod records;

mod formats;
mod io;
mod parsers;
mod printers;

use error::BankError;
// use records::base_record::BaseRecord;
use records::{CsvRecords, JsonRecord, JsonRecords};
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
///     "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
///     1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial \"\"account\"\" funding\"",
/// );
///
/// let records: CsvRecords = parse(input).unwrap();
/// let record = records.list().first().unwrap();
///
/// assert_eq!(record.from_user_id, 0);
/// assert_eq!(record.to_user_id, 501);
/// assert_eq!(record.amount, 50000);
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
/// use banker::records::{Transaction, CsvRecords, JsonRecords};
/// use banker::records::base::{Status, TxType};
/// use chrono::DateTime;
///
/// let record = Transaction {
///     tx_id: 1001,
///     tx_type: TxType::Deposit,
///     from_user_id: 0,
///     to_user_id: 0,
///     amount: 50000,
///     timestamp: DateTime::from_timestamp_millis(1672531200000).unwrap(),
///     status: Status::Success,
///     description: "Initial \"account\" funding".into(),
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
/// use banker::records::{Transaction, CsvRecords};
/// use banker::records::base::{Status, TxType};
/// use chrono::DateTime;
///
/// let record = Transaction {
///     tx_id: 1001,
///     tx_type: TxType::Deposit,
///     from_user_id: 0,
///     to_user_id: 501,
///     amount: 50000,
///     timestamp: DateTime::from_timestamp_millis(1672531200000).unwrap(),
///     status: Status::Success,
///     description: "Initial \"account\" funding".into(),
/// };
/// let records: CsvRecords = vec![record].into();
///
/// let mut buffer = Vec::new();
/// print(&mut buffer, &records).unwrap();
///
/// let expected = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
///     1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial \"\"account\"\" funding\""
///     .to_string();
///
/// assert_eq!(String::from_utf8(buffer).unwrap(), expected);
pub fn print<T>(writer: impl Write, records: T) -> Result<(), BankError>
where
    T: Print,
{
    log::trace!("writing `T` to writer");
    Ok(records.print(writer)?)
}

#[cfg(test)]
mod tests;
