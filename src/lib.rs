mod parsers;
mod printers;
mod records;

pub use records::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};

use parsers::ParseError;
use records::{Parse, Print};

use std::io::{self, Read, Write};

pub fn parse<T>(reader: impl Read) -> Result<T, ParseError>
where
    T: Parse<T>,
{
    T::parse(reader)
}

pub fn convert_to<T1, T2>(records: T1) -> T2
where
    T1: Into<T2>,
{
    records.into()
}

pub fn print<T>(writer: impl Write, records: T) -> Result<(), io::Error>
where
    T: Print,
{
    records.print(writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use std::io::Cursor;

    #[test]
    fn parse_fn_successfuly_parses_valid_csv_input() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: CsvRecords = parse(input).unwrap();

        let expected = CsvRecord {
            name: "Vova".into(),
            balance: 100,
        };

        assert_eq!(records.list().len(), 1);
        assert_eq!(records.list().first().unwrap(), &expected);
    }

    #[test]
    fn parse_fn_fails_to_parse_invalid_csv_input() {
        let input = Cursor::new("full_name,balance\nVova,100");

        let err = parse::<CsvRecords>(input).err().unwrap();

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

        let records: JsonRecords = parse(input).unwrap();

        let expected = JsonRecord {
            name: "Petr".into(),
            balance: 300,
            bank_name: Some("central bank".into()),
        };

        assert_eq!(records.list().len(), 1);
        assert_eq!(records.list().first().unwrap(), &expected);
    }

    #[test]
    fn parse_fn_fails_to_parse_invalid_json_input() {
        let data = json!({
            "name": "Petr",
            "balance": 300,
            "bank_name": "central bank",
        });
        let input = Cursor::new(data.to_string());

        let err = parse::<JsonRecords>(input).err().unwrap();

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

        let records: JsonRecords = parse(input).unwrap();
        let csv_records: CsvRecords = convert_to(records);

        let expected = CsvRecord {
            name: "Petr".into(),
            balance: 300,
        };

        assert_eq!(csv_records.list().len(), 1);
        assert_eq!(csv_records.list().first().unwrap(), &expected);
    }

    #[test]
    fn convert_to_csv_fn_leave_csv_records_untouched() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: CsvRecords = parse(input).unwrap();
        let csv_records: CsvRecords = convert_to(records);

        let expected = CsvRecord {
            name: "Vova".into(),
            balance: 100,
        };

        assert_eq!(csv_records.list().len(), 1);
        assert_eq!(csv_records.list().first().unwrap(), &expected);
    }

    #[test]
    fn convert_to_json_fn_converts_csv_to_json() {
        let input = Cursor::new("name,balance\nVova,100");

        let records: CsvRecords = parse(input).unwrap();
        let json_records: JsonRecords = convert_to(records);

        let expected = JsonRecord {
            name: "Vova".into(),
            balance: 100,
            bank_name: None,
        };

        assert_eq!(json_records.list().len(), 1);
        assert_eq!(json_records.list().first().unwrap(), &expected);
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

        let records: JsonRecords = parse(input).unwrap();
        let json_records: JsonRecords = convert_to(records);

        let expected = JsonRecord {
            name: "Petr".into(),
            balance: 300,
            bank_name: Some("central bank".into()),
        };

        assert_eq!(json_records.list().len(), 1);
        assert_eq!(json_records.list().first().unwrap(), &expected);
    }

    #[test]
    fn print_fn_writes_csv_to_writer() {
        let record = CsvRecord {
            name: "Petr".into(),
            balance: 100,
        };

        let records: CsvRecords = vec![record].into();

        let mut buffer = Vec::new();
        print(&mut buffer, &records).unwrap();

        let expected = "name,balance\n\
                        Petr,100\n"
            .to_string();
        assert_eq!(buffer, expected.into_bytes());
    }

    #[test]
    fn print_fn_writes_json_to_writer() {
        let record = JsonRecord {
            name: "Petr".into(),
            balance: 100,
            bank_name: Some("Central Bank".into()),
        };

        let records: JsonRecords = vec![record].into();

        let mut buffer = Vec::new();
        print(&mut buffer, &records).unwrap();

        let expected = r#"[
  {
    "name": "Petr",
    "balance": 100,
    "bank_name": "Central Bank"
  }
]"#;

        assert_eq!(buffer, expected.as_bytes());
    }
}
