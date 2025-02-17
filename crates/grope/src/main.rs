#![allow(unused_qualifications)]

mod cli;
mod matcher;
mod printer;
mod error;

use std::process;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = cli::parse_args(std::env::args().collect::<Vec<_>>())?;
    let matcher = matcher::Matcher::new(&config);
    let printer = printer::Printer::new(&config);

    for path in &config.paths {
        matcher.search_file(path, &printer)?;
    }

    Ok(())
}
