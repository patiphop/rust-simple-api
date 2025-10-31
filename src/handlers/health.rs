use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Rejection, Reply};
use chrono::{DateTime, Utc};

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
        version: "1.0.0".to_string(),
    };
    
    Ok(warp::reply::json(&response))
}

pub async fn health_check_with_status() -> Result<impl Reply, Rejection> {
    let response = health_check().await?;
    
    Ok(warp::reply::with_status(
        response,
        StatusCode::OK,
    ))
}