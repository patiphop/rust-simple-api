use crate::models::User;
use mongodb::{bson::doc, Collection, Database};
use std::error::Error;

/// Result type for seed operations
pub type SeedResult<T> = Result<T, Box<dyn Error>>;

/// Seed mock user data into the database
pub async fn seed_users(db: &Database) -> SeedResult<usize> {
    let collection: Collection<User> = db.collection("users");

    // Check if users already exist
    let existing_count = collection.count_documents(doc! {}, None).await?;

    if existing_count > 0 {
        println!(
            "Database already contains {} users. Skipping seed operation.",
            existing_count
        );
        return Ok(0);
    }

    // Create mock users with realistic data
    let mock_users = vec![
        User::new_user(
            "Alice Johnson".to_string(),
            "alice.johnson@example.com".to_string(),
        ),
        User::new_user("Bob Smith".to_string(), "bob.smith@example.com".to_string()),
        User::new_user(
            "Carol Williams".to_string(),
            "carol.williams@example.com".to_string(),
        ),
        User::new_user(
            "David Brown".to_string(),
            "david.brown@example.com".to_string(),
        ),
        User::new_user("Eva Davis".to_string(), "eva.davis@example.com".to_string()),
        User::new_user(
            "Frank Miller".to_string(),
            "frank.miller@example.com".to_string(),
        ),
        User::new_user(
            "Grace Wilson".to_string(),
            "grace.wilson@example.com".to_string(),
        ),
        User::new_user(
            "Henry Moore".to_string(),
            "henry.moore@example.com".to_string(),
        ),
    ];

    // Insert all users
    let insert_result = collection.insert_many(mock_users, None).await?;
    let inserted_count = insert_result.inserted_ids.len();

    println!(
        "Successfully seeded {} users into the database.",
        inserted_count
    );
    Ok(inserted_count)
}

