#[cfg(test)]
mod tests {
    use nosql_db::{Database, Value, DatabaseError};
    use tempfile::TempDir;
    use log::debug;
    use tokio;

    #[tokio::test]
    async fn test_persistence() -> Result<(), DatabaseError> {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test_db.db");
        let file_path_str = file_path.to_str().unwrap();
        
        debug!("Test database file path: {:?}", file_path);
        
        {
            debug!("Creating first database instance");
            let db = Database::new(file_path_str).await?;
            debug!("Putting key-value pair");
            db.put("key1".to_string(), Value { data: "value1".to_string() }).await?;
            debug!("Database instance will be dropped");
        }

        {
            debug!("Creating second database instance");
            let db = Database::new(file_path_str).await?;
            debug!("Getting value for key1");
            let value = db.get("key1")?;
            assert_eq!(value.data, "value1");
            debug!("Value retrieved successfully");
        }

        Ok(())
    }
}
