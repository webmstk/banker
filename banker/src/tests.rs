mod convert_tests;
mod parse_tests;
mod print_tests;

use crate::{CsvRecord, JsonRecord};
use std::io::Cursor;

use serde_json::json;

fn sample_json_record() -> JsonRecord {
    JsonRecord {
        sender: "Alice".into(),
        sender_bank: "bank_a".into(),
        reciever: "Bob".into(),
        reciever_bank: "bank_b".into(),
        transaction_id: "123".into(),
        quantity: 500.05,
        date: "24-01-2025".into(),
    }
}

fn sample_json_data() -> Cursor<String> {
    let data = json!([
        {
            "sender": "Alice",
            "sender_bank": "bank_a",
            "reciever": "Bob",
            "reciever_bank": "bank_b",
            "transaction_id": "123",
            "quantity": 500.05,
            "date": "24-01-2025",
        },
    ]);
    Cursor::new(data.to_string())
}

fn sample_csv_record() -> CsvRecord {
    CsvRecord {
        from_client: "Alice".into(),
        from_bank: "bank_a".into(),
        to_client: "Bob".into(),
        to_bank: "bank_b".into(),
        transaction: "123".into(),
        amount: 500.05,
        date: "24-01-2025".into(),
    }
}

fn sample_csv_data() -> Cursor<&'static str> {
    Cursor::new(
        "from_client,from_bank,to_client,to_bank,transaction,amount,date\n\
        Alice,bank_a,Bob,bank_b,123,500.05,24-01-2025\n",
    )
}
