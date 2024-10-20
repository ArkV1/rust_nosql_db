use crate::error::DatabaseError;
use crate::storage::persistence::{save_to_file, load_from_file};
use crate::core::Value;
use log::{debug, error};
use dashmap::DashMap;
use crate::transactions::Transaction;
use std::sync::Arc;
use std::fs::File;

pub struct Database {
    data: Arc<DashMap<String, Value>>,
    file_path: String,
    transaction: Option<Transaction>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            file_path: self.file_path.clone(),
            transaction: self.transaction.clone(),
        }
    }
}

impl Database {
    pub async fn new(file_path: &str) -> Result<Self, DatabaseError> {
        let data = Arc::new(DashMap::new());
        
        let db = Self {
            data: Arc::clone(&data),
            file_path: file_path.to_string(),
            transaction: None,
        };

        // Try to load existing data, but don't fail if the file doesn't exist
        match db.load() {
            Ok(_) => debug!("Existing data loaded successfully"),
            Err(DatabaseError::IoError(e)) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("No existing database file found. Starting with an empty database.");
                // Create an empty file
                File::create(file_path)?;
            }
            Err(e) => return Err(e),
        }

        Ok(db)
    }

    pub async fn put(&self, key: String, value: Value) -> Result<(), DatabaseError> {
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Value, DatabaseError> {
        self.data.get(key)
            .map(|ref_multi| ref_multi.clone())
            .ok_or_else(|| DatabaseError::KeyNotFound(key.to_string()))
    }

    pub fn delete(&self, key: &str) -> Result<(), DatabaseError> {
        self.data.remove(key);
        Ok(())
    }

    pub fn save(&self) -> Result<(), DatabaseError> {
        let data_vec: Vec<(String, Value)> = self.data.iter()
            .map(|ref_multi| (ref_multi.key().clone(), ref_multi.value().clone()))
            .collect();
        
        save_to_file(&self.file_path, &data_vec)
    }

    pub fn load(&self) -> Result<(), DatabaseError> {
        debug!("Attempting to load data from file");
        let loaded_data = load_from_file(&self.file_path)?;
        self.data.clear();
        for (key, value) in loaded_data {
            self.data.insert(key, value);
        }
        debug!("Data loaded successfully");
        Ok(())
    }

    pub fn begin_transaction(&mut self) -> Result<(), DatabaseError> {
        if self.transaction.is_some() {
            return Err(DatabaseError::TransactionAlreadyInProgress);
        }
        self.transaction = Some(Transaction::new());
        Ok(())
    }

    pub fn put_in_transaction(&mut self, key: String, value: Value) -> Result<(), DatabaseError> {
        if let Some(trans) = self.transaction.as_mut() {
            trans.put(key, value);
            Ok(())
        } else {
            Err(DatabaseError::NoTransactionInProgress)
        }
    }

    pub fn commit_transaction(&mut self) -> Result<(), DatabaseError> {
        if let Some(trans) = self.transaction.take() {
            for (key, value) in trans.operations() {
                self.data.insert(key.clone(), value.clone());
            }
            self.save()?;
            Ok(())
        } else {
            Err(DatabaseError::NoTransactionInProgress)
        }
    }

    pub async fn snapshot(&self) -> Result<(), DatabaseError> {
        let snapshot: Vec<_> = self.data.iter().map(|entry| (entry.key().clone(), entry.value().clone())).collect();
        save_to_file(&format!("{}_snapshot", self.file_path), &snapshot)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            error!("Failed to save database on drop: {:?}", e);
        }
    }
}
