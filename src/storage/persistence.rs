use std::fs::{File, OpenOptions};
use std::io::{Write, Read, ErrorKind};
use crate::error::DatabaseError;
use log::{debug, error};

pub fn save_to_file(file_path: &str, data: &str) -> Result<(), DatabaseError> {
    debug!("Attempting to save to file: {}", file_path);
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file for writing: {}", e);
            return Err(DatabaseError::IoError(e));
        }
    };

    match file.write_all(data.as_bytes()) {
        Ok(_) => {
            debug!("Data written to file successfully");
            match file.sync_all() {
                Ok(_) => {
                    debug!("File synced successfully");
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to sync file: {}", e);
                    Err(DatabaseError::IoError(e))
                }
            }
        },
        Err(e) => {
            error!("Failed to write data to file: {}", e);
            Err(DatabaseError::IoError(e))
        }
    }
}

pub fn load_from_file(file_path: &str) -> Result<String, DatabaseError> {
    debug!("Attempting to load from file: {}", file_path);
    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    debug!("File contents loaded successfully");
                    Ok(contents)
                },
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    debug!("File is empty");
                    Ok(String::new())
                },
                Err(e) => {
                    error!("Error reading file: {:?}", e);
                    Err(DatabaseError::IoError(e))
                }
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            debug!("File not found, creating a new empty file");
            File::create(file_path)?;
            Ok(String::new())
        },
        Err(e) => {
            error!("Error opening file: {:?}", e);
            Err(DatabaseError::IoError(e))
        }
    }
}
