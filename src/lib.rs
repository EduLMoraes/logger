use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Seek, Write};
use std::path::Path;
use std::{fs::create_dir_all, path::PathBuf};

/// This code snippet defines a function `mkdir` that takes a `&str` as input parameter and returns a `Result<(File, String), String>`.
/// 
/// The function creates a new directory structure based on the given path. It splits the path into individual directory names and iteratively creates each directory using the `create_dir_all` function from the `std::fs` module.
/// 
/// After creating the directories, the function determines the limit index for inserting a file count suffix into the path. It then enters a loop where it checks if a file with the current path already exists. If it does, it increments the file count and continues to the next iteration. If it doesn't, it creates a new file using the `File::create` function and returns it along with the modified path as a tuple wrapped in the `Ok` variant of the `Result` type.
/// 
/// If any error occurs during the directory creation or file operations, the function returns an error with the corresponding error message.
/// 
/// The function also prints the modified path during each iteration of the loop.
/// 
/// Example usage:
/// ```
/// use std::fs::File;
/// use logger::mkdir;
/// 
/// match mkdir("path/to/directory") {
///     Ok((file, path)) => {
///         println!("Directory created: {}", path);
///         // Use the file and path as needed
///     },
///     Err(e) => println!("Error creating directory: {}", e),
/// }
/// ```
///
pub fn mkdir(path: &str) -> Result<(File, String), String> {
    let mut new_path: String = String::new();

    let paths: Vec<&str> = path.split('/').collect();

    for i in 0..paths.len() - 1 {
        new_path.push_str(paths[i]);
        new_path.push('/');
    }

    match create_dir_all(new_path) {
        Ok(_) => {
            let mut limit: usize = 0;
            for a in path.chars() {
                if a == '.' && limit == 0 {
                    limit += 1;
                } else if a == '.' {
                    break;
                } else {
                    limit += 1;
                }
            }

            let mut path = path.to_string();
            let mut is_alterated: bool = false;
            let mut count_files = 0;

            loop {
                if count_files > 0 && count_files < 11 {
                    if is_alterated {
                        path.replace_range(limit..limit + 3, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                    println!("{path}");
                } else if count_files > 0 {
                    if is_alterated {
                        path.replace_range(limit..limit + 4, format!("({count_files})").trim());
                    } else {
                        path.insert_str(limit, format!("({count_files})").trim());
                        is_alterated = true;
                    }
                    println!("{path}");
                }

                match fs::metadata(path.clone()) {
                    Ok(_) => count_files += 1,
                    Err(_) => {
                        let file = match File::create(path.clone()) {
                            Ok(f) => f,
                            Err(e) => return Err(e.to_string()),
                        };

                        return Ok((file, path));
                    }
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}


/// This code snippet defines a function `log` that logs a message to a file specified by the `path` parameter.
/// 
/// # Arguments
/// 
/// * `path` - A `PathBuf` representing the path to the log file.
/// * `message` - A string slice containing the message to be logged.
/// 
/// # Returns
/// 
/// * `Ok(())` - If the log operation is successful.
/// * `Err(String)` - If there is an error during the log operation.
/// 
/// # Examples
/// 
/// ```
/// use std::path::PathBuf;
/// use logger::log;
/// 
/// let path = PathBuf::from("log.txt");
/// let message = "This is a log message";
/// 
/// match log(path, message) {
///     Ok(()) => println!("Log operation successful"),
///     Err(e) => println!("Error during log operation: {}", e),
/// }
/// ```
///
pub fn log(path: PathBuf, message: &str) -> Result<(), String> {
    if path.as_path().to_str().unwrap().is_empty() || ( path.extension() != Some(&OsStr::new("txt")) && path.extension() != Some(&OsStr::new("log")) ) {
        println!("Path invalido");
        return Err("Path is empty".to_string());
    }

    match OpenOptions::new().read(true).write(true).open(&path) {
        Ok(mut file) => {
            let mut log_file = String::new();

            // file.read_to_string(&mut log_file).unwrap();
            file.seek(std::io::SeekFrom::End(0)).unwrap();

            log_file.push_str(&message);

            let result = file
                .write_all(log_file.as_bytes())
                .map_err(|e| e.to_string());

            println!("{result:?}");

            match result {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(_) => {
            let (_, path) = mkdir(path.as_path().to_str().unwrap())
                .map_err(|e| log(path, &e))
                .unwrap();

            log(Path::new(&path).to_path_buf(), message)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    // When the file exists, the message is appended to the end of the file.
    #[test]
    fn test_append_message_to_existing_file() {
        // Arrange
        let path = PathBuf::from("./existing_file.txt");
        let message = "Test message";

        // Create an existing file
        let mut file = File::create(&path).unwrap();
        file.write_all(b"Existing content").unwrap();

        // Act
        let _ = log(path.clone(), message);

        // Assert
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Existing contentTest message");
    }

    // When the file does not exist, it is created and the message is written to it.
    #[test]
    fn test_create_file_and_write_message() {
        // Arrange
        let path = PathBuf::from("./new_file.txt");
        let message = "Test message";

        // Act
        let _ = log(path.clone(), message);

        // Assert
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Test message");
    }

    // When the file is empty, the message is written to it.
    #[test]
    fn test_write_message_to_empty_file() {
        // Arrange
        let path = PathBuf::from("./empty_file.txt");
        let message = "Test message";

        // Create an empty file
        let _ = File::create(&path).unwrap();

        // Act
        let _ = log(path.clone(), message);

        // Assert
        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Test message");
    }

    // When the path is an empty string, an error is returned.
    #[test]
    fn test_empty_path_returns_error() {
        // Arrange
        let path = PathBuf::from("");
        let message = "Test message";

        // Act
        let result = log(path.clone(), message);

        // Assert
        assert!(result.is_err());
    }

    // When the path contains invalid characters, an error is returned.
    #[test]
    fn test_invalid_path_returns_error() {
        // Arrange
        let path = PathBuf::from("invalid/path");
        let message = "Test message";

        // Act
        let result = log(path.clone(), message);

        // Assert
        assert!(result.is_err());
    }

    // When the path is too long, an error is returned.
    #[test]
    fn test_long_path_returns_error() {
        // Arrange
        let path = PathBuf::from("a".repeat(256));
        let message = "Test message";

        // Act
        let result = log(path.clone(), message);

        // Assert
        assert!(result.is_err());
    }
}
