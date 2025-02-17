use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use crate::cli::Config;
use crate::printer::Printer;

pub struct Matcher<'a> {
    config: &'a Config,
}

impl<'a> Matcher<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn search_file(&self, path: &Path, printer: &Printer) -> io::Result<()> {
        let reader: Box<dyn BufRead> = if path.to_str() == Some("-") {
            Box::new(BufReader::new(io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(path)?))
        };

        let mut matches = 0;
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let matches_pattern = if self.config.case_sensitive {
                line.contains(&self.config.pattern)
            } else {
                line.to_lowercase().contains(&self.config.pattern.to_lowercase())
            };

            if matches_pattern != self.config.invert_match {
                matches += 1;
                if !self.config.count_only {
                    printer.print_match(path, i + 1, &line)?;
                }
            }
        }

        if self.config.count_only {
            printer.print_count(path, matches)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_case_sensitive_search() {
        let config = Config {
            pattern: "Hello".to_string(),
            paths: vec![],
            case_sensitive: true,
            invert_match: false,
            line_numbers: false,
            count_only: true,
        };

        let file = create_test_file("Hello World\nhello world");
        let matcher = Matcher::new(&config);
        let printer = Printer::new(&config);

        matcher.search_file(file.path(), &printer).unwrap();
    }

    #[test]
    fn test_case_insensitive_search() {
        let config = Config {
            pattern: "Hello".to_string(),
            paths: vec![],
            case_sensitive: false,
            invert_match: false,
            line_numbers: false,
            count_only: true,
        };

        let file = create_test_file("Hello World\nhello world");
        let matcher = Matcher::new(&config);
        let printer = Printer::new(&config);

        matcher.search_file(file.path(), &printer).unwrap();
    }
}
