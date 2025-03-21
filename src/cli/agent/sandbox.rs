use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use ignore::{Error, WalkBuilder};
use crate::common::create_folder::{create_directory_with_files, TextFile};
use crate::config::config::read_json_config;

pub(crate) fn create_sandbox() -> io::Result<()> {

    let path = Path::new(".ai");
    fs::create_dir_all(path)?;

    let files = vec![
        TextFile {
            filename: "config.json".to_string(),
            content: "{ \"project\": { \"root\": \"../\" } }".to_string(),
        },
        TextFile {
            filename: "../.aignore".to_string(),
            content: ".ai/\n".to_string(),
        },
    ];

    create_directory_with_files(&path, Some(files)).unwrap();

    Ok(())
}

pub(crate) fn sync_sandbox(from_path_str: &str, to_path_str: &str, respect_gitignore: bool) -> io::Result<usize> {

    let config = read_json_config("./.ai/config.json").unwrap();
    let sandboxroot = config.project.unwrap().sandboxroot;
    let from_path = Path::new(from_path_str);
    let to_path = Path::new(to_path_str).join(sandboxroot);

    let aignore_path = Path::new(".aignore");
    copy_respecting_gitignore(from_path, &to_path, respect_gitignore, Some(aignore_path))
}


/// Copies all files from source_dir to target_dir, excluding files that match patterns in .gitignore
///
/// # Arguments
///
/// * `source_dir` - The source directory to copy from
/// * `target_dir` - The target directory to copy to
/// * `respect_gitignore` - Whether to respect .gitignore rules (defaults to true)
/// * `custom_gitignore_path` - Optional path to a specific .aignore file to use
///
/// # Returns
///
/// * `io::Result<usize>` - Number of files copied or an error
pub fn copy_respecting_gitignore(
    source_dir: &Path,
    target_dir: &Path,
    respect_gitignore: bool,
    custom_gitignore_path: Option<&Path>,
) -> io::Result<usize> {
    // Ensure the target directory exists
    fs::create_dir_all(target_dir)?;

    // Count of copied files
    let mut copied_count = 0;

    // Use the ignore crate to walk the directory
    let mut builder = WalkBuilder::new(source_dir);

    // Configure the builder
    builder.hidden(false);      // Don't skip hidden files by default
    builder.ignore(respect_gitignore);
    builder.git_ignore(respect_gitignore);

    if let Some(gitignore_path) = custom_gitignore_path {

        println!(".aignore path: {:?}", gitignore_path);
        println!(".aignore exists: {}", gitignore_path.exists());
        println!(".aignore is file: {}", gitignore_path.is_file());

        // Then add the custom gitignore file
        if(gitignore_path.exists() && gitignore_path.is_file()) {

            let add_result = builder.add_ignore(gitignore_path);
            match add_result {
                None => {
                    println!("Using custom gitignore file: {}", gitignore_path.display());
                }
                Some(err) => {
                    eprintln!("Warning: no custom .aignore file found: {}", err);
                }
            }
        }
    }

    let walker = builder.build();

    for result in walker {
        match result {
            Ok(entry) => {
                let entry_path = entry.path();

                // Skip directories (we'll create them as needed when copying files)
                if entry_path.is_dir() {
                    continue;
                }

                // Calculate the relative path from the source directory
                let relative_path = match entry_path.strip_prefix(source_dir) {
                    Ok(path) => path,
                    Err(e) => {
                        eprintln!("Error calculating relative path: {}", e);
                        continue;
                    }
                };

                // Create the target path
                let target_path = target_dir.join(relative_path);

                // Ensure the parent directory exists
                if let Some(parent) = target_path.parent() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        eprintln!("Error creating directory {}: {}", parent.display(), e);
                        continue;
                    }
                }

                // Copy the file
                match fs::copy(entry_path, &target_path) {
                    Ok(_) => {
                        copied_count += 1;
                    },
                    Err(e) => {
                        eprintln!("Error copying {} to {}: {}",
                                  entry_path.display(), target_path.display(), e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error walking directory: {}", e);
            }
        }
    }

    Ok(copied_count)
}

// Example of how to use the function
fn main() -> io::Result<()> {
    let source = PathBuf::from("source_directory");
    let target = PathBuf::from("target_directory");

    // Example 1: Using default .gitignore files in the directory structure
    let files_copied = copy_respecting_gitignore(&source, &target, true, None)?;
    println!("Successfully copied {} files", files_copied);

    // Example 2: Using a specific custom .gitignore file
    let custom_gitignore = PathBuf::from(".aignore");
    let files_copied = copy_respecting_gitignore(&source, &target, true, Some(&custom_gitignore))?;
    println!("Successfully copied {} files using custom gitignore", files_copied);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::common::create_folder::{create_directory_with_files, TextFile};
    use super::*;
    #[test]
    fn test_copy_respecting_gitignore() -> io::Result<()> {

        create_sandbox().unwrap();

        let source = PathBuf::from("target/source_directory");
        let target = PathBuf::from("target/target_directory");


        let files = vec![
            TextFile {
                filename: ".aignore".to_string(),
                content: "target/source_directory/.aignore\ntarget/source_directory/secret.txt\n".to_string(),
            },
            TextFile {
                filename: "secret.txt".to_string(),
                content: "debug=true\nverbose=false".to_string(),
            },
            TextFile {
                filename: "config.txt".to_string(),
                content: "debug=true\nverbose=false".to_string(),
            },
        ];

        create_directory_with_files(&source, Some(files)).unwrap();

        sync_sandbox("target/source_directory", "target/target_directory", true).unwrap();

        Ok(())
    }
}