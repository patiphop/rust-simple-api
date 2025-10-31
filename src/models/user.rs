use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user instance
    pub fn new_user(name: String, email: String) -> Self {
        User {
            id: None,
            name,
            email,
            created_at: Utc::now(),
        }
    }
    
    /// Create a user with a specific ID (useful when retrieving from database)
    pub fn with_id(id: ObjectId, name: String, email: String, created_at: DateTime<Utc>) -> Self {
        User {
            id: Some(id),
            name,
            email,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new_user("John Doe".to_string(), "john@example.com".to_string());
        
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert!(user.id.is_none());
    }

    #[test]
    fn test_user_with_id() {
        let id = ObjectId::new();
        let created_at = Utc::now();
        let user = User::with_id(id.clone(), "Jane Doe".to_string(), "jane@example.com".to_string(), created_at);
        
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "jane@example.com");
        assert_eq!(user.id, Some(id));
        assert_eq!(user.created_at, created_at);
    }
}