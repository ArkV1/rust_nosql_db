use nosql_db::{Database, Value};
use log::{error, info, debug};
use chrono::Local;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("Starting database operations");

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let db_path = current_dir.join("database.db");
    let db_path_str = db_path.to_str().unwrap();

    let db = match Database::new(db_path_str).await {
        Ok(db) => {
            info!("Database created or loaded successfully");
            db
        }
        Err(e) => {
            error!("Failed to create or load database: {:?}", e);
            return Err(e.into());
        }
    };

    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let key = "compile_time".to_string();
    let value = Value { data: format!("Compiled at {}", formatted_time) };

    debug!("Attempting to put a key-value pair");
    db.put(key, value).await?;
    info!("New compile-time entry added to database");

    // Perform a snapshot every 5 minutes
    let db_clone = db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            if let Err(e) = db_clone.snapshot().await {
                error!("Failed to create snapshot: {:?}", e);
            }
        }
    });

    Ok(())
}
