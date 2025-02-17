use crate::options::Options;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ArgError {
    UnknownOption(char),
    InvalidArgument(String),
}

pub struct ArgParser;

impl ArgParser {
    pub fn parse<I>(args: I) -> Result<(Options, Vec<String>), ArgError>
    where
        I: IntoIterator<Item = String>
    {

        let mut opts = Options::default();
        let mut paths = Vec::new();
        let mut parsing_options = true;

        for arg in args.into_iter().skip(1) {
            if arg.is_empty() {
                return Err(ArgError::InvalidArgument("Empty path is not allowed".to_string()));
            }

            if arg == "--help" {
                Self::print_help();
                std::process::exit(0);
            }

            if arg == "--" {
                parsing_options = false;
                continue;
            }

            if parsing_options && arg.starts_with('-') && arg.len() > 1 {
                for c in arg.chars().skip(1) {
                    match c {
                        'a' => opts.show_hidden = true,
                        'A' => opts.show_almost_all = true,
                        'l' => opts.long_format = true,
                        'h' => opts.human_readable = true,
                        't' => opts.sort_time = true,
                        'r' => opts.reverse_sort = true,
                        'R' => opts.recursive = true,
                        '?' => {
                            Self::print_help();
                            std::process::exit(0);
                        }
                        _ => return Err(ArgError::UnknownOption(c)),
                    }
                }
            } else {
                paths.push(arg);
            }
        }
        Ok((opts, paths))
    }

    pub fn print_help() {
        println!("Usage: lsd [OPTION]...");
        println!("List directory contents");
        println!("\nOptions:");
        println!("  -a\t\tshow hidden files");
        println!("  -A\t\tlike -a, but do not list . and ..");
        println!("  -l\t\tuse long listing format");
        println!("  -h\t\thuman-readable sizes");
        println!("  -t\t\tsort by modification time");
        println!("  -r\t\treverse sort order");
        println!("  -R\t\tlist subdirectories recursively");
        println!("  -?\t\tdisplay this help");
        println!("  --help\tdisplay this help");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_option() {
        let args = vec!["lsd".to_string(), "-z".to_string()];
        match ArgParser::parse(args) {
            Err(ArgError::UnknownOption('z')) => (),
            _ => panic!("Expected UnknownOption error"),
        }
    }

    #[test]
    fn test_paths_after_options() {
        let args = vec!["lsd".to_string(), "-l".to_string(), "test_dir".to_string()];
        let (opts, paths) = ArgParser::parse(args).unwrap();
        assert!(opts.long_format);
        assert_eq!(paths, vec!["test_dir"]);
    }

    #[test]
    fn test_paths_with_dash_separator() {
        let args = vec!["lsd".to_string(), "-l".to_string(), "--".to_string(), "-file".to_string()];
        let (opts, paths) = ArgParser::parse(args).unwrap();
        assert!(opts.long_format);
        assert_eq!(paths, vec!["-file"]);
    }

    #[test]
    fn test_empty_args() {
        let args = vec!["lsd".to_string()];
        let (opts, paths) = ArgParser::parse(args).unwrap();
        assert_eq!(opts, Options::default());
        assert!(paths.is_empty());
    }

    #[test]
    fn test_single_flag() {
        let args = vec!["lsd".to_string(), "-a".to_string()];
        let (opts, _) = ArgParser::parse(args).unwrap();
        assert!(opts.show_hidden);
        assert!(!opts.long_format);
    }

    #[test]
    fn test_path_argument() {
        let args = vec!["lsd".to_string(), "some_path".to_string()];
        let (opts, paths) = ArgParser::parse(args).unwrap();
        assert_eq!(paths, vec!["some_path"]);
        assert_eq!(opts, Options::default());
    }

    #[test]
    fn test_unknown_flag() {
        let args = vec!["lsd".to_string(), "-z".to_string()];
        match ArgParser::parse(args) {
            Err(ArgError::UnknownOption('z')) => (),
            _ => panic!("Expected unknown option error"),
        }
    }

    #[test]
    fn test_invalid_argument() {
        let args = vec!["lsd".to_string(), "".to_string()];
        match ArgParser::parse(args) {
            Err(ArgError::InvalidArgument(_)) => (),
            _ => panic!("Expected invalid argument error"),
        }
    }
}
