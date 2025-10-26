mod cli;
mod converter;
mod error;

fn main() {
    let cfg = cli::parse();
    if let Err(err) = converter::run(cfg) {
        eprintln!("{err}");
    }
}
