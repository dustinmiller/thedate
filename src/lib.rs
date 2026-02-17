//! # thedate
//!
//! A fast, simple HTTP service that returns the current date and time in 61+ different formats.
//!
//! ## Features
//!
//! - Returns comprehensive timestamp information via JSON API
//! - Supports 61+ date/time formats (ISO 8601, RFC 2822, RFC 3339, Unix timestamp, etc.)
//! - Health check endpoint for monitoring and orchestration
//! - Configurable via environment variables
//! - Lightweight and fast (built with Actix-web and chrono)
//!
//! ## Usage
//!
//! ```bash
//! # Get current timestamp in all formats
//! curl http://localhost:8080/
//!
//! # Check service health
//! curl http://localhost:8080/health
//! ```
//!
//! ## Configuration
//!
//! Set via environment variables:
//! - `HOST`: Bind address (default: 0.0.0.0)
//! - `PORT`: Bind port (default: 8080)
//! - `RUST_LOG`: Log level (default: info)

pub mod config;
pub mod handlers;
pub mod timestamp;

pub use handlers::{health_check, home};
