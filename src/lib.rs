use std::ffi::OsStr;
use std::path::Path;
use std::{path::PathBuf, fs::create_dir_all};
use std::fs::{File, self, OpenOptions};
use std::io::{Write, Read, Seek};

fn mkdir(path: &str) -> Result<(File, String), String> {
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

pub fn log(path: PathBuf, message: &str) -> Result<(), String>{

    if path.as_path().to_str().unwrap().is_empty() || path.extension() != Some(&OsStr::new("txt")) { 
        println!("invalido");
        return Err("Path is empty".to_string());
    }
    
    match OpenOptions::new()
    .read(true)
    .write(true)
    .open(&path) {
        Ok(mut file) => {
            let mut log_file = String::new();

            file.read_to_string(&mut log_file).unwrap();
            file.seek(std::io::SeekFrom::Start(0)).unwrap();

            log_file.push_str(
                &message
            );

            let result = file.write_all(log_file.as_bytes()).map_err(|e| e.to_string() );

            println!("{result:?}");

            match result {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            }
        },
        Err(_) => {
            let (_, path) = mkdir(path.as_path().to_str().unwrap()).map_err(|e|log(path, &e)).unwrap();

            log(Path::new(&path).to_path_buf(), message)
        },
    }
}

#[cfg(test)]
mod tests{
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