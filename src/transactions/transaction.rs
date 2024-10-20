pub enum Operation {
    Put,
    Delete,
}

pub struct Transaction {
    operations: Vec<Operation>,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            operations: Vec::new(),
        }
    }

    pub fn put(&mut self) {
        self.operations.push(Operation::Put);
    }

    pub fn delete(&mut self) {
        self.operations.push(Operation::Delete);
    }

    // Add any other methods you need for the Transaction struct
}
