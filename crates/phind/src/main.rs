use phind::{Config, FindResult};
use phind::finder::Finder;
use std::env;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> FindResult<()> {
    let config = Config::new(env::args())?;
    let finder = Finder::new(config.paths, config.expression);
    finder.find()?;
    Ok(())
}
