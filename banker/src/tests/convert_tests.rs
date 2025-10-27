use super::*;
use crate::{CsvRecords, JsonRecords};
use crate::{convert_to, parse};

#[test]
fn convert_to_csv_fn_converts_json_to_csv() {
    let data = sample_json_data();

    let records: JsonRecords = parse(data).unwrap();
    let csv_records: CsvRecords = convert_to(records);

    let expected = sample_csv_record();

    assert_eq!(csv_records.list().len(), 1);
    assert_eq!(csv_records.list().first().unwrap(), &expected);
}

#[test]
fn convert_to_csv_fn_leave_csv_records_untouched() {
    let data = sample_csv_data();

    let records: CsvRecords = parse(data).unwrap();
    let csv_records: CsvRecords = convert_to(records);

    let expected = sample_csv_record();

    assert_eq!(csv_records.list().len(), 1);
    assert_eq!(csv_records.list().first().unwrap(), &expected);
}

#[test]
fn convert_to_json_fn_converts_csv_to_json() {
    let data = sample_csv_data();

    let records: CsvRecords = parse(data).unwrap();
    let json_records: JsonRecords = convert_to(records);

    let expected = sample_json_record();

    assert_eq!(json_records.list().len(), 1);
    assert_eq!(json_records.list().first().unwrap(), &expected);
}

#[test]
fn convert_to_json_fn_leave_json_records_untouched() {
    let data = sample_json_data();

    let records: JsonRecords = parse(data).unwrap();
    let json_records: JsonRecords = convert_to(records);

    let expected = sample_json_record();

    assert_eq!(json_records.list().len(), 1);
    assert_eq!(json_records.list().first().unwrap(), &expected);
}
