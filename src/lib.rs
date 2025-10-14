mod parsers;
mod records;

use parsers::{CsvParser, Parse, ParseError};
use records::CsvRecord;

use std::io::BufRead;

pub fn read_csv(input: impl BufRead) -> Result<Vec<CsvRecord>, ParseError> {
    CsvParser::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn read_csv_fn_successfuly_parses_valid_input() {
        let input = Cursor::new("name,balance\nVova,100");

        let records = read_csv(input).unwrap();

        let expected = CsvRecord {
            name: "Vova".into(),
            balance: 100,
        };

        assert_eq!(records.len(), 1);
        assert_eq!(records.first().unwrap(), &expected);
    }

    #[test]
    fn read_csv_fn_fails_to_parse_invalid_input() {
        let input = Cursor::new("full_name,balance\nVova,100");

        let err = read_csv(input).err().unwrap();

        let expected = "CSV deserialize error: record 1 (line: 2, byte: 18): missing field `name`";
        assert_eq!(err.to_string(), expected);
    }
}
