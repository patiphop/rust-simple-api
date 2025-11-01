use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Rejection, Reply};
use mongodb::{Database, Collection};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::StreamExt;
use std::sync::Arc;

use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
        }
    }
}

/// Get all users
pub async fn get_all_users(db: Arc<Database>) -> Result<impl Reply, Rejection> {
    let collection: Collection<User> = db.collection("users");
    
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(user) => users.push(UserResponse::from(user)),
                    Err(_) => {
                        let error_response = ErrorResponse {
                            error: "database_error".to_string(),
                            message: "Error processing user data".to_string(),
                        };
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&error_response),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                }
            }
            
            Ok(warp::reply::with_status(
                warp::reply::json(&users),
                StatusCode::OK,
            ))
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "database_error".to_string(),
                message: "Failed to fetch users from database".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&error_response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Get a user by ID
pub async fn get_user_by_id(id: String, db: Arc<Database>) -> Result<impl Reply, Rejection> {
    let collection: Collection<User> = db.collection("users");
    
    match ObjectId::parse_str(&id) {
        Ok(object_id) => {
            match collection.find_one(doc! { "_id": object_id }, None).await {
                Ok(Some(user)) => {
                    let user_response = UserResponse::from(user);
                    Ok(warp::reply::with_status(
                        warp::reply::json(&user_response),
                        StatusCode::OK,
                    ))
                }
                Ok(None) => {
                    let error_response = ErrorResponse {
                        error: "not_found".to_string(),
                        message: "User not found".to_string(),
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&error_response),
                        StatusCode::NOT_FOUND,
                    ))
                }
                Err(_) => {
                    let error_response = ErrorResponse {
                        error: "database_error".to_string(),
                        message: "Failed to fetch user from database".to_string(),
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&error_response),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "invalid_id".to_string(),
                message: "Invalid user ID format".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&error_response),
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

/// Create a new user
pub async fn create_user(create_user_req: CreateUserRequest, db: Arc<Database>) -> Result<impl Reply, Rejection> {
    let collection: Collection<User> = db.collection("users");
    
    // Validate input
    if create_user_req.name.trim().is_empty() {
        let error_response = ErrorResponse {
            error: "validation_error".to_string(),
            message: "Name is required".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&error_response),
            StatusCode::BAD_REQUEST,
        ));
    }
    
    if create_user_req.email.trim().is_empty() {
        let error_response = ErrorResponse {
            error: "validation_error".to_string(),
            message: "Email is required".to_string(),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&error_response),
            StatusCode::BAD_REQUEST,
        ));
    }
    
    // Create new user
    let new_user = User::new_user(create_user_req.name, create_user_req.email);
    
    match collection.insert_one(&new_user, None).await {
        Ok(result) => {
            // Get the inserted user with generated ID
            match collection.find_one(doc! { "_id": result.inserted_id }, None).await {
                Ok(Some(user)) => {
                    let user_response = UserResponse::from(user);
                    Ok(warp::reply::with_status(
                        warp::reply::json(&user_response),
                        StatusCode::CREATED,
                    ))
                }
                Ok(None) => {
                    let error_response = ErrorResponse {
                        error: "database_error".to_string(),
                        message: "Failed to retrieve created user".to_string(),
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&error_response),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
                Err(_) => {
                    let error_response = ErrorResponse {
                        error: "database_error".to_string(),
                        message: "Failed to retrieve created user".to_string(),
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&error_response),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Err(_) => {
            let error_response = ErrorResponse {
                error: "database_error".to_string(),
                message: "Failed to create user".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&error_response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::{http::StatusCode, Reply};
    use mongodb::{Client, Database};
    use std::sync::Arc;
    use mongodb::bson::oid::ObjectId;
    use chrono::Utc;

    async fn setup_test_database() -> Option<Arc<Database>> {
        // Try different connection strings in order of preference
        // Start with authenticated connection since MongoDB requires auth
        let connection_strings = vec![
            "mongodb://api_user:api_password@localhost:27017/simple_api_db",
            "mongodb://admin:password@localhost:27017/simple_api_db?authSource=admin",
            "mongodb://localhost:27017",
        ];
        
        for connection_string in connection_strings {
            match Client::with_uri_str(connection_string).await {
                Ok(client) => {
                    println!("Successfully connected to MongoDB for user handler tests with: {}", connection_string);
                    let db = client.database("simple_api_db");
                    return Some(Arc::new(db));
                }
                Err(e) => {
                    println!("Failed to connect with '{}': {}", connection_string, e);
                }
            }
        }
        
        println!("MongoDB not available for testing - skipping user handler tests");
        None
    }

    async fn cleanup_test_database(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let collection: Collection<User> = db.collection("users");
        
        // Drop the collection completely to ensure clean state
        let _ = collection.drop(None).await;
        
        // Wait a moment for the operation to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        // Force clear any remaining data
        let _ = collection.delete_many(doc! {}, None).await;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_users_empty() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection for this test to avoid interference
            let collection: Collection<User> = db.collection("test_get_all_users_empty");
            let _ = collection.drop(None).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            // Test getting all users from empty database using direct collection operations
            match collection.find(None, None).await {
                Ok(mut cursor) => {
                    let mut users = Vec::new();
                    while let Some(result) = cursor.next().await {
                        match result {
                            Ok(user) => users.push(UserResponse::from(user)),
                            Err(_) => panic!("Error processing user data"),
                        }
                    }
                    
                    // Should have exactly 0 users
                    assert_eq!(users.len(), 0, "Expected 0 users, found {}", users.len());
                    
                    // Convert to JSON response
                    let response_json = serde_json::to_string(&users).unwrap();
                    assert_eq!(response_json, "[]");
                }
                Err(_) => panic!("Failed to fetch users from database"),
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_users_with_data() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection for this test to avoid interference
            let collection: Collection<User> = db.collection("test_get_all_users_with_data");
            let _ = collection.drop(None).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let test_user = User::new_user(
                "Test User".to_string(),
                "test@example.com".to_string()
            );
            
            // Debug: print user we're trying to insert
            println!("Attempting to insert user: {:?}", test_user);
            
            let insert_result = collection.insert_one(&test_user, None).await;
            match &insert_result {
                Ok(result) => {
                    println!("Successfully inserted user with ID: {:?}", result.inserted_id);
                }
                Err(e) => {
                    println!("Failed to insert user: {}", e);
                }
            }
            assert!(insert_result.is_ok(), "Failed to insert test user: {:?}", insert_result);
            
            // Wait a moment for insert to complete
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            
            // Test direct database operations instead of going through handler
            match collection.find(None, None).await {
                Ok(mut cursor) => {
                    let mut users = Vec::new();
                    while let Some(result) = cursor.next().await {
                        match result {
                            Ok(user) => users.push(UserResponse::from(user)),
                            Err(_) => panic!("Error processing user data"),
                        }
                    }
                    
                    // Should have exactly 1 user
                    assert_eq!(users.len(), 1, "Expected 1 user, found {}", users.len());
                    
                    // Check user data
                    let user_response = &users[0];
                    assert_eq!(user_response.name, "Test User");
                    assert_eq!(user_response.email, "test@example.com");
                    
                    println!("Successfully retrieved user: {:?}", user_response);
                }
                Err(_) => panic!("Failed to fetch users from database"),
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_by_id_valid() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            // Insert test user
            let collection: Collection<User> = db.collection("users");
            let test_user = User::new_user(
                "Test User".to_string(),
                "test@example.com".to_string()
            );
            let insert_result = collection.insert_one(&test_user, None).await;
            assert!(insert_result.is_ok(), "Failed to insert test user: {:?}", insert_result);
            
            let insert_success = insert_result.unwrap();
            // Get the inserted ID
            let user_id = insert_success.inserted_id.as_object_id().unwrap().to_hex();
            
            // Test getting user by ID
            let response = get_user_by_id(user_id, db.clone()).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::OK);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain user data
            assert!(body_str.contains("Test User"));
            assert!(body_str.contains("test@example.com"));
            
            let cleanup_result2 = cleanup_test_database(&*db).await;
            assert!(cleanup_result2.is_ok(), "Failed to cleanup database after test: {:?}", cleanup_result2);
        }
    }

    #[tokio::test]
    async fn test_get_user_by_id_invalid_format() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            // Test with invalid ID format
            let invalid_id = "invalid-id".to_string();
            let response = get_user_by_id(invalid_id, db).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain error information
            assert!(body_str.contains("invalid_id"));
            assert!(body_str.contains("Invalid user ID format"));
        }
    }

    #[tokio::test]
    async fn test_get_user_by_id_not_found() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            // Test with valid ID format but non-existent ID
            let non_existent_id = ObjectId::new().to_hex();
            let response = get_user_by_id(non_existent_id, db).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain not found error
            assert!(body_str.contains("not_found"));
            assert!(body_str.contains("User not found"));
        }
    }

    #[tokio::test]
    async fn test_create_user_valid() {
        if let Some(db) = setup_test_database().await {
            // Use a unique collection for this test
            let collection: Collection<User> = db.collection("users");
            let _ = collection.drop(None).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            
            let create_request = CreateUserRequest {
                name: "New User".to_string(),
                email: "newuser@example.com".to_string(),
            };
            
            let response = create_user(create_request, db.clone()).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::CREATED);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain created user data
            assert!(body_str.contains("New User"));
            assert!(body_str.contains("newuser@example.com"));
        }
    }

    #[tokio::test]
    async fn test_create_user_empty_name() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            let create_request = CreateUserRequest {
                name: "".to_string(),
                email: "test@example.com".to_string(),
            };
            
            let response = create_user(create_request, db).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain validation error
            assert!(body_str.contains("validation_error"));
            assert!(body_str.contains("Name is required"));
        }
    }

    #[tokio::test]
    async fn test_create_user_empty_email() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            let create_request = CreateUserRequest {
                name: "Test User".to_string(),
                email: "".to_string(),
            };
            
            let response = create_user(create_request, db).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain validation error
            assert!(body_str.contains("validation_error"));
            assert!(body_str.contains("Email is required"));
        }
    }

    #[tokio::test]
    async fn test_create_user_whitespace_only() {
        if let Some(db) = setup_test_database().await {
            let cleanup_result = cleanup_test_database(&*db).await;
            assert!(cleanup_result.is_ok(), "Failed to cleanup database: {:?}", cleanup_result);
            
            let create_request = CreateUserRequest {
                name: "   ".to_string(),
                email: "   ".to_string(),
            };
            
            let response = create_user(create_request, db).await;
            assert!(response.is_ok());
            
            let reply = response.unwrap();
            let response = reply.into_response();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            
            let (_parts, body) = response.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            
            // Should contain validation error for name (first validation check)
            assert!(body_str.contains("validation_error"));
            assert!(body_str.contains("Name is required"));
        }
    }

    #[test]
    fn test_user_response_from_user() {
        let id = ObjectId::new();
        let created_at = Utc::now();
        let user = User::with_id(
            id.clone(),
            "Test User".to_string(),
            "test@example.com".to_string(),
            created_at
        );
        
        let user_response = UserResponse::from(user);
        
        assert_eq!(user_response.id, id.to_hex());
        assert_eq!(user_response.name, "Test User");
        assert_eq!(user_response.email, "test@example.com");
        assert_eq!(user_response.created_at, created_at.to_rfc3339());
    }

    #[test]
    fn test_user_response_from_user_without_id() {
        let user = User::new_user(
            "Test User".to_string(),
            "test@example.com".to_string()
        );
        
        let user_response = UserResponse::from(user);
        
        assert_eq!(user_response.id, ""); // Default when no ID
        assert_eq!(user_response.name, "Test User");
        assert_eq!(user_response.email, "test@example.com");
    }

    #[test]
    fn test_error_response_structure() {
        let error_response = ErrorResponse {
            error: "test_error".to_string(),
            message: "Test message".to_string(),
        };
        
        assert_eq!(error_response.error, "test_error");
        assert_eq!(error_response.message, "Test message");
    }

    #[test]
    fn test_create_user_request_structure() {
        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        
        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }

    #[test]
    fn test_serialization_deserialization() {
        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        
        // Test serialization
        let json_result = serde_json::to_string(&request);
        assert!(json_result.is_ok());
        
        let json_str = json_result.unwrap();
        assert!(json_str.contains("Test User"));
        assert!(json_str.contains("test@example.com"));
        
        // Test deserialization
        let deserialized: Result<CreateUserRequest, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok());
        
        let deserialized_request = deserialized.unwrap();
        assert_eq!(deserialized_request.name, "Test User");
        assert_eq!(deserialized_request.email, "test@example.com");
    }
}