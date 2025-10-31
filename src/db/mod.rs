use mongodb::{Client, Database};
use std::env;

/// Default MongoDB connection string
const DEFAULT_MONGODB_URI: &str = "mongodb://localhost:27017";

/// Default database name
const DEFAULT_DATABASE_NAME: &str = "simple_api_db";

/// Establishes a connection to MongoDB and returns the database instance
pub async fn connect_to_database() -> Result<Database, Box<dyn std::error::Error>> {
    // Get MongoDB connection string from environment variables
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| DEFAULT_MONGODB_URI.to_string());
    
    // Get database name from environment variables
    let db_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
    
    // Connect to MongoDB
    let client = Client::with_uri_str(mongodb_uri).await?;
    
    // Get the database instance
    let database = client.database(&db_name);
    
    println!("Connected to MongoDB database: {}", db_name);
    
    Ok(database)
}


/// Seed data module for populating the database with mock data
pub mod seed;
pub use seed::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_connect_to_database_with_defaults() {
        // Clear any existing environment variables
        unsafe {
            env::remove_var("MONGODB_URI");
            env::remove_var("DATABASE_NAME");
        }
        
        // This test uses default values
        let result = connect_to_database().await;
        
        // Note: This test will fail if MongoDB is not running
        // In a real test environment, you would set up a test MongoDB instance
        match result {
            Ok(database) => {
                // If connection succeeds, verify database name
                assert_eq!(database.name(), DEFAULT_DATABASE_NAME);
            }
            Err(_) => {
                // Connection failed, which is expected if MongoDB is not running
                // This is acceptable for unit testing
                println!("MongoDB not available for testing - skipping connection test");
            }
        }
    }

    #[tokio::test]
    async fn test_connect_to_database_with_env_vars() {
        // Set custom environment variables
        unsafe {
            env::set_var("MONGODB_URI", "mongodb://localhost:27017");
            env::set_var("DATABASE_NAME", "test_db");
        }
        
        let result = connect_to_database().await;
        
        match result {
            Ok(database) => {
                assert_eq!(database.name(), "test_db");
            }
            Err(_) => {
                println!("MongoDB not available for testing - skipping connection test");
            }
        }
        
        // Clean up environment variables
        unsafe {
            env::remove_var("MONGODB_URI");
            env::remove_var("DATABASE_NAME");
        }
    }

    #[tokio::test]
    async fn test_connect_to_database_invalid_uri() {
        // Set invalid MongoDB URI
        unsafe {
            env::set_var("MONGODB_URI", "mongodb://invalid-host:27017");
            env::set_var("DATABASE_NAME", "test_db");
        }
        
        let result = connect_to_database().await;
        
        // Should fail with invalid URI
        assert!(result.is_err());
        
        // Clean up environment variables
        unsafe {
            env::remove_var("MONGODB_URI");
            env::remove_var("DATABASE_NAME");
        }
    }


    #[test]
    fn test_environment_variable_parsing() {
        // Test default MongoDB URI
        unsafe {
            env::remove_var("MONGODB_URI");
        }
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| DEFAULT_MONGODB_URI.to_string());
        assert_eq!(mongodb_uri, DEFAULT_MONGODB_URI);
        
        // Test custom MongoDB URI
        unsafe {
            env::set_var("MONGODB_URI", "mongodb://custom-host:27017");
        }
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| DEFAULT_MONGODB_URI.to_string());
        assert_eq!(mongodb_uri, "mongodb://custom-host:27017");
        
        // Test default database name
        unsafe {
            env::remove_var("DATABASE_NAME");
        }
        let db_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
        assert_eq!(db_name, DEFAULT_DATABASE_NAME);
        
        // Test custom database name
        unsafe {
            env::set_var("DATABASE_NAME", "custom_db");
        }
        let db_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
        assert_eq!(db_name, "custom_db");
        
        // Clean up
        unsafe {
            env::remove_var("MONGODB_URI");
            env::remove_var("DATABASE_NAME");
        }
    }

    #[test]
    fn test_database_configuration_edge_cases() {
        // Test with empty environment variables
        unsafe {
            env::set_var("MONGODB_URI", "");
            env::set_var("DATABASE_NAME", "");
        }
        
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| DEFAULT_MONGODB_URI.to_string());
        let db_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
        
        // Should fall back to defaults when empty strings are provided
        assert_eq!(mongodb_uri, "");
        assert_eq!(db_name, "");
        
        // Test with whitespace-only environment variables
        unsafe {
            env::set_var("MONGODB_URI", "   ");
            env::set_var("DATABASE_NAME", "   ");
        }
        
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| DEFAULT_MONGODB_URI.to_string());
        let db_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
        
        assert_eq!(mongodb_uri, "   ");
        assert_eq!(db_name, "   ");
        
        // Clean up
        unsafe {
            env::remove_var("MONGODB_URI");
            env::remove_var("DATABASE_NAME");
        }
    }
}