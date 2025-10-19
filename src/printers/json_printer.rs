use crate::JsonRecords;

use std::io::{self, Write};

pub fn print(writer: impl Write, records: &JsonRecords) -> Result<(), io::Error> {
    serde_json::to_writer_pretty(writer, records.list())?;
    Ok(())
}
