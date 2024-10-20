# RustDB

RustDB is a high-performance, memory-safe NoSQL database written in Rust, designed for easy integration with multiple programming languages.

## Features

- Basic NoSQL functionality: Key-value store with CRUD operations
- Concurrent read/write operations
- Efficient serialization
- Disk-based storage with in-memory database option
- High performance and low latency

## Getting Started

### Prerequisites

- Rust 1.xx or higher

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rustdb.git
   ```
2. Build the project:
   ```
   cd rustdb
   cargo build --release
   ```

## Usage

Here's a basic example of how to use RustDB:

```rust
use rustdb::Database;

fn main() {
    let db = Database::new("my_database");
    
    // Put a value
    db.put("key1", "value1");
    
    // Get a value
    let value = db.get("key1");
    println!("Value: {:?}", value);
    
    // Delete a value
    db.delete("key1");
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).