/// Clear all user data from the database
pub async fn clear_users(db: &Database) -> SeedResult<u64> {
    let collection: Collection<User> = db.collection("users");

    let delete_result = collection.delete_many(doc! {}, None).await?;
    let deleted_count = delete_result.deleted_count;

    println!(
        "Successfully deleted {} users from the database.",
        deleted_count
    );
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

/// Seed mock user data into a specific collection
#[allow(dead_code)]
pub async fn seed_users_to_collection(db: &Database, collection_name: &str) -> SeedResult<usize> {
    let collection: Collection<User> = db.collection(collection_name);

    // Check if users already exist
    let existing_count = collection.count_documents(doc! {}, None).await?;

    if existing_count > 0 {
        println!(
            "Collection '{}' already contains {} users. Skipping seed operation.",
            collection_name, existing_count
        );
        return Ok(0);
    }

    // Create mock users with realistic data
    let mock_users = vec![
        User::new_user(
            "Alice Johnson".to_string(),
            "alice.johnson@example.com".to_string(),
        ),
        User::new_user("Bob Smith".to_string(), "bob.smith@example.com".to_string()),
        User::new_user(
            "Carol Williams".to_string(),
            "carol.williams@example.com".to_string(),
        ),
        User::new_user(
            "David Brown".to_string(),
            "david.brown@example.com".to_string(),
        ),
        User::new_user("Eva Davis".to_string(), "eva.davis@example.com".to_string()),
        User::new_user(
            "Frank Miller".to_string(),
            "frank.miller@example.com".to_string(),
        ),
        User::new_user(
            "Grace Wilson".to_string(),
            "grace.wilson@example.com".to_string(),
        ),
        User::new_user(
            "Henry Moore".to_string(),
            "henry.moore@example.com".to_string(),
        ),
    ];

    // Insert all users
    let insert_result = collection.insert_many(mock_users, None).await?;
    let inserted_count = insert_result.inserted_ids.len();

    println!(
        "Successfully seeded {} users into collection '{}'.",
        inserted_count, collection_name
    );
    Ok(inserted_count)
}

/// Clear all user data from a specific collection
#[allow(dead_code)]
pub async fn clear_users_from_collection(db: &Database, collection_name: &str) -> SeedResult<u64> {
    let collection: Collection<User> = db.collection(collection_name);

    let delete_result = collection.delete_many(doc! {}, None).await?;
    let deleted_count = delete_result.deleted_count;

    println!(
        "Successfully deleted {} users from collection '{}'.",
        deleted_count, collection_name
    );
    Ok(deleted_count)
}

/// Get the count of users in a specific collection
#[allow(dead_code)]
pub async fn get_user_count_from_collection(
    db: &Database,
    collection_name: &str,
) -> SeedResult<u64> {
    let collection: Collection<User> = db.collection(collection_name);

    let count = collection.count_documents(doc! {}, None).await?;
    Ok(count)
}

/// Force reseed a specific collection (clear existing data and insert new mock data)
#[allow(dead_code)]
pub async fn reseed_users_to_collection(db: &Database, collection_name: &str) -> SeedResult<usize> {
    println!(
        "Clearing existing users from collection '{}'...",
        collection_name
    );
    clear_users_from_collection(db, collection_name).await?;

    println!("Seeding new users into collection '{}'...", collection_name);
    seed_users_to_collection(db, collection_name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{Client, Database};

    async fn setup_test_database() -> Option<Database> {
        // Use the same database name as the one user has permissions for
        let test_db_name = "simple_api_db";

        // Try different connection strings in order of preference
        // Start with authenticated connection since MongoDB requires auth
        let connection_strings = vec![
            "mongodb://admin:admin@localhost:27017/simple_api_db?authSource=admin",
            "mongodb://admin:password@localhost:27017/simple_api_db?authSource=admin",
            "mongodb://localhost:27017",
        ];

        for connection_string in connection_strings {
            match Client::with_uri_str(connection_string).await {
                Ok(client) => {
                    println!(
                        "Successfully connected to MongoDB for testing with: {}",
                        connection_string
                    );
                    return Some(client.database(test_db_name));
                }
                Err(e) => {
                    println!("Failed to connect with '{}': {}", connection_string, e);
                }
            }
        }

        println!("MongoDB not available for testing - skipping database tests");
        None
    }

    #[allow(dead_code)]
    async fn ensure_clean_database(db: &Database) -> Result<(), Box<dyn Error>> {
        let collection: Collection<User> = db.collection("users");

        // Try to drop the collection first
        let _ = db.collection::<User>("users").drop(None).await;

        // Force clear any remaining data
        let _ = collection.delete_many(doc! {}, None).await;

        // Wait a moment for operations to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Verify the database is actually empty
        let count = collection.count_documents(doc! {}, None).await?;
        if count != 0 {
            // If still not empty, try one more aggressive clear
            let _ = collection.delete_many(doc! {}, None).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_seed_users_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_seed_users_empty";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Ensure completely clean collection
            let _ = collection.drop(None).await;

            // Test seeding with a custom seed function that uses our test collection
            let result = seed_users_to_collection(&db, test_collection_name).await;
            assert!(result.is_ok(), "Failed to seed users: {:?}", result);

            let seeded_count = result.unwrap();
            assert!(seeded_count > 0);
            assert_eq!(seeded_count, 8); // We have 8 mock users in the seed function

            // Verify count
            let count_result = get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_result.is_ok(),
                "Failed to get user count: {:?}",
                count_result
            );
            let count = count_result.unwrap();
            assert_eq!(count as usize, seeded_count);

            // Clean up
            let _ = clear_users_from_collection(&db, test_collection_name).await;
        }
    }

    #[tokio::test]
    async fn test_seed_users_non_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_seed_users_non_empty";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Ensure completely clean collection
            let _ = collection.drop(None).await;

            let seed_result = seed_users_to_collection(&db, test_collection_name).await;
            assert!(
                seed_result.is_ok(),
                "Failed to seed initial users: {:?}",
                seed_result
            );
            let initial_count = seed_result.unwrap();

            // Try to seed again - should return 0 since data already exists
            let result = seed_users_to_collection(&db, test_collection_name).await;
            assert!(
                result.is_ok(),
                "Failed to seed users second time: {:?}",
                result
            );

            let second_seed_count = result.unwrap();
            assert_eq!(second_seed_count, 0);

            // Verify count is still the same
            let count_result = get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_result.is_ok(),
                "Failed to get user count: {:?}",
                count_result
            );
            let count = count_result.unwrap();
            assert_eq!(count as usize, initial_count);

            // Clean up
            let _ = clear_users_from_collection(&db, test_collection_name).await;
        }
    }

    #[tokio::test]
    async fn test_clear_users() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_clear_users";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Ensure completely clean collection
            let _ = collection.drop(None).await;

            // Seed some data first
            let seed_result = seed_users_to_collection(&db, test_collection_name).await;
            assert!(
                seed_result.is_ok(),
                "Failed to seed users: {:?}",
                seed_result
            );

            // Verify data exists
            let count_result = get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_result.is_ok(),
                "Failed to get user count: {:?}",
                count_result
            );
            let count_before = count_result.unwrap();
            assert!(count_before > 0);

            // Clear the data
            let result = clear_users_from_collection(&db, test_collection_name).await;
            assert!(result.is_ok(), "Failed to clear users: {:?}", result);

            let deleted_count = result.unwrap();
            assert_eq!(deleted_count, count_before);

            // Verify data is cleared
            let count_after_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_after_result.is_ok(),
                "Failed to get user count after clear: {:?}",
                count_after_result
            );
            let count_after = count_after_result.unwrap();
            assert_eq!(count_after, 0);
        }
    }

    #[tokio::test]
    async fn test_clear_empty_database() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_clear_empty";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Ensure collection is completely empty
            let _ = collection.drop(None).await;

            // Try to clear again
            let result = clear_users_from_collection(&db, test_collection_name).await;
            assert!(
                result.is_ok(),
                "Failed to clear empty database: {:?}",
                result
            );

            let deleted_count = result.unwrap();
            assert_eq!(deleted_count, 0);
        }
    }

    #[tokio::test]
    async fn test_get_user_count() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_get_user_count";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Test empty database
            let _ = collection.drop(None).await;

            let empty_count_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                empty_count_result.is_ok(),
                "Failed to get user count from empty database: {:?}",
                empty_count_result
            );
            let empty_count = empty_count_result.unwrap();
            assert_eq!(empty_count, 0);

            // Seed some data
            let seed_result = seed_users_to_collection(&db, test_collection_name).await;
            assert!(
                seed_result.is_ok(),
                "Failed to seed users: {:?}",
                seed_result
            );
            let seeded_count = seed_result.unwrap();

            let count_after_seed_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_after_seed_result.is_ok(),
                "Failed to get user count after seed: {:?}",
                count_after_seed_result
            );
            let count_after_seed = count_after_seed_result.unwrap();
            assert_eq!(count_after_seed as usize, seeded_count);

            // Clear data and verify count is 0
            let clear_result2 = clear_users_from_collection(&db, test_collection_name).await;
            assert!(
                clear_result2.is_ok(),
                "Failed to clear database second time: {:?}",
                clear_result2
            );

            let count_after_clear_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_after_clear_result.is_ok(),
                "Failed to get user count after clear: {:?}",
                count_after_clear_result
            );
            let count_after_clear = count_after_clear_result.unwrap();
            assert_eq!(count_after_clear, 0);
        }
    }

    #[tokio::test]
    async fn test_reseed_users() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection name for this test
            let test_collection_name = "test_reseed_users";
            let collection: Collection<User> = db.collection(test_collection_name);

            // Ensure completely clean collection
            let _ = collection.drop(None).await;

            // Seed initial data using reseed to ensure clean state
            let seed_result = reseed_users_to_collection(&db, test_collection_name).await;
            assert!(
                seed_result.is_ok(),
                "Failed to seed initial users: {:?}",
                seed_result
            );

            let initial_count_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                initial_count_result.is_ok(),
                "Failed to get initial user count: {:?}",
                initial_count_result
            );
            let initial_count = initial_count_result.unwrap();
            assert!(initial_count > 0);

            // Add some additional data manually to test reseed
            let additional_user = User::new_user(
                "Additional User".to_string(),
                "additional@example.com".to_string(),
            );
            let insert_result = collection.insert_one(additional_user, None).await;
            assert!(
                insert_result.is_ok(),
                "Failed to insert additional user: {:?}",
                insert_result
            );

            let count_before_reseed_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                count_before_reseed_result.is_ok(),
                "Failed to get user count before reseed: {:?}",
                count_before_reseed_result
            );
            let count_before_reseed = count_before_reseed_result.unwrap();
            assert!(count_before_reseed > initial_count);

            // Reseed - should clear all data and add fresh seed data
            let result = reseed_users_to_collection(&db, test_collection_name).await;
            assert!(result.is_ok(), "Failed to reseed users: {:?}", result);

            let reseeded_count = result.unwrap();
            assert_eq!(reseeded_count, 8); // Should be exactly 8 users after reseed

            let final_count_result =
                get_user_count_from_collection(&db, test_collection_name).await;
            assert!(
                final_count_result.is_ok(),
                "Failed to get final user count: {:?}",
                final_count_result
            );
            let final_count = final_count_result.unwrap();
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
                "alice.johnson@example.com".to_string(),
            ),
            User::new_user("Bob Smith".to_string(), "bob.smith@example.com".to_string()),
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
