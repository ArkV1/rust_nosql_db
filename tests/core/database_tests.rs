#[cfg(test)]
mod tests {
    use nosql_db::{Database, Value, DatabaseError};
    use tempfile::TempDir;
    use std::sync::Arc;

    fn create_temp_database() -> (Database, TempDir) {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test_db.json");
        
        // Create an empty file
        std::fs::File::create(&file_path).unwrap();
        
        let db = Database::new(file_path.to_str().unwrap()).expect("Failed to create database");
        (db, dir)
    }

    #[test]
    fn test_put_and_get() -> Result<(), DatabaseError> {
        let (db, _dir) = create_temp_database();
        
        db.put("key1".to_string(), Value { data: "value1".to_string() })?;
        
        let value = db.get("key1")?;
        assert_eq!(value.data, "value1");
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), DatabaseError> {
        let (db, _dir) = create_temp_database();
        
        db.put("key1".to_string(), Value { data: "value1".to_string() })?;
        db.delete("key1")?;
        
        assert!(db.get("key1").is_err());
        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), DatabaseError> {
        let (db, _dir) = create_temp_database();
        
        db.put("key1".to_string(), Value { data: "value1".to_string() })?;
        db.put("key1".to_string(), Value { data: "updated_value1".to_string() })?;
        
        let value = db.get("key1")?;
        assert_eq!(value.data, "updated_value1");
        Ok(())
    }

    #[test]
    fn test_get_non_existent_key() -> Result<(), DatabaseError> {
        let (db, _dir) = create_temp_database();
        
        match db.get("non_existent_key") {
            Err(DatabaseError::KeyNotFound(_)) => Ok(()),
            _ => panic!("Expected KeyNotFound error"),
        }
    }

    #[test]
    fn test_concurrent_access() -> Result<(), DatabaseError> {
        let (db, _dir) = create_temp_database();
        let db = Arc::new(db);

        let threads: Vec<_> = (0..10)
            .map(|i| {
                let db = db.clone();
                std::thread::spawn(move || -> Result<(), DatabaseError> {
                    let key = format!("key{}", i);
                    let value = Value { data: format!("value{}", i) };
                    db.put(key.clone(), value)?;
                    assert_eq!(db.get(&key)?.data, format!("value{}", i));
                    Ok(())
                })
            })
            .collect();

        for thread in threads {
            thread.join().unwrap()?;
        }

        assert!(db.get("key0").is_ok());
        assert!(db.get("key9").is_ok());
        Ok(())
    }
}
