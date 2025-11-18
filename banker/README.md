# Banker
Библиотека для чтения банковских операций в разных форматах.

Может читать.

Может конвертировать.

Может писать.

Это учебный проект. Не использовать!

#### Пример использования

```
use banker::{convert_to, parse, print};
use banker::records::{CsvRecords, JsonRecords};
use std::io::Cursor;

let input = Cursor::new(
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
    1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,\"Initial \"\"account\"\" funding\"",
);

// Распарсили данные в формате `csv`
let csv_records: CsvRecords = parse(input).unwrap();
let record = csv_records.list().first().unwrap();

assert_eq!(record.from_user_id, 0);
assert_eq!(record.to_user_id, 501);
assert_eq!(record.amount, 50000);

// Сконвертировали в формат `json`
let json_records: JsonRecords = convert_to(csv_records);

let mut buffer = Vec::new();

// Записали результат в буфер
print(&mut buffer, &json_records).unwrap();

let expected = r#"[
  {
    "tx_id": 1001,
    "tx_type": "DEPOSIT",
    "from": 0,
    "to": 501,
    "quantity": 50000,
    "timestamp": 1672531200000,
    "status": "SUCCESS"
  }
]"#;

assert_eq!(String::from_utf8(buffer).unwrap(), expected);
```
