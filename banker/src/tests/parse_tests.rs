use super::*;
use crate::parse;
use crate::{csv, json};

#[test]
fn parse_fn_successfuly_parses_valid_csv_input() {
    let data = sample_csv_data();

    let records: csv::Records = parse(data).unwrap();

    let expected = sample_base_record();

    assert_eq!(records.list().len(), 1);
    assert_eq!(records.list().first().unwrap(), &expected);
}

#[test]
fn parse_fn_fails_to_parse_invalid_csv_input() {
    let data = Cursor::new("full_name,balance\nPetr,100");

    let err = parse::<csv::Records>(data).err().unwrap();

    let expected = "не получилось распарсить вашу фигню: \
        line: 1, field TX_ID: unexpected value 'full_name'";
    assert_eq!(err.to_string(), expected);
}

#[test]
fn parse_fn_successfuly_parses_valid_json_input() {
    let data = sample_json_data();

    let records: json::Records = parse(data).unwrap();

    let expected = sample_json_record();

    assert_eq!(records.list().len(), 1);
    assert_eq!(records.list().first().unwrap(), &expected);
}

#[test]
fn parse_fn_fails_to_parse_invalid_json_input() {
    let data = Cursor::new(
        json!({
            "name": "Petr",
            "balance": 300,
        })
        .to_string(),
    );

    let err = parse::<json::Records>(data).err().unwrap();

    let expected = "не получилось распарсить вашу фигню: \
                        invalid type: map, expected a sequence at line 1 column 1";
    assert_eq!(err.to_string(), expected);
}
