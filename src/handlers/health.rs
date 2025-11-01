use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Rejection, Reply};

/// Application version constant
const API_VERSION: &str = "1.0.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

pub async fn health_check() -> Result<impl Reply, Rejection> {
    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now(),
        version: API_VERSION.to_string(),
    };

    Ok(warp::reply::json(&response))
}

pub async fn health_check_with_status() -> Result<impl Reply, Rejection> {
    let response = health_check().await?;

    Ok(warp::reply::with_status(response, StatusCode::OK))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use warp::{http::StatusCode, Reply};

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.is_ok());

        let reply = response.unwrap();
        let response = reply.into_response();

        // Check status code (should be 200 OK by default)
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_check_with_status() {
        let response = health_check_with_status().await;
        assert!(response.is_ok());

        let reply = response.unwrap();
        let response = reply.into_response();

        // Check status code
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_response_structure() {
        let response = health_check().await.unwrap();
        let response = response.into_response();

        // Extract the response body
        let (_parts, body) = response.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        // Parse the JSON response
        let health_response: HealthResponse = serde_json::from_str(&body_str).unwrap();

        // Verify the response structure
        assert_eq!(health_response.status, "ok");
        assert_eq!(health_response.version, API_VERSION);

        // Verify timestamp is recent (within last 5 seconds)
        let now = Utc::now();
        let time_diff = now.signed_duration_since(health_response.timestamp);
        assert!(time_diff.num_seconds() < 5);
    }

    #[tokio::test]
    async fn test_health_response_serialization() {
        let health_response = HealthResponse {
            status: "ok".to_string(),
            timestamp: Utc::now(),
            version: API_VERSION.to_string(),
        };

        // Test serialization
        let json_result = serde_json::to_string(&health_response);
        assert!(json_result.is_ok());

        let json_str = json_result.unwrap();

        // Verify JSON contains expected fields
        assert!(json_str.contains("status"));
        assert!(json_str.contains("timestamp"));
        assert!(json_str.contains("version"));
        assert!(json_str.contains("ok"));
        assert!(json_str.contains(API_VERSION));
    }

    #[tokio::test]
    async fn test_health_response_deserialization() {
        let json_data = r#"
        {
            "status": "ok",
            "timestamp": "2023-01-01T00:00:00Z",
            "version": "1.0.0"
        }
        "#;

        let health_response: Result<HealthResponse, _> = serde_json::from_str(json_data);
        assert!(health_response.is_ok());

        let response = health_response.unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.version, API_VERSION);
    }

    #[tokio::test]
    async fn test_health_check_timestamp_uniqueness() {
        // Make two calls and ensure timestamps are different
        let response1 = health_check().await.unwrap();
        let response1 = response1.into_response();
        let (_parts1, body1) = response1.into_parts();
        let body_bytes1 = hyper::body::to_bytes(body1).await.unwrap();
        let body_str1 = String::from_utf8(body_bytes1.to_vec()).unwrap();
        let health_response1: HealthResponse = serde_json::from_str(&body_str1).unwrap();

        // Small delay to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let response2 = health_check().await.unwrap();
        let response2 = response2.into_response();
        let (_parts2, body2) = response2.into_parts();
        let body_bytes2 = hyper::body::to_bytes(body2).await.unwrap();
        let body_str2 = String::from_utf8(body_bytes2.to_vec()).unwrap();
        let health_response2: HealthResponse = serde_json::from_str(&body_str2).unwrap();

        // Timestamps should be different
        assert_ne!(health_response1.timestamp, health_response2.timestamp);

        // But other fields should be the same
        assert_eq!(health_response1.status, health_response2.status);
        assert_eq!(health_response1.version, health_response2.version);
    }

    #[test]
    fn test_health_response_debug() {
        let health_response = HealthResponse {
            status: "ok".to_string(),
            timestamp: Utc::now(),
            version: API_VERSION.to_string(),
        };

        // Test Debug trait implementation
        let debug_str = format!("{:?}", health_response);
        assert!(debug_str.contains("HealthResponse"));
        assert!(debug_str.contains("ok"));
        assert!(debug_str.contains(API_VERSION));
    }

    #[test]
    fn test_health_response_creation() {
        let now = Utc::now();
        let health_response = HealthResponse {
            status: "ok".to_string(),
            timestamp: now,
            version: API_VERSION.to_string(),
        };

        assert_eq!(health_response.status, "ok");
        assert_eq!(health_response.timestamp, now);
        assert_eq!(health_response.version, API_VERSION);
    }
}
