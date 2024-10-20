use nosql_db::{Database, Value};
use log::{error, info, debug};
use std::panic;
use chrono::Local;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let result = panic::catch_unwind(|| -> Result<(), Box<dyn std::error::Error>> {
        debug!("Starting database operations");

        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let db_path = current_dir.join("database.json");
        
        info!("Database file location: {:?}", db_path);

        let db = Database::new(db_path.to_str().unwrap())?;

        info!("Database created successfully");

        // Get current date and time in a readable format
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // Create a key-value pair with the current date and time
        let key = format!("compile_time");
        let value = Value { data: format!("Compiled at {}", formatted_time) };

        debug!("Attempting to put a key-value pair");
        db.put(key, value)?;
        info!("New compile-time entry added to database");

        Ok(())
    });

    match result {
        Ok(Ok(())) => info!("Program completed successfully"),
        Ok(Err(e)) => error!("Program encountered an error: {}", e),
        Err(e) => error!("Program panicked: {:?}", e),
    }

    debug!("Program execution completed");
    Ok(())
}
