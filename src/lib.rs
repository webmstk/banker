mod parsers;
mod records;

use parsers::{Parse, ParseError};
pub use records::{CsvRecord, JsonRecord};

use std::io::BufRead;

pub fn parse<T: Parse<T>>(input: impl BufRead) -> Result<Vec<T>, ParseError> {
    T::parse(input)
}

pub fn convert_to_csv<T>(records: Vec<T>) -> Vec<CsvRecord>
where
    T: Into<CsvRecord>,
{
    records.into_iter().map(|r| r.into()).collect()
}

pub fn convert_to_json<T>(records: Vec<T>) -> Vec<JsonRecord>
where
    T: Into<JsonRecord>,
{
    records.into_iter().map(|r| r.into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use std::io::Cursor;

    #[test]
    fn parse_fn_successfuly_parses_valid_csv_input() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: Vec<CsvRecord> = parse(input).unwrap();

        let expected = CsvRecord {
            name: "Vova".into(),
            balance: 100,
        };

        assert_eq!(records.len(), 1);
        assert_eq!(records.first().unwrap(), &expected);
    }

    #[test]
    fn parse_fn_fails_to_parse_invalid_csv_input() {
        let input = Cursor::new("full_name,balance\nVova,100");

        let err = parse::<CsvRecord>(input).err().unwrap();

        let expected = "CSV deserialize error: record 1 (line: 2, byte: 18): missing field `name`";
        assert_eq!(err.to_string(), expected);
    }

    #[test]
    fn parse_fn_successfuly_parses_valid_json_input() {
        let data = json!([
            {
                "name": "Petr",
                "balance": 300,
                "bank_name": "central bank",
            },
        ]);

        let input = Cursor::new(data.to_string());

        let records: Vec<JsonRecord> = parse(input).unwrap();

        let expected = JsonRecord {
            name: "Petr".into(),
            balance: 300,
            bank_name: Some("central bank".into()),
        };

        assert_eq!(records.len(), 1);
        assert_eq!(records.first().unwrap(), &expected);
    }

    #[test]
    fn parse_fn_fails_to_parse_invalid_json_input() {
        let data = json!({
            "name": "Petr",
            "balance": 300,
            "bank_name": "central bank",
        });
        let input = Cursor::new(data.to_string());

        let err = parse::<JsonRecord>(input).err().unwrap();

        let expected = "invalid type: map, expected a sequence at line 1 column 1";
        assert_eq!(err.to_string(), expected);
    }

    #[test]
    fn convert_to_csv_fn_converts_json_to_csv() {
        let data = json!([
            {
                "name": "Petr",
                "balance": 300,
                "bank_name": "central bank",
            },
        ]);

        let input = Cursor::new(data.to_string());

        let records: Vec<JsonRecord> = parse(input).unwrap();
        let csv_records = convert_to_csv(records);

        let expected = CsvRecord {
            name: "Petr".into(),
            balance: 300,
        };

        assert_eq!(csv_records.len(), 1);
        assert_eq!(csv_records.first().unwrap(), &expected);
    }

    #[test]
    fn convert_to_csv_fn_leave_csv_records_untouched() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: Vec<CsvRecord> = parse(input).unwrap();
        let csv_records = convert_to_csv(records);

        let expected = CsvRecord {
            name: "Vova".into(),
            balance: 100,
        };

        assert_eq!(csv_records.len(), 1);
        assert_eq!(csv_records.first().unwrap(), &expected);
    }

    #[test]
    fn convert_to_json_fn_converts_csv_to_json() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: Vec<CsvRecord> = parse(input).unwrap();
        let json_records = convert_to_json(records);

        let expected = JsonRecord {
            name: "Vova".into(),
            balance: 100,
            bank_name: None,
        };

        assert_eq!(json_records.len(), 1);
        assert_eq!(json_records.first().unwrap(), &expected);
    }

    #[test]
    fn convert_to_json_fn_leave_json_records_untouched() {
        let data = json!([
            {
                "name": "Petr",
                "balance": 300,
                "bank_name": "central bank",
            },
        ]);

        let input = Cursor::new(data.to_string());

        let records: Vec<JsonRecord> = parse(input).unwrap();
        let json_records = convert_to_json(records);

        let expected = JsonRecord {
            name: "Petr".into(),
            balance: 300,
            bank_name: Some("central bank".into()),
        };

        assert_eq!(json_records.len(), 1);
        assert_eq!(json_records.first().unwrap(), &expected);
    }
}
