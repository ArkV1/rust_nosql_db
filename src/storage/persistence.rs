use std::fs::File;
use std::io::{Write, Read};
use crate::error::DatabaseError;
use crate::core::Value;

pub fn save_to_file(file_path: &str, data: &[(String, Value)]) -> Result<(), DatabaseError> {
    let file = File::create(file_path)?;
    let mut writer = std::io::BufWriter::new(file);
    rmp_serde::encode::write(&mut writer, data).map_err(|e| DatabaseError::SerializationError(e.to_string()))?;
    writer.flush()?;
    Ok(())
}

pub fn load_from_file(file_path: &str) -> Result<Vec<(String, Value)>, DatabaseError> {
    let mut file = File::open(file_path)?;
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        return Ok(Vec::new());
    }
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    rmp_serde::from_slice(&buffer).map_err(|e| DatabaseError::SerializationError(e.to_string()))
}
