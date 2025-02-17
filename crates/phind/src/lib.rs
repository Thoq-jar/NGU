pub mod finder;

use std::error::Error;
use std::path::PathBuf;

pub type FindResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub expression: Vec<String>,
}

impl Config {
    pub fn new(args: impl Iterator<Item = String>) -> FindResult<Config> {
        let args: Vec<String> = args.skip(1).collect();

        if args.is_empty() {
            return Ok(Config {
                paths: vec![PathBuf::from(".")],
                expression: vec![],
            });
        }

        let mut paths = Vec::new();
        let mut expression = Vec::new();
        let mut in_expression = false;

        for arg in args {
            if !in_expression {
                if arg.starts_with('-') {
                    in_expression = true;
                    expression.push(arg);
                } else {
                    paths.push(PathBuf::from(arg));
                }
            } else {
                expression.push(arg);
            }
        }

        if paths.is_empty() {
            paths.push(PathBuf::from("."));
        }

        Ok(Config { paths, expression })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_no_args() {
        let args = vec!["phind".to_string()];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.paths, vec![PathBuf::from(".")]);
        assert!(config.expression.is_empty());
    }

    #[test]
    fn test_config_with_path() {
        let args = vec!["phind".to_string(), "/test/path".to_string()];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.paths, vec![PathBuf::from("/test/path")]);
        assert!(config.expression.is_empty());
    }

    #[test]
    fn test_config_with_expression() {
        let args = vec!["phind".to_string(), "-name".to_string(), "*.rs".to_string()];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.paths, vec![PathBuf::from(".")]);
        assert_eq!(config.expression, vec!["-name".to_string(), "*.rs".to_string()]);
    }
}
