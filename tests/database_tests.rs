use nosql_db::{Database, Value, DatabaseError};
use std::sync::Arc;
use tempfile::tempdir;

fn create_temp_database() -> (Database, tempfile::TempDir) {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_db.json");
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
    assert!(db.get("key1").is_ok());
    
    db.delete("key1")?;
    assert!(db.get("key1").is_err());
    Ok(())
}

#[test]
fn test_persistence() -> Result<(), DatabaseError> {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_db.json");
    let file_path_str = file_path.to_str().unwrap();
    
    {
        let db = Database::new(file_path_str)?;
        db.put("key1".to_string(), Value { data: "value1".to_string() })?;
        // db is dropped here, which should save the data
    }

    {
        let db = Database::new(file_path_str)?;
        let value = db.get("key1")?;
        assert_eq!(value.data, "value1");
    }
    Ok(())
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

#[test]
fn test_transaction() -> Result<(), DatabaseError> {
    let (db, _dir) = create_temp_database();
    
    let mut transaction = db.begin_transaction();
    
    db.put_in_transaction(&mut transaction, "key1".to_string(), Value { data: "value1".to_string() });
    db.put_in_transaction(&mut transaction, "key2".to_string(), Value { data: "value2".to_string() });
    db.delete_in_transaction(&mut transaction, "key1".to_string());
    
    db.commit_transaction(transaction)?;
    
    assert!(db.get("key1").is_err());
    assert_eq!(db.get("key2")?.data, "value2");
    
    Ok(())
}

#[test]
fn test_concurrent_transactions() -> Result<(), DatabaseError> {
    let (db, _dir) = create_temp_database();
    let db = Arc::new(db);

    let threads: Vec<_> = (0..10)
        .map(|i| {
            let db = db.clone();
            std::thread::spawn(move || -> Result<(), DatabaseError> {
                let mut transaction = db.begin_transaction();
                let key = format!("key{}", i);
                let value = Value { data: format!("value{}", i) };
                db.put_in_transaction(&mut transaction, key.clone(), value);
                db.commit_transaction(transaction)?;
                assert_eq!(db.get(&key)?.data, format!("value{}", i));
                Ok(())
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap()?;
    }

    for i in 0..10 {
        let key = format!("key{}", i);
        assert_eq!(db.get(&key)?.data, format!("value{}", i));
    }

    Ok(())
}
