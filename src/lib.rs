use std::collections::HashMap;
use std::sync::Arc;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use parking_lot::{RwLock, Mutex};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use log::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub data: String,
}

pub struct Database {
    data: Arc<RwLock<HashMap<String, Value>>>,
    file_path: String,
}

pub struct Transaction {
    pub(crate) operations: Vec<Operation>,
}

pub(crate) enum Operation {
    Put(String, Value),
    Delete(String),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

impl Database {
    pub fn new(file_path: &str) -> Result<Self, DatabaseError> {
        println!("Creating new database with file path: {}", file_path);
        let data = Arc::new(RwLock::new(HashMap::new()));
        let db = Database {
            data: data.clone(),
            file_path: file_path.to_string(),
        };
        db.load()?;
        println!("Database loaded successfully");
        Ok(db)
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction {
            operations: Vec::new(),
        }
    }

    pub fn commit_transaction(&self, transaction: Transaction) -> Result<(), DatabaseError> {
        let mut data = self.data.write();
        for operation in transaction.operations {
            match operation {
                Operation::Put(key, value) => {
                    data.insert(key, value);
                }
                Operation::Delete(key) => {
                    data.remove(&key);
                }
            }
        }
        self.save()?;
        Ok(())
    }

    pub fn put_in_transaction(&self, transaction: &mut Transaction, key: String, value: Value) {
        transaction.operations.push(Operation::Put(key, value));
    }

    pub fn delete_in_transaction(&self, transaction: &mut Transaction, key: String) {
        transaction.operations.push(Operation::Delete(key));
    }

    pub fn put(&self, key: String, value: Value) -> Result<(), DatabaseError> {
        println!("Putting key-value pair: {} - {:?}", key, value);
        let mut data = self.data.write();
        data.insert(key, value);
        match self.save() {
            Ok(_) => {
                println!("Put operation successful");
                Ok(())
            },
            Err(e) => {
                error!("Failed to save after put: {}", e);
                Err(e)
            }
        }
    }

    pub fn get(&self, key: &str) -> Result<Value, DatabaseError> {
        let data = self.data.read();
        data.get(key).cloned().ok_or_else(|| DatabaseError::KeyNotFound(key.to_string()))
    }

    pub fn delete(&self, key: &str) -> Result<(), DatabaseError> {
        let mut data = self.data.write();
        if data.remove(key).is_some() {
            self.save()?;
            Ok(())
        } else {
            Err(DatabaseError::KeyNotFound(key.to_string()))
        }
    }

    fn save(&self) -> Result<(), DatabaseError> {
        let data = self.data.read();
        let serialized = match serde_json::to_string(&*data) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize data: {}", e);
                return Err(DatabaseError::Serialization(e));
            }
        };
        println!("Attempting to save data: {}", serialized);
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
        {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open file for writing: {}", e);
                return Err(DatabaseError::Io(e));
            }
        };
        match file.write_all(serialized.as_bytes()) {
            Ok(_) => {
                println!("Data saved successfully to: {}", self.file_path);
                Ok(())
            },
            Err(e) => {
                error!("Failed to write data to file: {}", e);
                Err(DatabaseError::Io(e))
            }
        }
    }

    fn load(&self) -> Result<(), DatabaseError> {
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("No existing database file found. Starting with an empty database.");
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let deserialized: HashMap<String, Value> = serde_json::from_str(&contents)?;
        let mut data = self.data.write();
        *data = deserialized;
        info!("Database loaded successfully");
        Ok(())
    }
}
