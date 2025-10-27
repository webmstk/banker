//! Библиотка для чтения банковских операций в разных форматах.
//! Может читать.
//! Может конвертировать.
//! Может писать.
//! Поднимает настроение и тонус, делает волосы гладкими и шелковистыми!

mod error;
mod parsers;
mod printers;
mod records;

pub use error::BankError;
pub use records::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};
pub use records::{Parse, Print};

use std::io::{Read, Write};

/// Парсит данные в обобщённый тип `Т`. В качестве таких типов могут
/// выступать структуры из модуля [records].
///
/// # Пример
///
/// ```
/// use banker::{parse, CsvRecords};
/// use std::io::Cursor;
///
/// let input = Cursor::new("name,balance\nVova,100");
///
/// let records: CsvRecords = parse(input).unwrap();
/// let record = records.list().first().unwrap();
///
/// assert_eq!(record.name, "Vova");
/// assert_eq!(record.balance, 100);
pub fn parse<T>(reader: impl Read) -> Result<T, BankError>
where
    T: Parse<T>,
{
    Ok(T::parse(reader)?)
}

/// Конвертирует записи в другой формат. Структуры из модуля [records]
/// можно конвертировать друг в друга.
///
/// # Пример
///
/// ```
/// use banker::{convert_to, CsvRecord, CsvRecords, JsonRecords};
///
/// let record = CsvRecord {
///     name: "Vova".into(),
///     balance: 400,
/// };
/// let csv: CsvRecords = vec![record].into();
///
/// let json: JsonRecords = convert_to(csv);
pub fn convert_to<T1, T2>(records: T1) -> T2
where
    T1: Into<T2>,
{
    records.into()
}

/// Записывает банковские операции в требуемом формате в источник вывода.
///
/// # Пример
///
/// ```
/// use banker::{print, CsvRecord, CsvRecords};
///
/// let record = CsvRecord {
///     name: "Petr".into(),
///     balance: 200,
/// };
/// let records: CsvRecords = vec![record].into();
///
/// let mut buffer = Vec::new();
/// print(&mut buffer, &records).unwrap();
///
/// let expected = "name,balance\n\
///                     Petr,200\n"
///     .to_string();
/// assert_eq!(buffer, expected.into_bytes());
pub fn print<T>(writer: impl Write, records: T) -> Result<(), BankError>
where
    T: Print,
{
    Ok(records.print(writer)?)
}

#[cfg(test)]
mod tests;
