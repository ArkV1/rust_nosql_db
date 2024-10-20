#[cfg(test)]
mod tests {
    use nosql_db::{Database, Value, DatabaseError};
    use tempfile::TempDir;
    use log::debug;

    #[test]
    fn test_persistence() -> Result<(), DatabaseError> {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test_db.json");
        let file_path_str = file_path.to_str().unwrap();
        
        debug!("Test database file path: {:?}", file_path);
        
        {
            debug!("Creating first database instance");
            let db = Database::new(file_path_str)?;
            debug!("Putting key-value pair");
            db.put("key1".to_string(), Value { data: "value1".to_string() })?;
            debug!("Database instance will be dropped");
        }

        debug!("Checking if file exists");
        assert!(file_path.exists(), "Database file does not exist");
        
        debug!("Reading file contents");
        let file_contents = std::fs::read_to_string(&file_path)?;
        debug!("File contents: {}", file_contents);
        
        assert!(file_contents.contains("key1"), "File does not contain 'key1'");
        assert!(file_contents.contains("value1"), "File does not contain 'value1'");

        {
            debug!("Creating second database instance");
            let db = Database::new(file_path_str)?;
            debug!("Getting value for key1");
            let value = db.get("key1")?;
            assert_eq!(value.data, "value1", "Retrieved value does not match expected value");
        }

        Ok(())
    }
}
