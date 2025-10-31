use mongodb::{Collection, Database, bson::doc};
use crate::models::User;
use std::error::Error;

/// Result type for seed operations
pub type SeedResult<T> = Result<T, Box<dyn Error>>;

/// Seed mock user data into the database
pub async fn seed_users(db: &Database) -> SeedResult<usize> {
    let collection: Collection<User> = db.collection("users");
    
    // Check if users already exist
    let existing_count = collection.count_documents(doc! {}, None).await?;
    
    if existing_count > 0 {
        println!("Database already contains {} users. Skipping seed operation.", existing_count);
        return Ok(0);
    }
    
    // Create mock users with realistic data
    let mock_users = vec![
        User::new_user(
            "Alice Johnson".to_string(),
            "alice.johnson@example.com".to_string()
        ),
        User::new_user(
            "Bob Smith".to_string(),
            "bob.smith@example.com".to_string()
        ),
        User::new_user(
            "Carol Williams".to_string(),
            "carol.williams@example.com".to_string()
        ),
        User::new_user(
            "David Brown".to_string(),
            "david.brown@example.com".to_string()
        ),
        User::new_user(
            "Eva Davis".to_string(),
            "eva.davis@example.com".to_string()
        ),
        User::new_user(
            "Frank Miller".to_string(),
            "frank.miller@example.com".to_string()
        ),
        User::new_user(
            "Grace Wilson".to_string(),
            "grace.wilson@example.com".to_string()
        ),
        User::new_user(
            "Henry Moore".to_string(),
            "henry.moore@example.com".to_string()
        ),
    ];
    
    // Insert all users
    let insert_result = collection.insert_many(mock_users, None).await?;
    let inserted_count = insert_result.inserted_ids.len();
    
    println!("Successfully seeded {} users into the database.", inserted_count);
    Ok(inserted_count)
}

/// Clear all user data from the database
pub async fn clear_users(db: &Database) -> SeedResult<u64> {
    let collection: Collection<User> = db.collection("users");
    
    let delete_result = collection.delete_many(doc! {}, None).await?;
    let deleted_count = delete_result.deleted_count;
    
    println!("Successfully deleted {} users from the database.", deleted_count);
    Ok(deleted_count)
}

/// Get the count of users in the database
pub async fn get_user_count(db: &Database) -> SeedResult<u64> {
    let collection: Collection<User> = db.collection("users");
    
    let count = collection.count_documents(doc! {}, None).await?;
    Ok(count)
}

