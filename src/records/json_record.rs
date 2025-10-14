use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JsonRecord {
    pub name: String,
    pub balance: i32,
    pub bank_name: Option<String>,
}
