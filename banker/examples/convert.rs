use banker::records::{CsvRecords, JsonRecords};
use banker::{convert_to, parse, print};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("samples/data.csv");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Распарсили данные в формате `csv`
    let csv_records: CsvRecords = parse(reader).unwrap();

    // Сконвертировали в формат `json`
    let json_records: JsonRecords = convert_to(csv_records);

    let writer = std::io::stdout().lock();

    // Записали результат в stdout
    print(writer, &json_records).unwrap();
}