/// Force reseed the database (clear existing data and insert new mock data)
pub async fn reseed_users(db: &Database) -> SeedResult<usize> {
    println!("Clearing existing users...");
    clear_users(db).await?;
    
    println!("Seeding new users...");
    seed_users(db).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, Database};
    
    async fn setup_test_database() -> Option<Database> {
        // Use a test database name to avoid conflicts
        let test_db_name = "test_seed_db";
        
        // Try to connect to MongoDB
        match Client::with_uri_str("mongodb://localhost:27017").await {
            Ok(client) => Some(client.database(test_db_name)),
            Err(_) => {
                println!("MongoDB not available for testing - skipping database tests");
                None
            }
        }
    }

    #[tokio::test]
    async fn test_seed_users_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Clear the database first
            let _ = clear_users(&db).await;
            
            // Test seeding
            let result = seed_users(&db).await;
            assert!(result.is_ok());
            
            let seeded_count = result.unwrap();
            assert!(seeded_count > 0);
            assert_eq!(seeded_count, 8); // We have 8 mock users in the seed function
            
            // Verify count
            let count = get_user_count(&db).await.unwrap();
            assert_eq!(count as usize, seeded_count);
            
            // Clean up
            let _ = clear_users(&db).await;
        }
    }

    #[tokio::test]
    async fn test_seed_users_non_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Clear and seed initial data
            let _ = clear_users(&db).await;
            let initial_count = seed_users(&db).await.unwrap();
            
            // Try to seed again - should return 0 since data already exists
            let result = seed_users(&db).await;
            assert!(result.is_ok());
            
            let second_seed_count = result.unwrap();
            assert_eq!(second_seed_count, 0);
            
            // Verify count is still the same
            let count = get_user_count(&db).await.unwrap();
            assert_eq!(count as usize, initial_count);
            
            // Clean up
            let _ = clear_users(&db).await;
        }
    }

    #[tokio::test]
    async fn test_clear_users() {
        if let Some(db) = setup_test_database().await {
            // Seed some data first
            let _ = seed_users(&db).await;
            
            // Verify data exists
            let count_before = get_user_count(&db).await.unwrap();
            assert!(count_before > 0);
            
            // Clear the data
            let result = clear_users(&db).await;
            assert!(result.is_ok());
            
            let deleted_count = result.unwrap();
            assert_eq!(deleted_count, count_before);
            
            // Verify data is cleared
            let count_after = get_user_count(&db).await.unwrap();
            assert_eq!(count_after, 0);
        }
    }

    #[tokio::test]
    async fn test_clear_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Ensure database is empty
            let _ = clear_users(&db).await;
            
            // Try to clear again
            let result = clear_users(&db).await;
            assert!(result.is_ok());
            
            let deleted_count = result.unwrap();
            assert_eq!(deleted_count, 0);
        }
    }

    #[tokio::test]
    async fn test_get_user_count() {
        if let Some(db) = setup_test_database().await {
            // Test empty database
            let _ = clear_users(&db).await;
            let empty_count = get_user_count(&db).await.unwrap();
            assert_eq!(empty_count, 0);
            
            // Seed some data
            let seeded_count = seed_users(&db).await.unwrap();
            let count_after_seed = get_user_count(&db).await.unwrap();
            assert_eq!(count_after_seed as usize, seeded_count);
            
            // Clear data and verify count is 0
            let _ = clear_users(&db).await;
            let count_after_clear = get_user_count(&db).await.unwrap();
            assert_eq!(count_after_clear, 0);
        }
    }

    #[tokio::test]
    async fn test_reseed_users() {
        if let Some(db) = setup_test_database().await {
            // Seed initial data
            let _ = seed_users(&db).await;
            let initial_count = get_user_count(&db).await.unwrap();
            assert!(initial_count > 0);
            
            // Add some additional data manually to test reseed
            let collection: Collection<User> = db.collection("users");
            let additional_user = User::new_user(
                "Additional User".to_string(),
                "additional@example.com".to_string()
            );
            let _ = collection.insert_one(additional_user, None).await;
            
            let count_before_reseed = get_user_count(&db).await.unwrap();
            assert!(count_before_reseed > initial_count);
            
            // Reseed - should clear all data and add fresh seed data
            let result = reseed_users(&db).await;
            assert!(result.is_ok());
            
            let reseeded_count = result.unwrap();
            assert_eq!(reseeded_count, 8); // Should be exactly 8 users after reseed
            
            let final_count = get_user_count(&db).await.unwrap();
            assert_eq!(final_count as usize, reseeded_count);
            assert_eq!(final_count as usize, 8); // Back to original seed count
        }
    }


    #[tokio::test]
    async fn test_mock_user_data_structure() {
        // Verify that our mock user data has the expected structure
        let test_users = vec![
            User::new_user(
                "Alice Johnson".to_string(),
                "alice.johnson@example.com".to_string()
            ),
            User::new_user(
                "Bob Smith".to_string(),
                "bob.smith@example.com".to_string()
            ),
        ];
        
        assert_eq!(test_users.len(), 2);
        assert_eq!(test_users[0].name, "Alice Johnson");
        assert_eq!(test_users[0].email, "alice.johnson@example.com");
        assert_eq!(test_users[1].name, "Bob Smith");
        assert_eq!(test_users[1].email, "bob.smith@example.com");
        
        // Verify all users have valid timestamps
        for user in &test_users {
            assert!(user.id.is_none());
            assert!(user.updated_at.is_some());
            assert_eq!(user.updated_at.unwrap(), user.created_at);
        }
    }

}