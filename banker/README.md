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
    "from_client,from_bank,to_client,to_bank,transaction,amount,date\n\
    Alice,bank_a,Bob,bank_b,123,500.05,24-01-2025\n",
);

// Распарсили данные в формате `csv`
let csv_records: CsvRecords = parse(input).unwrap();
let record = csv_records.list().first().unwrap();

assert_eq!(record.from_client, "Alice");
assert_eq!(record.to_client, "Bob");
assert_eq!(record.amount, 500.05);

// Сконвертировали в формат `json`
let json_records: JsonRecords = convert_to(csv_records);

let mut buffer = Vec::new();

// Записали результат в буфер
print(&mut buffer, &json_records).unwrap();

let expected = r#"[
  {
    "sender": "Alice",
    "sender_bank": "bank_a",
    "reciever": "Bob",
    "reciever_bank": "bank_b",
    "transaction_id": "123",
    "quantity": 500.05,
    "date": "24-01-2025"
  }
]"#;

            
assert_eq!(buffer, expected.as_bytes());
```

## Roadmap
- camt 053
- mt 940
