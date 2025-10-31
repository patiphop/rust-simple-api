use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    /// Create a new user instance
    pub fn new_user(name: String, email: String) -> Self {
        let now = Utc::now();
        User {
            id: None,
            name,
            email,
            created_at: now,
            updated_at: Some(now),
        }
    }
    
    /// Create a user with a specific ID (useful when retrieving from database)
    pub fn with_id(id: ObjectId, name: String, email: String, created_at: DateTime<Utc>) -> Self {
        User {
            id: Some(id),
            name,
            email,
            created_at,
            updated_at: Some(created_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_new_user() {
        let user = User::new_user("John Doe".to_string(), "john@example.com".to_string());
        
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert!(user.id.is_none());
        assert!(user.updated_at.is_some());
        assert!(user.created_at <= Utc::now());
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
        assert!(user.updated_at.is_some());
    }

    #[test]
    fn test_user_serialization() {
        let user = User::new_user("Test User".to_string(), "test@example.com".to_string());
        
        // Test serialization to JSON
        let json_result = serde_json::to_string(&user);
        assert!(json_result.is_ok());
        
        let json_str = json_result.unwrap();
        
        // Verify JSON contains expected fields (but not _id since it's None)
        assert!(json_str.contains("name"));
        assert!(json_str.contains("email"));
        assert!(json_str.contains("created_at"));
        assert!(json_str.contains("updated_at"));
        assert!(!json_str.contains("_id"));
    }

    #[test]
    fn test_user_with_id_serialization() {
        let id = ObjectId::new();
        let created_at = Utc::now();
        let user = User::with_id(id.clone(), "Test User".to_string(), "test@example.com".to_string(), created_at);
        
        // Test serialization to JSON
        let json_result = serde_json::to_string(&user);
        assert!(json_result.is_ok());
        
        let json_str = json_result.unwrap();
        
        // Verify JSON contains expected fields including _id
        assert!(json_str.contains("name"));
        assert!(json_str.contains("email"));
        assert!(json_str.contains("created_at"));
        assert!(json_str.contains("updated_at"));
        assert!(json_str.contains("_id"));
    }

    #[test]
    fn test_user_deserialization() {
        let json_data = r#"
        {
            "_id": "507f1f77bcf86cd799439011",
            "name": "Test User",
            "email": "test@example.com",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        }
        "#;
        
        let user_result: Result<User, _> = serde_json::from_str(json_data);
        assert!(user_result.is_ok());
        
        let user = user_result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert!(user.id.is_some());
    }

    #[test]
    fn test_user_deserialization_without_id() {
        let json_data = r#"
        {
            "name": "Test User",
            "email": "test@example.com",
            "created_at": "2023-01-01T00:00:00Z"
        }
        "#;
        
        let user_result: Result<User, _> = serde_json::from_str(json_data);
        assert!(user_result.is_ok());
        
        let user = user_result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert!(user.id.is_none());
    }

    #[test]
    fn test_user_creation_edge_cases() {
        // Test with empty strings
        let user_empty = User::new_user("".to_string(), "".to_string());
        assert_eq!(user_empty.name, "");
        assert_eq!(user_empty.email, "");
        
        // Test with very long strings
        let long_name = "a".repeat(1000);
        let long_email = "b".repeat(1000);
        let user_long = User::new_user(long_name.clone(), long_email.clone());
        assert_eq!(user_long.name, long_name);
        assert_eq!(user_long.email, long_email);
        
        // Test with special characters
        let user_special = User::new_user(
            "用户测试".to_string(),
            "test+special@example.co.uk".to_string()
        );
        assert_eq!(user_special.name, "用户测试");
        assert_eq!(user_special.email, "test+special@example.co.uk");
    }

    #[test]
    fn test_user_timestamps() {
        let before_creation = Utc::now();
        let user = User::new_user("Time Test".to_string(), "time@example.com".to_string());
        let after_creation = Utc::now();
        
        // Verify timestamps are set correctly
        assert!(user.created_at >= before_creation);
        assert!(user.created_at <= after_creation);
        assert!(user.updated_at.is_some());
        assert_eq!(user.updated_at.unwrap(), user.created_at);
        
        // Test with specific timestamp
        let specific_time = Utc::now();
        let user_specific = User::with_id(
            ObjectId::new(),
            "Specific Time".to_string(),
            "specific@example.com".to_string(),
            specific_time
        );
        
        assert_eq!(user_specific.created_at, specific_time);
        assert_eq!(user_specific.updated_at.unwrap(), specific_time);
    }
}