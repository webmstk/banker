//! Модуль предоставляет функционал для парсинга `csv` в структуру [CsvRecords].

use crate::csv::Records;
use crate::formats::text::*;
use crate::io::text::{self, *};
use crate::records::base::{Transaction, Validation};

use thiserror::Error;

use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

/// Ошибка, возникающая при парсинге csv-формата. Содержит в себе инфомрацию о том,
/// на какой строке возникла проблема и с каким полем (если есть).
#[derive(Error, Debug)]
pub struct CsvError {
    line: u32,
    field: Option<&'static str>,
    kind: CsvErrorKind,
}

impl CsvError {
    fn is_eof(&self) -> bool {
        if let CsvErrorKind::InvalidFormat(text::ParseError::UnexpectedEOF) = self.kind {
            return true;
        }
        false
    }
}

impl Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.field {
            Some(field) => {
                write!(f, "line: {}, field {}: {}", self.line, field, self.kind)
            }
            None => {
                write!(f, "line: {}: {}", self.line, self.kind)
            }
        }
    }
}

/// Ошибки, которые могу произойти при парсинге csv.
#[derive(Error, Debug)]
pub enum CsvErrorKind {
    /// Неизвестный заголовок.
    #[error("unexpected value '{0}'")]
    InvalidHeader(String),
    /// Ошибка возникающая при парсинге
    #[error("{0}")]
    InvalidFormat(#[from] text::ParseError),
    /// Ошибка при конвертировании строки в ожидаемый формат (в число, например).
    #[error("{0}")]
    ConversionError(String),
    /// Валидационная ошибка, например, если идентификатор транзакации оказался 0.
    #[error("{0}")]
    ValidationError(#[from] Validation),
}

/// Парсит в [CsvRecords] структуру, реализующую трейт [Read].
pub fn parse(reader: impl Read) -> Result<Records, CsvError> {
    let mut reader = BufReader::new(reader);

    // первая строка - это заголовки
    let mut records = Vec::new();
    read_headers(&mut reader)?;

    // начиная со второй строки - данные
    let mut line = 2;
    while let Some(record) = read_row(&mut reader, line)? {
        records.push(record);
        line += 1;
    }

    Ok(records.into())
}

fn read_headers(mut reader: impl BufRead) -> Result<(), CsvError> {
    read_header(TX_ID_HEADER, &mut reader)?;
    read_header(TX_TYPE_HEADER, &mut reader)?;
    read_header(FROM_USER_ID_HEADER, &mut reader)?;
    read_header(TO_USER_ID_HEADER, &mut reader)?;
    read_header(AMOUNT_HEADER, &mut reader)?;
    read_header(TIMESTAMP_HEADER, &mut reader)?;
    read_header(STATUS_HEADER, &mut reader)?;
    read_header(DESCRIPTION_HEADER, &mut reader)?;
    Ok(())
}

fn read_header(field: Field, mut reader: impl BufRead) -> Result<(), CsvError> {
    // хедеры всегда на первой строке
    let line = 1;

    let value: String = read_field(&mut reader, line, field)?;

    if value != field {
        return Err(CsvError {
            line,
            field: Some(field),
            kind: CsvErrorKind::InvalidHeader(value),
        });
    }

    Ok(())
}

fn read_row(mut reader: impl BufRead, line: u32) -> Result<Option<Transaction>, CsvError> {
    let tx_id: Result<u64, CsvError> = read_field(&mut reader, line, TX_TYPE_HEADER);
    // Первое поле новой строки проверяем, не пустое ли оно. Если пустое, то это валидная
    // ситуация, когда мы дочитали файл до конца.
    let tx_id = match tx_id {
        Ok(tx_id) => tx_id,
        Err(err) => {
            if err.is_eof() {
                return Ok(None);
            } else {
                return Err(err);
            }
        }
    };

    let tx_type: String = read_field(&mut reader, line, TX_TYPE_HEADER)?;
    let from_user_id: u64 = read_field(&mut reader, line, FROM_USER_ID_HEADER)?;
    let to_user_id: u64 = read_field(&mut reader, line, TO_USER_ID_HEADER)?;
    let amount: i64 = read_field(&mut reader, line, AMOUNT_HEADER)?;
    let timestamp: i64 = read_field(&mut reader, line, TIMESTAMP_HEADER)?;
    let status: String = read_field(&mut reader, line, STATUS_HEADER)?;
    let description: String = read_quoted_field(&mut reader, line, DESCRIPTION_HEADER)?;

    Ok(Some(
        Transaction::builder()
            .tx_id(tx_id)
            .tx_type(tx_type)
            .from_user_id(from_user_id)
            .to_user_id(to_user_id)
            .amount(amount)
            .timestamp(timestamp)
            .status(status)
            .description(description)
            .try_build()
            .map_err(|e| CsvError {
                line,
                field: None,
                kind: CsvErrorKind::ValidationError(e),
            })?,
    ))
}

fn read_field<T>(mut reader: impl BufRead, line: u32, field: Field) -> Result<T, CsvError>
where
    T: FromStr<Err: Display>,
{
    read_value_until(&mut reader, b',').map_err(|e| CsvError {
        line,
        field: Some(field),
        kind: e,
    })
}

fn read_quoted_field(
    mut reader: impl BufRead,
    line: u32,
    field: Field,
) -> Result<String, CsvError> {
    read_quoted(&mut reader).map_err(|e| CsvError {
        line,
        field: Some(field),
        kind: CsvErrorKind::InvalidFormat(e),
    })
}

fn read_value_until<T>(mut reader: impl BufRead, stop: u8) -> Result<T, CsvErrorKind>
where
    T: FromStr<Err: Display>,
{
    let raw_value = read_until(&mut reader, stop).map_err(|e| CsvErrorKind::InvalidFormat(e))?;
    parse_value(raw_value)
}

fn parse_value<T>(value: String) -> Result<T, CsvErrorKind>
where
    T: FromStr<Err: Display>,
{
    value
        .parse::<T>()
        .map_err(|e| CsvErrorKind::ConversionError(e.to_string()))
}

type Field = &'static str;
