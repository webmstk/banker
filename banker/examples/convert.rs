use banker::{convert_to, parse, print};
use banker::{csv, json};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("../samples/data.csv");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Распарсили данные в формате `csv`
    let csv_records: csv::Records = parse(reader).unwrap();

    // Сконвертировали в формат `json`
    let json_records: json::Records = convert_to(csv_records);

    let writer = std::io::stdout().lock();

    // Записали результат в stdout
    print(writer, &json_records).unwrap();
}
