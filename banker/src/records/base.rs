use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Валидационные ошибки полей [BaseRecord].
#[derive(Debug, Error)]
pub enum Validation {
    /// Идентификатор транзакции должен быть положительным.
    #[error("tx_id must be greater than 0")]
    TxIdMustBePositive,
    /// Неизвестный тип транзакции.
    #[error("unknown tx_type: {0}")]
    InvalidTxType(String),
    /// Неизвестный статус транзакции.
    #[error("unknown status: {0}")]
    InvalidStatus(String),
    /// Некорректная дата транзакции.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(i64),
    /// Не заполнены все поля в билдере [BaseRecordBuilder].
    #[error("some required fields are missing, cannot build BaseRecord")]
    FieldsMissing,
}

/// Базовая банковская операция, содержащая стандартные поля, которые могут быть общими
/// для разных форматов. Все учебные форматы (YPBankBinFormat, YPBankCsvFormat, YPBankTextFormat)
/// могут быть полностью сконвертированы в эту структуру. Однако существуют и другие форматы,
/// у которых может быть своя терминология и даже другой набор полей, поэтому данная структура
/// является внутренней только для совместимых форматов (например, форматы, указанные ранее).
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Transaction {
    /// Уникальный идентификатор транзакции
    pub tx_id: u64,
    /// Тип транзакции
    pub tx_type: TxType,
    /// Идентификатор пользователя-отправителя. Для системных пополнений ([TxType::Deposit]) может быть `0`
    pub from_user_id: u64,
    /// Идентификатор пользователя-получателя. Для системных списаний ([TxType::Withdrawal]`) может быть `0`
    pub to_user_id: u64,
    /// Сумма транзакции в наименьших единицах валюты (например, в центах)
    pub amount: i64,
    /// Время совершения транзакции в формате Unix-времени (миллисекунды с начала эпохи)
    pub timestamp: DateTime<Utc>,
    /// Статус транзакции
    pub status: Status,
    /// Текстовое описание транзакции
    pub description: String,
}

impl Transaction {
    pub(crate) fn builder() -> TransactionBuilder {
        TransactionBuilder::default()
    }
}

/// Билдер для транзакции. Требует заполнения всех полей.
#[derive(Default)]
pub struct TransactionBuilder {
    tx_id: Option<u64>,
    tx_type: Option<String>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<i64>,
    timestamp: Option<i64>,
    status: Option<String>,
    description: Option<String>,
}

impl TransactionBuilder {
    /// Сохраняет в билдер TX_ID.
    pub fn tx_id(mut self, value: u64) -> Self {
        self.tx_id = Some(value);
        self
    }

    /// Сохраняет в билдер TX_TYPE.
    pub fn tx_type(mut self, value: String) -> Self {
        self.tx_type = Some(value);
        self
    }

    /// Сохраняет в билдер FROM_USER_ID.
    pub fn from_user_id(mut self, value: u64) -> Self {
        self.from_user_id = Some(value);
        self
    }

    /// Сохраняет в билдер TO_USER_ID.
    pub fn to_user_id(mut self, value: u64) -> Self {
        self.to_user_id = Some(value);
        self
    }

    /// Сохраняет в билдер AMOUNT.
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    /// Сохраняет в билдер TIMESTAMP.
    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Сохраняет в билдер STATUS.
    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
        self
    }

    /// Сохраняет в билдер DESCRIPTION.
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// В результате билда получаем либо транзакцию [Transaction], либо
    /// валидационную ошибку [Validation]. Все поля должны быть заполнены.
    pub fn try_build(self) -> Result<Transaction, Validation> {
        self.validate_required()?;
        self.validate_tx_id()?;

        Ok(Transaction {
            tx_id: self.tx_id.unwrap(),
            tx_type: self.convert_tx_type()?,
            from_user_id: self.from_user_id.unwrap(),
            to_user_id: self.to_user_id.unwrap(),
            amount: self.amount.unwrap(),
            timestamp: self.convert_timestamp()?,
            status: self.convert_status()?,
            description: self.description.unwrap(),
        })
    }

    fn validate_required(&self) -> Result<(), Validation> {
        if self.tx_id.is_none()
            || self.tx_type.is_none()
            || self.from_user_id.is_none()
            || self.to_user_id.is_none()
            || self.amount.is_none()
            || self.timestamp.is_none()
            || self.status.is_none() && self.description.is_none()
        {
            return Err(Validation::FieldsMissing);
        }
        Ok(())
    }

    fn validate_tx_id(&self) -> Result<(), Validation> {
        if self.tx_id.unwrap() == 0 {
            return Err(Validation::TxIdMustBePositive);
        }
        Ok(())
    }

    fn convert_tx_type(&self) -> Result<TxType, Validation> {
        Ok(self.tx_type.as_ref().unwrap().try_into()?)
    }

    fn convert_timestamp(&self) -> Result<DateTime<Utc>, Validation> {
        Ok(DateTime::from_timestamp_millis(self.timestamp.unwrap())
            .ok_or(Validation::InvalidTimestamp(self.timestamp.unwrap()))?)
    }

    fn convert_status(&self) -> Result<Status, Validation> {
        Ok(self.status.as_ref().unwrap().try_into()?)
    }
}

/// Перечисление возможных вариантов транзакции.
#[derive(Debug, Clone, Copy, strum_macros::Display, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TxType {
    /// Пополнение
    #[serde(rename = "DEPOSIT")]
    #[strum(to_string = "DEPOSIT")]
    Deposit,
    /// Перевод
    #[serde(rename = "TRANSFER")]
    #[strum(to_string = "TRANSFER")]
    Transfer,
    /// Снятие
    #[serde(rename = "WITHDRAWAL")]
    #[strum(to_string = "WITHDRAWAL")]
    Withdrawal,
}

impl TryFrom<&String> for TxType {
    type Error = Validation;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        use TxType::*;
        match value.as_str() {
            "DEPOSIT" => Ok(Deposit),
            "TRANSFER" => Ok(Transfer),
            "WITHDRAWAL" => Ok(Withdrawal),
            _ => Err(Validation::InvalidTxType(value.clone())),
        }
    }
}

/// Перечисление возможных статусов транзакции.
#[derive(Debug, Clone, Copy, strum_macros::Display, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Status {
    /// Успешная транзакция
    #[serde(rename = "SUCCESS")]
    #[strum(to_string = "SUCCESS")]
    Success,
    /// Неуспешная транзакция
    #[serde(rename = "FAILURE")]
    #[strum(to_string = "FAILURE")]
    Failure,
    /// Новая транзакция
    #[serde(rename = "PENDING")]
    #[strum(to_string = "PENDING")]
    Pending,
}

impl TryFrom<&String> for Status {
    type Error = Validation;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        use Status::*;
        match value.as_str() {
            "SUCCESS" => Ok(Success),
            "FAILURE" => Ok(Failure),
            "PENDING" => Ok(Pending),
            _ => Err(Validation::InvalidStatus(value.clone())),
        }
    }
}
