use serde::Deserialize;

use super::JsonRecord;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CsvRecord {
    pub name: String,
    pub balance: i32,
}

impl From<JsonRecord> for CsvRecord {
    fn from(value: JsonRecord) -> Self {
        Self {
            name: value.name,
            balance: value.balance,
        }
    }
}
