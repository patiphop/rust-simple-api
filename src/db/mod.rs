use mongodb::{Client, Database};
use std::env;

/// Establishes a connection to MongoDB and returns the database instance
pub async fn connect_to_database() -> Result<Database, Box<dyn std::error::Error>> {
    // Get MongoDB connection string from environment variables
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    // Get database name from environment variables
    let db_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "simple_api_db".to_string());
    
    // Connect to MongoDB
    let client = Client::with_uri_str(mongodb_uri).await?;
    
    // Get the database instance
    let database = client.database(&db_name);
    
    println!("Connected to MongoDB database: {}", db_name);
    
    Ok(database)
}

/// Get the database instance (should be called after connect_to_database)
pub fn get_database() -> Result<Database, Box<dyn std::error::Error>> {
    // This function would typically return a stored database instance
    // For now, we'll re-establish the connection
    // In a real application, you might use Arc<Mutex<Database>> or similar
    Err("Database not initialized. Call connect_to_database first.".into())
}

/// Seed data module for populating the database with mock data
pub mod seed;
pub use seed::*;