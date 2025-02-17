use crate::error::GropeError;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub paths: Vec<PathBuf>,
    pub case_sensitive: bool,
    pub invert_match: bool,
    pub line_numbers: bool,
    pub count_only: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            pattern: String::new(),
            paths: Vec::new(),
            case_sensitive: true,
            invert_match: false,
            line_numbers: false,
            count_only: false,
        }
    }
}

pub fn parse_args<I>(args: I) -> Result<Config, GropeError>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut config = Config::new();
    let args: Vec<_> = args.into_iter().skip(1).collect();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_ref() {
            "-i" | "--ignore-case" => config.case_sensitive = false,
            "-v" | "--invert-match" => config.invert_match = true,
            "-n" | "--line-number" => config.line_numbers = true,
            "-c" | "--count" => config.count_only = true,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            arg if arg.starts_with('-') => {
                return Err(GropeError::InvalidArgument(arg.to_string()))
            }
            _ => {
                if config.pattern.is_empty() {
                    config.pattern = args[i].as_ref().to_string();
                } else {
                    config.paths.push(PathBuf::from(args[i].as_ref()));
                }
            }
        }
        i += 1;
    }

    if config.pattern.is_empty() {
        return Err(GropeError::NoPattern);
    }

    if config.paths.is_empty() {
        config.paths.push(PathBuf::from("-"));  // stdin
    }

    Ok(config)
}

fn print_help() {
    println!("Usage: grope [OPTIONS] PATTERN [FILE...]");
    println!("\nOptions:");
    println!("  -i, --ignore-case    Ignore case distinctions");
    println!("  -v, --invert-match   Select non-matching lines");
    println!("  -n, --line-number    Print line number with output lines");
    println!("  -c, --count          Print only a count of matching lines");
    println!("  -h, --help           Print this help message");
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_basic_pattern_and_path() {
        let args = vec!["grope", "pattern", "file.txt"];
        let config = parse_args(args).unwrap();
        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.paths, vec![PathBuf::from("file.txt")]);
    }

    #[test]
    fn test_ignore_case_option() {
        let args = vec!["grope", "-i", "pattern"];
        let config = parse_args(args).unwrap();
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_no_pattern_error() {
        let args = vec!["grope"];
        assert!(matches!(parse_args(args), Err(GropeError::NoPattern)));
    }
}
