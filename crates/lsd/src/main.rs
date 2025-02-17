mod options;
mod formatter;
mod display;
mod args;

use std::path::Path;
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

fn run() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let (opts, paths) = match args::ArgParser::parse(args) {
        Ok((opts, paths)) => (opts, paths),
        Err(e) => {
            eprintln!("Error parsing arguments: {:?}", e);
            args::ArgParser::print_help();
            process::exit(1);
        }
    };

    let paths = if paths.is_empty() {
        vec![Path::new(".").to_path_buf()]
    } else {
        paths.iter()
            .map(|p| Path::new(p).to_path_buf())
            .collect()
    };

    let multiple_dirs = paths.len() > 1;

    for (i, path) in paths.iter().enumerate() {
        if multiple_dirs {
            if i > 0 {
                println!();
            }
            println!("{}:", path.display());
        }

        if let Err(e) = display::list_directory(path, &opts, 0) {
            eprintln!("{}: {}", path.display(), e);
        }
    }

    Ok(())
}
