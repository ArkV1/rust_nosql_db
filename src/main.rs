use nosql_db::{Database, Value};
use log::{error, info};
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let db_path = current_dir.join("database.json");
    
    println!("Database file location: {:?}", db_path);

    let db = match Database::new(db_path.to_str().unwrap()) {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to create database: {}", e);
            return;
        }
    };

    println!("Database created successfully");

    // Direct put operation
    match db.put("direct_key".to_string(), Value { data: "direct_value".to_string() }) {
        Ok(_) => println!("Direct put successful"),
        Err(e) => error!("Direct put failed: {}", e),
    }

    // Example usage with transactions
    let mut transaction = db.begin_transaction();
    println!("Transaction begun");

    db.put_in_transaction(&mut transaction, "key1".to_string(), Value { data: "value1".to_string() });
    db.put_in_transaction(&mut transaction, "key2".to_string(), Value { data: "value2".to_string() });
    db.delete_in_transaction(&mut transaction, "key1".to_string());

    match db.commit_transaction(transaction) {
        Ok(_) => println!("Transaction committed successfully"),
        Err(e) => error!("Failed to commit transaction: {}", e),
    }

    match db.get("key1") {
        Ok(value) => println!("Value for key1: {:?}", value),
        Err(e) => println!("key1 not found (as expected): {}", e),
    }

    match db.get("key2") {
        Ok(value) => println!("Value for key2: {:?}", value),
        Err(e) => error!("Failed to get key2: {}", e),
    }

    println!("Program finished. Check if the database file exists now.");
}
