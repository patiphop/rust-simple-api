//! Test configuration and utilities for integration tests

use std::env;

/// Test configuration constants
pub mod config {
    use std::time::Duration;
    
    /// Default base URL for the API
    pub const DEFAULT_API_BASE_URL: &str = "http://localhost:3030";
    
    /// Test timeout duration
    pub const TEST_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Health check endpoint
    pub const HEALTH_ENDPOINT: &str = "/health";
    
    /// Users endpoint
    pub const USERS_ENDPOINT: &str = "/users";
    
    /// Maximum retry attempts for server readiness
    pub const MAX_RETRIES: u32 = 10;
    
    /// Delay between retry attempts
    pub const RETRY_DELAY: Duration = Duration::from_secs(1);
}

/// Get the API base URL from environment variable or use default
pub fn get_api_base_url() -> String {
    env::var("TEST_API_BASE_URL")
        .unwrap_or_else(|_| config::DEFAULT_API_BASE_URL.to_string())
}

/// Get test timeout from environment variable or use default
pub fn get_test_timeout() -> std::time::Duration {
    if let Ok(timeout_secs) = env::var("TEST_TIMEOUT_SECONDS") {
        std::time::Duration::from_secs(timeout_secs.parse().unwrap_or(30))
    } else {
        config::TEST_TIMEOUT
    }
}

/// Test user data for creating test users
pub mod test_data {
    use serde_json::json;
    
    /// Create test user data with the given index
    pub fn create_test_user(index: usize) -> serde_json::Value {
        json!({
            "name": format!("Test User {}", index),
            "email": format!("testuser{}@example.com", index)
        })
    }
    
    /// Create invalid test user data (empty name)
    pub fn create_invalid_user_empty_name() -> serde_json::Value {
        json!({
            "name": "",
            "email": "test@example.com"
        })
    }
    
    /// Create invalid test user data (empty email)
    pub fn create_invalid_user_empty_email() -> serde_json::Value {
        json!({
            "name": "Test User",
            "email": ""
        })
    }
    
    /// Create invalid test user data (whitespace only)
    pub fn create_invalid_user_whitespace() -> serde_json::Value {
        json!({
            "name": "   ",
            "email": "   "
        })
    }
}

/// Test utilities for common operations
pub mod utils {
    use super::config::*;
    use reqwest;
    use serde_json::Value;
    use tokio::time::sleep;
    
    /// Wait for the server to be ready
    pub async fn wait_for_server_ready() -> Result<(), Box<dyn std::error::Error>> {
        let base_url = crate::get_api_base_url();
        let health_url = format!("{}{}", base_url, HEALTH_ENDPOINT);
        
        for attempt in 1..=MAX_RETRIES {
            match reqwest::get(&health_url).await {
                Ok(response) if response.status().is_success() => {
                    println!("Server is ready after {} attempts", attempt);
                    return Ok(());
                }
                Ok(_) => {
                    println!("Attempt {}: Server responded but not healthy", attempt);
                }
                Err(e) => {
                    println!("Attempt {}: Cannot connect to server: {}", attempt, e);
                }
            }
            
            if attempt < MAX_RETRIES {
                sleep(RETRY_DELAY).await;
            }
        }
        
        Err("Server did not become ready within the timeout period".into())
    }
    
    /// Clean up test data (if cleanup endpoint is available)
    pub async fn cleanup_test_data() -> Result<(), Box<dyn std::error::Error>> {
        // This would typically call a cleanup endpoint
        // For now, we'll rely on the test database being isolated
        // or manual cleanup between test runs
        println!("Cleanup completed (note: actual cleanup depends on test database isolation)");
        Ok(())
    }
    
    /// Extract user ID from response
    pub fn extract_user_id(response: &Value) -> Option<String> {
        response["id"].as_str().map(|s| s.to_string())
    }
    
    /// Verify user data matches expected values
    pub fn verify_user_data(user: &Value, expected_name: &str, expected_email: &str) -> bool {
        user["name"].as_str() == Some(expected_name) &&
        user["email"].as_str() == Some(expected_email) &&
        user["id"].is_string() &&
        user["created_at"].is_string()
    }
    
    /// Verify error response structure
    pub fn verify_error_response(error: &Value, expected_error: &str, expected_message: &str) -> bool {
        error["error"].as_str() == Some(expected_error) &&
        error["message"].as_str() == Some(expected_message)
    }
}

/// Custom test result type for better error handling
pub type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Test macros for common assertions
#[macro_export]
macro_rules! assert_success {
    ($response:expr) => {
        assert!(
            $response.status().is_success(),
            "Expected successful response, got status: {}",
            $response.status()
        );
    };
}

#[macro_export]
macro_rules! assert_status {
    ($response:expr, $expected_status:expr) => {
        assert_eq!(
            $response.status(),
            $expected_status,
            "Expected status {}, got: {}",
            $expected_status,
            $response.status()
        );
    };
}

#[macro_export]
macro_rules! assert_user_data {
    ($user:expr, $expected_name:expr, $expected_email:expr) => {
        assert_eq!($user["name"], $expected_name);
        assert_eq!($user["email"], $expected_email);
        assert!($user["id"].is_string());
        assert!($user["created_at"].is_string());
    };
}

#[macro_export]
macro_rules! assert_error_response {
    ($error:expr, $expected_error:expr, $expected_message:expr) => {
        assert_eq!($error["error"], $expected_error);
        assert_eq!($error["message"], $expected_message);
    };
}