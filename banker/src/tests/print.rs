use crate::print;
use crate::{CsvRecord, CsvRecords, JsonRecord, JsonRecords};

use std::io::{self, Write};

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

    let record = JsonRecord {
        name: "Petr".into(),
        balance: 100,
        bank_name: None,
    };

    let records: JsonRecords = vec![record].into();

    let mut buffer = TestWriter {};
    let err = print(&mut buffer, &records).err().unwrap();

    let expected = "не получилось сохранить результат: boom";
    assert_eq!(err.to_string(), expected);
}
