use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::core::Value;
use tokio::sync::oneshot;

pub struct WriteCache {
    pending_writes: HashMap<String, Option<Value>>,
    write_sender: mpsc::Sender<(String, Option<Value>)>,
}

impl WriteCache {
    pub fn new(write_sender: mpsc::Sender<(String, Option<Value>)>) -> Self {
        WriteCache {
            pending_writes: HashMap::new(),
            write_sender,
        }
    }

    pub fn add(&mut self, key: String, value: Option<Value>) {
        self.pending_writes.insert(key, value);
    }

    pub async fn flush(&mut self) {
        let batch: Vec<_> = self.pending_writes.drain().collect();
        if !batch.is_empty() {
            if let Err(e) = self.write_sender.send((batch, oneshot::channel().0)).await {
                eprintln!("Failed to send write batch: {:?}", e);
            }
        }
    }
}
