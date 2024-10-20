use nosql_db::{Database, Value, DatabaseError};
use tempfile::TempDir;
use std::sync::{Arc, Mutex};
use tokio;

async fn create_temp_database() -> (Database, TempDir) {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test_db.json");
    
    // Create an empty file
    std::fs::File::create(&file_path).unwrap();
    
    let db = Database::new(file_path.to_str().unwrap()).await.expect("Failed to create database");
    (db, dir)
}

#[tokio::test]
async fn test_basic_transaction() -> Result<(), DatabaseError> {
    let (mut db, _dir) = create_temp_database().await;
    
    db.begin_transaction()?;
    db.put_in_transaction("key1".to_string(), Value { data: "value1".to_string() })?;
    db.put_in_transaction("key2".to_string(), Value { data: "value2".to_string() })?;
    db.commit_transaction()?;
    
    assert_eq!(db.get("key1")?.data, "value1");
    assert_eq!(db.get("key2")?.data, "value2");

    Ok(())
}

#[tokio::test]
async fn test_concurrent_transactions() -> Result<(), DatabaseError> {
    let (db, _dir) = create_temp_database().await;
    let db = Arc::new(Mutex::new(db));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let db = Arc::clone(&db);
            tokio::spawn(async move {
                let mut db = db.lock().unwrap();
                db.begin_transaction()?;
                db.put_in_transaction(format!("key{}", i), Value { data: format!("value{}", i) })?;
                db.commit_transaction()?;
                Ok::<_, DatabaseError>(())
            })
        })
        .collect();

    for handle in handles {
        handle.await.map_err(|_| DatabaseError::OperationFailed)??;
    }

    for i in 0..10 {
        assert_eq!(db.lock().unwrap().get(&format!("key{}", i))?.data, format!("value{}", i));
    }

    Ok(())
}
