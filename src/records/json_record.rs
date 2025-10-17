use serde::Deserialize;

use super::CsvRecord;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonRecord {
    pub name: String,
    pub balance: i32,
    pub bank_name: Option<String>,
}

impl From<CsvRecord> for JsonRecord {
    fn from(value: CsvRecord) -> Self {
        Self {
            name: value.name,
            balance: value.balance,
            bank_name: None,
        }
    }
}
