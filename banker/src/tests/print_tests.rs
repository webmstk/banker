use super::*;
use crate::print;
use crate::{CsvRecords, JsonRecords};

use std::io::{self, Read, Write};

#[test]
fn print_fn_writes_csv_to_writer() {
    let record = sample_base_record();

    let records: CsvRecords = vec![record].into();

    let mut buffer = Vec::new();
    print(&mut buffer, records).unwrap();

    let mut expected = String::new();
    sample_csv_data().read_to_string(&mut expected).unwrap();

    assert_eq!(buffer, expected.into_bytes());
}

#[test]
fn print_fn_writes_json_to_writer() {
    use serde_json::Value as Json;

    let record = sample_json_record();

    let records: JsonRecords = vec![record].into();

    let mut buffer = Vec::new();
    print(&mut buffer, &records).unwrap();

    let mut expected = String::new();
    sample_json_data().read_to_string(&mut expected).unwrap();

    let actual_json: Json = serde_json::from_slice(&buffer).unwrap();
    let expected_json: Json = serde_json::from_str(&expected).unwrap();

    assert_eq!(actual_json, expected_json);
}

#[test]
fn print_fn_returns_error_if_writer_fails() {
    struct TestWriter {}

    impl Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::PermissionDenied, "boom"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            todo!()
        }
    }

    let record = sample_json_record();

    let records: JsonRecords = vec![record].into();

    let mut buffer = TestWriter {};
    let err = print(&mut buffer, &records).err().unwrap();

    let expected = "не получилось сохранить результат: boom";
    assert_eq!(err.to_string(), expected);
}
