//! Модуль предоставляет функционал для записи [CsvRecords].

use crate::CsvRecords;

use std::io::{self, Write};

/// Записывает [CsvRecords] в структуру, реализующую трейт [Write].
pub fn print(writer: impl Write, records: &CsvRecords) -> Result<(), io::Error> {
    let mut csv_writer = csv::Writer::from_writer(writer);

    for record in records.list() {
        csv_writer.serialize(record)?;
    }

    Ok(())
}
