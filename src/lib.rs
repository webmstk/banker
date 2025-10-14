mod parsers;
mod records;

use parsers::{CsvParser, JsonParser, Parse, ParseError};
use records::{CsvRecord, JsonRecord};

use std::io::BufRead;

pub fn read_csv(input: impl BufRead) -> Result<Vec<CsvRecord>, ParseError> {
    CsvParser::parse(input)
}

pub fn read_json(input: impl BufRead) -> Result<Vec<JsonRecord>, ParseError> {
    JsonParser::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
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

    #[test]
    fn read_json_fn_successfuly_parses_valid_input() {
        let data = json!([
            {
                "name": "Petr",
                "balance": 300,
                "bank_name": "central bank",
            },
        ]);

        let input = Cursor::new(data.to_string());

        let records = read_json(input).unwrap();

        let expected = JsonRecord {
            name: "Petr".into(),
            balance: 300,
            bank_name: Some("central bank".into()),
        };

        assert_eq!(records.len(), 1);
        assert_eq!(records.first().unwrap(), &expected);
    }

    #[test]
    fn read_json_fn_fails_to_parse_invalid_input() {
        let data = json!({
            "name": "Petr",
            "balance": 300,
            "bank_name": "central bank",
        });
        let input = Cursor::new(data.to_string());

        let err = read_json(input).err().unwrap();

        let expected = "invalid type: map, expected a sequence at line 1 column 1";
        assert_eq!(err.to_string(), expected);
    }
}
