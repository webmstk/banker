use crate::config::Config;
use crate::config::Format;
use crate::error::BconvError;

use banker::error::BankError;
use banker::{Parse, Print};
use banker::{csv, json};

use anyhow::Result;
use thiserror_context::Context;

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write, stdin, stdout};

pub fn convert(cfg: &Config) -> Result<()> {
    let reader = get_reader(cfg).map_err(BconvError::InputError)?;
    let writer = get_writer(cfg).map_err(BconvError::OutputError)?;

    let from = &cfg.in_format;
    let to = &cfg.out_format;

    println!();
    match &cfg.input_path {
        Some(path) => println!("Читаю из '{}'", path.to_string_lossy()),
        None => println!("Читаю из stdin"),
    };

    if from != to {
        println!("Конвертирую из '{}' в '{}'", from, to);
    };

    match &cfg.output_path {
        Some(path) => println!("Пишу в '{}'", path.to_string_lossy()),
        None => {
            println!("Пишу в output");
            println!();
        }
    };

    let converter = Converter { reader, writer };
    converter.convert(from, to)?;

    Ok(())
}

struct Converter<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Converter<R, W> {
    pub fn convert(self, from: &Format, to: &Format) -> Result<(), BankError> {
        match (from, to) {
            (Format::Csv, Format::Json) => self
                .execute::<csv::Records, json::Records>()
                .context("csv to json"),
            (Format::Json, Format::Csv) => self
                .execute::<json::Records, csv::Records>()
                .context("json to csv"),
            (Format::Csv, Format::Csv) => self
                .execute::<csv::Records, csv::Records>()
                .context("csv to csv"),
            (Format::Json, Format::Json) => self
                .execute::<json::Records, json::Records>()
                .context("json to json"),
        }
    }

    fn execute<T1, T2>(self) -> Result<(), BankError>
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

fn get_reader(cfg: &Config) -> Result<Box<dyn Read>, io::Error> {
    match &cfg.input_path {
        Some(path) => {
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        }
        None => Ok(Box::new(stdin().lock())),
    }
}

fn get_writer(cfg: &Config) -> Result<Box<dyn Write>, io::Error> {
    match &cfg.output_path {
        Some(path) => {
            let file = OpenOptions::new().create(true).write(true).open(path)?;
            Ok(Box::new(BufWriter::new(file)))
        }
        None => Ok(Box::new(stdout().lock())),
    }
}
