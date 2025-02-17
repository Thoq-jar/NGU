use crate::FindResult;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Finder {
    paths: Vec<PathBuf>,
    expression: Vec<String>,
}

impl Finder {
    pub fn new(paths: Vec<PathBuf>, expression: Vec<String>) -> Finder {
        Finder { paths, expression }
    }

    pub fn find(&self) -> FindResult<()> {
        for path in &self.paths {
            self.walk_dir(path)?;
        }
        Ok(())
    }

    fn walk_dir(&self, dir: &PathBuf) -> FindResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Permission denied: {}: {}", dir.display(), e);
                return Ok(());
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            println!("{}", path.display());

            if path.is_dir() {
                if let Err(e) = self.walk_dir(&path) {
                    eprintln!("Permission denied: {}: {}", path.display(), e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_directory() -> TempDir {
        let temp_dir = tempfile::tempdir().unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("test.txt"), "test content").unwrap();
        fs::write(temp_dir.path().join("subdir").join("test2.txt"), "test content").unwrap();
        temp_dir
    }

    #[test]
    fn test_finder_creation() {
        let paths = vec![PathBuf::from(".")];
        let expression = vec!["-name".to_string(), "*.rs".to_string()];
        let finder = Finder::new(paths.clone(), expression.clone());
        assert_eq!(finder.paths, paths);
        assert_eq!(finder.expression, expression);
    }

    #[test]
    fn test_finder_walk_dir() {
        let temp_dir = setup_test_directory();
        let paths = vec![temp_dir.path().to_path_buf()];
        let finder = Finder::new(paths, vec![]);

        assert!(finder.find().is_ok());
    }
}
