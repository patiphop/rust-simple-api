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
            // Get the inserted user with the generated ID
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