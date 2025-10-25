use crate::parse;
use crate::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};

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

    let expected = "не получилось распарсить вашу фигню: \
                        CSV deserialize error: record 1 (line: 2, byte: 18): missing field `name`";
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

    let expected = "не получилось распарсить вашу фигню: \
                        invalid type: map, expected a sequence at line 1 column 1";
    assert_eq!(err.to_string(), expected);
}
