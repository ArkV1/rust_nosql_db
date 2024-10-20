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
fn test_transaction() -> Result<(), DatabaseError> {
    let (db, _dir) = create_temp_database();
    
    db.begin_transaction()?;
    db.put_in_transaction("key1".to_string(), Value { data: "value1".to_string() })?;
    db.put_in_transaction("key2".to_string(), Value { data: "value2".to_string() })?;
    db.commit_transaction()?;

    assert_eq!(db.get("key1")?.data, "value1");
    assert_eq!(db.get("key2")?.data, "value2");

    Ok(())
}

#[test]
fn test_concurrent_transactions() -> Result<(), DatabaseError> {
    let (db, _dir) = create_temp_database();
    let db = Arc::new(db);

    let handles: Vec<_> = (0..10).map(|i| {
        let db = db.clone();
        std::thread::spawn(move || -> Result<(), DatabaseError> {
            db.begin_transaction()?;
            db.put_in_transaction(format!("key{}", i), Value { data: format!("value{}", i) })?;
            db.commit_transaction()?;
            Ok(())
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap()?;
    }

    for i in 0..10 {
        assert_eq!(db.get(&format!("key{}", i))?.data, format!("value{}", i));
    }

    Ok(())
}

