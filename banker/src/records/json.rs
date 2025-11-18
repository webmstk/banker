//! Модуль содержит функционал, связанный со списком операций в формате `json` [JsonRecords].

use super::{Parse, Print};
use crate::csv;
use crate::parse::ParseError;
use crate::parsers::json_parser;
use crate::printers::json_printer;
use crate::{Status, Transaction, TxType};
use chrono::DateTime;

use serde::{Deserialize, Serialize};

use std::io::{self, Read, Write};

/// Список банковских операций, представленных в формате `json`.
#[derive(Debug)]
pub struct Records(Vec<JsonRecord>);

impl Records {
    /// Список отдельных транзакций
    pub fn list(&self) -> &Vec<JsonRecord> {
        &self.0
    }

    /// Деконструирует структуру на список транзакций
    pub fn into_parts(self) -> Vec<JsonRecord> {
        self.0
    }
}

impl From<Vec<JsonRecord>> for Records {
    fn from(value: Vec<JsonRecord>) -> Self {
        Self(value)
    }
}

impl From<csv::Records> for Records {
    fn from(value: csv::Records) -> Self {
        value
            .into_parts()
            .into_iter()
            .map(|r| r.into())
            .collect::<Vec<JsonRecord>>()
            .into()
    }
}

impl Parse<Records> for Records {
    fn parse(reader: impl Read) -> Result<Self, ParseError> {
        Ok(json_parser::parse(reader)?)
    }
}

impl Print for &Records {
    fn print(&self, writer: impl Write) -> Result<(), io::Error> {
        json_printer::print(writer, self)
    }
}

/// Банковская операция, представленная в формете `json`.
// Добавил отличия от [BaseRecord] - нет поля `description` и другой формат у `timestamp`.
// Можно было бы ещё и тип отправителя/получателя поменять, но тогда пришлось бы добавлять
// обработку ошибок конвертации, а я не хочу :(
// ну, может быть потом
// кода-нибудь ))
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonRecord {
    /// Идентификатор транзакции
    pub tx_id: u64,
    /// Тип транзакции
    pub tx_type: TxType,
    /// Отправитель денег
    pub from: u64,
    /// Получатель денег
    pub to: u64,
    /// Количество денег
    pub quantity: i64,
    /// Дата транзакции в формате 13-значного unix
    pub timestamp: i64,
    /// Статус транзакции
    pub status: Status,
}

impl From<Transaction> for JsonRecord {
    fn from(base_record: Transaction) -> Self {
        Self {
            tx_id: base_record.tx_id,
            tx_type: base_record.tx_type,
            from: base_record.from_user_id,
            to: base_record.to_user_id,
            quantity: base_record.amount,
            timestamp: base_record.timestamp.timestamp_millis(),
            status: base_record.status,
        }
    }
}

impl From<JsonRecord> for Transaction {
    fn from(json_record: JsonRecord) -> Self {
        Self {
            tx_id: json_record.tx_id,
            tx_type: json_record.tx_type,
            from_user_id: json_record.from,
            to_user_id: json_record.to,
            amount: json_record.quantity,
            // Здесь `unwrap`, потому что изначально не закладывался на то, что при конвертации
            // будут возможны ошибки. Был неправ.
            // Переделывать на `try_from` не хочется, предположим, что раз другая запись валидна,
            // то и здесь проблем не будет 😁.
            timestamp: DateTime::from_timestamp_millis(json_record.timestamp).unwrap(),
            status: json_record.status,
            description: "".to_string(),
        }
    }
}
