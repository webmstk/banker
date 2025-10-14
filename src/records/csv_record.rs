use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CsvRecord {
    pub name: String,
    pub balance: i32,
}
