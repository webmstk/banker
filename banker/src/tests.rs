mod convert_tests;
mod parse_tests;
mod print_tests;

use crate::json::JsonRecord;
use crate::records::base::{Status, Transaction, TxType};
use std::io::Cursor;

use chrono::DateTime;
use serde_json::json;

fn sample_json_record() -> JsonRecord {
    JsonRecord {
        tx_id: 1001,
        tx_type: TxType::Deposit,
        from: 0,
        to: 501,
        quantity: 50_000,
        timestamp: 1672531200000,
        status: Status::Success,
    }
}

fn sample_json_data() -> Cursor<String> {
    let data = json!([
        {
            "tx_id": 1001,
            "tx_type": "DEPOSIT",
            "from": 0,
            "to": 501,
            "quantity": 50_000,
            "timestamp": 1672531200000_i64,
            "status": "SUCCESS"
        },
    ]);
    Cursor::new(data.to_string())
}

fn sample_base_record() -> Transaction {
    Transaction {
        tx_id: 1001,
        tx_type: TxType::Deposit,
        from_user_id: 0,
        to_user_id: 501,
        amount: 50_000,
        timestamp: DateTime::from_timestamp_millis(1672531200000).unwrap(),
        status: Status::Success,
        description: "Initial \"account\" funding".into(),
    }
}

fn sample_csv_data() -> Cursor<&'static str> {
    Cursor::new(
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
        1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial \"\"account\"\" funding\"",
    )
}
