use nosql_db::{Database, DatabaseError};
use tempfile::TempDir;

pub async fn create_temp_database() -> Result<(Database, TempDir), DatabaseError> {
    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test_db.json");
    
    // Create an empty file
    std::fs::File::create(&file_path).unwrap();
    
    let db = Database::new(file_path.to_str().unwrap()).await?;
    Ok((db, dir))
}
