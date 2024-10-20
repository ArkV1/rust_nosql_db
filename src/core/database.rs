use crate::error::DatabaseError;
use crate::storage::persistence::{save_to_file, load_from_file};
use crate::core::Value;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde_json;
use log::{debug, info, error};

pub struct Database {
    file_path: String,
    data: Arc<RwLock<HashMap<String, Value>>>,
    transaction: Arc<RwLock<Option<HashMap<String, Option<Value>>>>>,
}

impl Database {
    pub fn new(file_path: &str) -> Result<Self, DatabaseError> {
        debug!("Creating new Database instance with file path: {}", file_path);
        let data = Arc::new(RwLock::new(HashMap::new()));
        let db = Database {
            file_path: file_path.to_string(),
            data,
            transaction: Arc::new(RwLock::new(None)),
        };
        
        debug!("Attempting to load data from file");
        match db.load() {
            Ok(_) => {
                debug!("Data loaded successfully");
                Ok(db)
            },
            Err(e) => {
                error!("Failed to load data: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn put(&self, key: String, value: Value) -> Result<(), DatabaseError> {
        debug!("Attempting to put key: {}", key);
        let mut data = self.data.write();
        data.insert(key.clone(), value);
        debug!("Key-value pair inserted into memory");
        drop(data); // Release the write lock before saving
        self.save()
    }

    pub fn get(&self, key: &str) -> Result<Value, DatabaseError> {
        let data = self.data.read();
        data.get(key)
            .cloned()
            .ok_or_else(|| DatabaseError::KeyNotFound(key.to_string()))
    }

    pub fn delete(&self, key: &str) -> Result<(), DatabaseError> {
        let mut data = self.data.write();
        data.remove(key);
        self.save()
    }

    pub fn save(&self) -> Result<(), DatabaseError> {
        debug!("Attempting to save data to file");
        let data = self.data.read();
        let json = match serde_json::to_string(&*data) {
            Ok(j) => {
                debug!("Data serialized successfully");
                j
            },
            Err(e) => {
                error!("Failed to serialize data: {}", e);
                return Err(DatabaseError::SerializationError(e));
            }
        };

        debug!("Attempting to save serialized data to file");
        match save_to_file(&self.file_path, &json) {
            Ok(_) => {
                info!("Data saved to file successfully");
                Ok(())
            },
            Err(e) => {
                error!("Failed to save data to file: {}", e);
                Err(e)
            }
        }
    }

    pub fn load(&self) -> Result<(), DatabaseError> {
        debug!("Attempting to load data from file");
        let contents = load_from_file(&self.file_path)?;
        if contents.is_empty() {
            debug!("File is empty, initializing with empty data");
            return Ok(());
        }
        let loaded_data: HashMap<String, Value> = serde_json::from_str(&contents)?;
        let mut data = self.data.write();
        *data = loaded_data;
        debug!("Data loaded successfully");
        Ok(())
    }

    pub fn begin_transaction(&self) -> Result<(), DatabaseError> {
        let mut transaction = self.transaction.write();
        if transaction.is_some() {
            return Err(DatabaseError::TransactionAlreadyInProgress);
        }
        *transaction = Some(HashMap::new());
        Ok(())
    }

    pub fn put_in_transaction(&self, key: String, value: Value) -> Result<(), DatabaseError> {
        let mut transaction = self.transaction.write();
        if let Some(trans) = transaction.as_mut() {
            trans.insert(key, Some(value));
            Ok(())
        } else {
            Err(DatabaseError::NoTransactionInProgress)
        }
    }

    pub fn commit_transaction(&self) -> Result<(), DatabaseError> {
        let mut transaction = self.transaction.write();
        if let Some(trans) = transaction.take() {
            let mut data = self.data.write();
            for (key, value_option) in trans {
                match value_option {
                    Some(value) => data.insert(key, value),
                    None => data.remove(&key),
                };
            }
            self.save()?;
            Ok(())
        } else {
            Err(DatabaseError::NoTransactionInProgress)
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            error!("Failed to save database on drop: {:?}", e);
        }
    }
}
