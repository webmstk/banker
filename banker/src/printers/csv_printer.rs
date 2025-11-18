//! Модуль предоставляет функционал для записи [CsvRecords].

use crate::csv::Records;
use crate::formats::text::*;
use crate::io::text::*;
use crate::records::base::Transaction;

use chrono::prelude::{DateTime, Utc};
use std::io::{self, Write};

/// Записывает [CsvRecords] в структуру, реализующую трейт [Write].
pub fn print(mut writer: impl Write, csv: &Records) -> io::Result<()> {
    writeln!(writer, "{}", headers())?;

    let mut records = csv.list().iter().peekable();
    while let Some(record) = records.next() {
        write_row(&mut writer, &record)?;
        if records.peek().is_some() {
            writeln!(writer, "")?;
        }
    }

    Ok(())
}

fn headers() -> String {
    [
        TX_ID_HEADER,
        TX_TYPE_HEADER,
        FROM_USER_ID_HEADER,
        TO_USER_ID_HEADER,
        AMOUNT_HEADER,
        TIMESTAMP_HEADER,
        STATUS_HEADER,
        DESCRIPTION_HEADER,
    ]
    .join(",")
}

fn write_row(mut writer: impl Write, record: &Transaction) -> io::Result<()> {
    write!(writer, "{},", &record.tx_id)?;
    write!(writer, "{},", record.tx_type)?;
    write!(writer, "{},", &record.from_user_id)?;
    write!(writer, "{},", &record.to_user_id)?;
    write!(writer, "{},", record.amount)?;
    write!(writer, "{},", format_timestamp(&record.timestamp))?;
    write!(writer, "{},", record.status)?;
    write_quoted(&mut writer, &record.description)?;
    Ok(())
}

fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
    format!("{}", timestamp.timestamp_millis())
}
