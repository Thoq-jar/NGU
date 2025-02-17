use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use chrono::{DateTime, Local};
use colored::*;
use users::{get_user_by_uid, get_group_by_gid};
use crate::formatter::{format_permissions, format_size};
use crate::options::Options;

pub(crate) fn list_directory(path: &Path, opts: &Options, level: usize) -> std::io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|r| r.ok())
        .collect();

    entries.retain(|entry| {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if !opts.show_hidden && !opts.show_almost_all {
            !name.starts_with('.')
        } else if opts.show_almost_all {
            name != "." && name != ".."
        } else {
            true
        }
    });


    entries.sort_by(|a, b| {
        if opts.sort_time {
            b.metadata().unwrap().modified().unwrap()
                .cmp(&a.metadata().unwrap().modified().unwrap())
        } else {
            a.file_name().cmp(&b.file_name())
        }
    });

    if opts.reverse_sort {
        entries.reverse();
    }

    if opts.recursive && level > 0 {
        println!("\n{}:", path.display());
    }

    for entry in entries {
        let metadata = entry.metadata()?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        let name_owned = name.to_string();

        if opts.long_format {
            let file_type = if metadata.is_dir() { "d" } else { "-" };
            let perms = format_permissions(metadata.permissions().mode());
            let size = format_size(metadata.len(), opts.human_readable);
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            let uid = metadata.uid();
            let gid = metadata.gid();
            let owner = get_user_by_uid(uid).map_or_else(|| uid.to_string(), |u| u.name().to_string_lossy().into_owned());
            let group = get_group_by_gid(gid).map_or_else(|| gid.to_string(), |g| g.name().to_string_lossy().into_owned());

            print!(
                "{}{} {:>8} {:>8} {} {} ",
                file_type,
                perms,
                owner,
                group,
                size,
                modified.format("%b %d %H:%M")
            );
        }

        let name_colored = if metadata.is_dir() {
            name_owned.blue().bold()
        } else if metadata.permissions().mode() & 0o111 != 0 {
            name_owned.green()
        } else {
            name_owned.normal()
        };

        print!("{}", name_colored);

        if opts.recursive && metadata.is_dir() {
            let _ = list_directory(&entry.path(), opts, level + 1);
        }
        print!(" ");
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_directory() -> TempDir {
        let temp_dir = TempDir::new().unwrap();

        let file1_path = temp_dir.path().join("test1.txt");
        let mut file1 = File::create(file1_path).unwrap();
        writeln!(file1, "test content").unwrap();

        let hidden_path = temp_dir.path().join(".hidden");
        File::create(hidden_path).unwrap();

        fs::create_dir(temp_dir.path().join("subdir")).unwrap();

        temp_dir
    }

    #[test]
    fn test_list_directory_basic() {
        let temp_dir = setup_test_directory();
        let opts = Options::default();

        let result = list_directory(temp_dir.path(), &opts, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_directory_with_hidden() {
        let temp_dir = setup_test_directory();
        let mut opts = Options::default();
        opts.show_hidden = true;

        let result = list_directory(temp_dir.path(), &opts, 0);
        assert!(result.is_ok());
    }
}
