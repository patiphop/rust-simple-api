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
    use mongodb::Client;
    
    #[tokio::test]
    async fn test_seed_operations() {
        // This test would require a test database instance
        // For now, we'll just verify the functions compile correctly
        // In a real test environment, you would:
        // 1. Set up a test MongoDB instance
        // 2. Create a test database
        // 3. Test the seed functions
        
        // Example test structure (commented out):
        /*
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let db = client.database("test_db");
        
        // Test seeding
        let result = seed_users(&db).await.unwrap();
        assert!(result > 0);
        
        // Test count
        let count = get_user_count(&db).await.unwrap();
        assert_eq!(count as usize, result);
        
        // Test clear
        let deleted = clear_users(&db).await.unwrap();
        assert_eq!(deleted, count);
        */
    }
}