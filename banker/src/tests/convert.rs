use crate::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};
use crate::{convert_to, parse};

use serde_json::json;
use std::io::Cursor;

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
