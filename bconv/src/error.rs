use banker::error::BankError;

use std::error::Error;
use std::io;

#[derive(Debug, strum_macros::Display)]
pub enum BconvError {
    #[strum(to_string = "ошибка с input: {0}")]
    InputError(io::Error),
    #[strum(to_string = "ошибка с output: {0}")]
    OutputError(io::Error),
    #[strum(to_string = "ошибка конвертации: {0}")]
    AppError(BankError),
}

impl Error for BconvError {}

impl From<BankError> for BconvError {
    fn from(value: BankError) -> Self {
        BconvError::AppError(value)
    }
}
