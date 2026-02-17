//! HTTP request handlers for thedate service.

use actix_web::{HttpResponse, Responder};
use chrono::Utc;

use crate::timestamp::from_chrono;

/// Main endpoint handler - returns current timestamp in 61+ formats
///
/// # Returns
///
/// JSON response containing timestamp in all supported formats.
///
/// # Example Response
///
/// ```json
/// {
///   "yyyy_mm_dd": "2024_03_15",
///   "unix_timestamp": 1710504045,
///   "rfc3339_date_format": "2024-03-15T12:30:45+00:00",
///   ...
/// }
/// ```
pub async fn home() -> impl Responder {
    HttpResponse::Ok().json(from_chrono(Utc::now()))
}

/// Health check endpoint for monitoring and orchestration
///
/// # Returns
///
/// JSON response with status "healthy" and HTTP 200.
///
/// # Example Response
///
/// ```json
/// {
///   "status": "healthy"
/// }
/// ```
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}
