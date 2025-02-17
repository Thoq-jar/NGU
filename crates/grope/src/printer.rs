use std::io;
use std::path::Path;
use crate::cli::Config;

pub struct Printer<'a> {
    config: &'a Config,
}

impl<'a> Printer<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn print_match(&self, path: &Path, line_number: usize, line: &str) -> io::Result<()> {
        if self.config.paths.len() > 1 {
            print!("{}:", path.display());
        }
        if self.config.line_numbers {
            print!("{}:", line_number);
        }
        println!("{}", line);
        Ok(())
    }

    pub fn print_count(&self, path: &Path, count: usize) -> io::Result<()> {
        if self.config.paths.len() > 1 {
            println!("{}:{}", path.display(), count);
        } else {
            println!("{}", count);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_print_match_with_line_numbers() {
        let config = Config {
            pattern: "test".to_string(),
            paths: vec![PathBuf::from("file.txt")],
            case_sensitive: true,
            invert_match: false,
            line_numbers: true,
            count_only: false,
        };

        let printer = Printer::new(&config);
        printer.print_match(Path::new("file.txt"), 1, "test line").unwrap();
    }
}
