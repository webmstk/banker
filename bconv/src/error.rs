use banker::BankError;

use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum BconvError {
    InputError(io::Error),
    OutputError(io::Error),
    AppError(BankError),
}

impl Error for BconvError {}

impl Display for BconvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BconvError::*;
        match self {
            InputError(err) => write!(f, "ошибка с input: {err}"),
            OutputError(err) => write!(f, "ошибка с output: {err}"),
            AppError(err) => write!(f, "ошибка конвертации: {err}"),
        }
    }
}

impl From<BankError> for BconvError {
    fn from(value: BankError) -> Self {
        BconvError::AppError(value)
    }
}
