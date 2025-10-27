# Banker
Библиотека для чтения банковских операций в разных форматах.

Может читать.

Может конвертировать.

Может писать.


#### Пример использования

```
use banker::{convert_to, parse, print};
use banker::records::{CsvRecords, JsonRecords};
use std::io::Cursor;

let input = Cursor::new("name,balance\nPetr,100");

// Распарсили данные в формате `csv`
let csv_records: CsvRecords = parse(input).unwrap();
let record = csv_records.list().first().unwrap();

assert_eq!(record.name, "Petr");
assert_eq!(record.balance, 100);

// Сконвертировали в формат `json`
let json_records: JsonRecords = convert_to(csv_records);

let mut buffer = Vec::new();

// Записали результат в буфер
print(&mut buffer, &json_records).unwrap();

let expected = r#"[
  {
    "name": "Petr",
    "balance": 100,
    "bank_name": null
  }
]"#;

            
assert_eq!(buffer, expected.as_bytes());
```

## Roadmap
- camt 053
- mt 940