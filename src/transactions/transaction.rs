use std::collections::HashMap;
use crate::core::Value;

#[derive(Clone)]
pub struct Transaction {
    operations: HashMap<String, Value>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            operations: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: Value) {
        self.operations.insert(key, value);
    }

    pub fn operations(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.operations.iter()
    }
}
