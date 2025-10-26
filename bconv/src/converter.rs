use crate::cli::{Cli, Format};
use crate::error::BconvError;
use banker::{BankError, Parse, Print};
use banker::{CsvRecords, JsonRecords};
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, IsTerminal, Read, Write, stdin, stdout};
use std::path::Path;
use std::process::exit;

type Extension<'a> = &'a str;

impl TryFrom<Extension<'_>> for Format {
    type Error = ();

    fn try_from(value: Extension<'_>) -> Result<Self, Self::Error> {
        match value {
            "csv" => Ok(Format::Csv),
            "json" => Ok(Format::Json),
            _ => Err(()),
        }
    }
}

pub fn run(cfg: Cli) -> Result<(), BconvError> {
    let input = cfg.input.as_deref();
    let output = cfg.output.as_deref();

    let reader = get_reader(input).map_err(|e| BconvError::InputError(e))?;
    let writer = get_writer(output).map_err(|e| BconvError::OutputError(e))?;

    let from = get_from(&cfg).unwrap_or_else(|| get_format_from_extension(input).unwrap());
    let to = get_to(&cfg, from);

    println!();
    match input {
        Some(path) => println!("Читаю из '{}'", path.to_string_lossy()),
        None => println!("Читаю из stdin"),
    };

    if from != to {
        println!("Конвертирую из '{}' в '{}'", from, to);
    };

    match output {
        Some(path) => println!("Пишу в '{}'", path.to_string_lossy()),
        None => {
            println!("Пишу в output");
            println!();
        }
    };

    let c = Converter { reader, writer };
    match (from, to) {
        (Format::Csv, Format::Json) => c.convert::<CsvRecords, JsonRecords>(),
        (Format::Json, Format::Csv) => c.convert::<JsonRecords, CsvRecords>(),
        (Format::Csv, Format::Csv) => c.convert::<CsvRecords, CsvRecords>(),
        (Format::Json, Format::Json) => c.convert::<JsonRecords, JsonRecords>(),
    }?;

    Ok(())
}

struct Converter<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Converter<R, W> {
    fn convert<T1, T2>(self) -> Result<(), BankError>
    where
        T1: Parse<T1> + Into<T2>,
        for<'a> &'a T2: Print,
    {
        let records1 = banker::parse::<T1>(self.reader)?;
        let records2: T2 = banker::convert_to(records1);
        banker::print(self.writer, &records2)?;

        Ok(())
    }
}

// fn get_reader(cli: &Cli) -> Result<Box<dyn Read>, io::Error> {
fn get_reader(input: Option<&Path>) -> Result<Box<dyn Read>, io::Error> {
    if let Some(path) = input {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        let input = stdin().lock();

        if input.is_terminal() {
            println!("");
            eprintln!("error: input is required");
            println!("");
            println!("For more information, try '--help'.");
            exit(0);
        }

        Ok(Box::new(input))
    }
}

// fn get_writer(cli: &Cli) -> Result<Box<dyn Write>, io::Error> {
fn get_writer(output: Option<&Path>) -> Result<Box<dyn Write>, io::Error> {
    if let Some(path) = output {
        let file = OpenOptions::new().create(true).write(true).open(path)?;
        Ok(Box::new(BufWriter::new(file)))
    } else {
        Ok(Box::new(stdout().lock()))
    }
}

fn get_from(cli: &Cli) -> Option<Format> {
    match cli.in_format {
        Some(format) => Some(format),
        None => None,
    }
}

fn get_format_from_extension(path: Option<&Path>) -> Option<Format> {
    let ext = path?.extension()?;
    OsStr::to_str(ext)?.try_into().ok()
}

fn get_to(cli: &Cli, in_format: Format) -> Format {
    cli.out_format.unwrap_or(in_format)
}
