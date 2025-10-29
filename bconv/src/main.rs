mod cli;
mod config;
mod converter;
mod error;

fn main() {
    let cfg = match cli::parse().try_into() {
        Ok(cfg) => cfg,
        Err(err) => {
            handle_config_errors(err);
            return;
        }
    };

    init_logger(&cfg);

    if let Err(err) = converter::convert(&cfg) {
        match cfg.log_level {
            log::LevelFilter::Trace => eprintln!("Ошибка: {:#?}", err),
            _ => eprintln!("Ошибка: {err}"),
        }
    }
}

fn handle_config_errors(err: config::ConfigError) {
    use config::ConfigError::*;

    println!();
    eprintln!("Ошибка конфигурации: {err}");

    match err {
        InFormatUndefined => {
            println!();
            println!("Подсказка: укажите формат явно через опцию --in-format.");
        }
        InputRequired => {
            println!();
            println!("Подсказка: укажите путь к файлу через опцию --input.");
            println!(
                "Так же можно передать файл на вход, например \
                        `bconv --in-format=csv <path/to/file.csv`."
            );
        }
    }
}

fn init_logger(cfg: &config::Config) {
    let mut clog = colog::default_builder();
    clog.filter(None, cfg.log_level);
    clog.init();
}
