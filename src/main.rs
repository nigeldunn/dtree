use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "dtree")]
#[command(about = "A directory tree viewer")]
struct Args {
    /// Include files in the tree view
    #[arg(short = 'f', long = "files")]
    show_files: bool,

    /// Directory to display (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist", args.path);
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: Path '{}' is not a directory", args.path);
        std::process::exit(1);
    }

    println!("{}", path.display());
    display_tree(path, "", true, args.show_files);
}

fn display_tree(dir: &Path, prefix: &str, _is_last: bool, show_files: bool) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory {}: {}", dir.display(), e);
            return;
        }
    };

    let mut entries: Vec<_> = entries.collect();
    entries.sort_by(|a, b| {
        let a = a.as_ref().unwrap();
        let b = b.as_ref().unwrap();

        // Sort directories first, then files
        let a_is_dir = a.file_type().unwrap().is_dir();
        let b_is_dir = b.file_type().unwrap().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let mut filtered_entries = Vec::new();
    for entry in entries.into_iter().flatten() {
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() || (show_files && file_type.is_file()) {
            filtered_entries.push(entry);
        }
    }

    let total_entries = filtered_entries.len();

    for (index, entry) in filtered_entries.iter().enumerate() {
        let is_last_entry = index == total_entries - 1;
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();

        // Create the tree branch characters
        let branch = if is_last_entry {
            "└── "
        } else {
            "├── "
        };

        println!("{}{}{}", prefix, branch, file_name.to_string_lossy());

        // If it's a directory, recursively display its contents
        if file_type.is_dir() {
            let new_prefix = if is_last_entry {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            display_tree(&entry.path(), &new_prefix, is_last_entry, show_files);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_args_parsing() {
        // Test that Args can be created with default values
        let args = Args {
            show_files: false,
            path: ".".to_string(),
        };
        assert!(!args.show_files);
        assert_eq!(args.path, ".");
    }

    #[test]
    fn test_args_with_files_flag() {
        let args = Args {
            show_files: true,
            path: "/some/path".to_string(),
        };
        assert!(args.show_files);
        assert_eq!(args.path, "/some/path");
    }

    #[test]
    fn test_display_tree_with_empty_directory() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("dtree_test_empty");
        fs::create_dir_all(&temp_dir).unwrap();

        // This should not panic when called on an empty directory
        display_tree(&temp_dir, "", true, false);

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_display_tree_with_files() {
        // Create a temporary directory structure for testing
        let temp_dir = std::env::temp_dir().join("dtree_test_files");
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a test file
        let test_file = temp_dir.join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Create a subdirectory
        let sub_dir = temp_dir.join("subdir");
        fs::create_dir_all(&sub_dir).unwrap();

        // This should not panic when called with show_files = true
        display_tree(&temp_dir, "", true, true);

        // This should not panic when called with show_files = false
        display_tree(&temp_dir, "", true, false);

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_path_validation() {
        // Test that a valid path (current directory) exists
        let current_dir = Path::new(".");
        assert!(current_dir.exists());
        assert!(current_dir.is_dir());
    }

    #[test]
    fn test_nonexistent_path() {
        // Test that a non-existent path returns false
        let fake_path = Path::new("/this/path/should/not/exist/hopefully");
        assert!(!fake_path.exists());
    }
}
