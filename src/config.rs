  //! Configuration module for thedate service.
//!
//! Handles reading configuration from environment variables with sensible defaults.

use std::env;

/// Application configuration
///
/// Reads from environment variables:
/// - `HOST`: Server bind address (default: "0.0.0.0")
/// - `PORT`: Server bind port (default: "8080")
pub struct Config {
    /// Host address to bind to
    pub host: String,
    /// Port number to bind to
    pub port: u16,
}

impl Config {
    /// Create configuration from environment variables
    ///
    /// # Examples
    ///
    /// ```
    /// use thedate::config::Config;
    ///
    /// let config = Config::from_env();
    /// assert_eq!(config.host, "0.0.0.0");
    /// assert_eq!(config.port, 8080);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if PORT is set but not a valid number.
    pub fn from_env() -> Self {
        Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
        }
    }

    /// Get the bind address as a string in "host:port" format
    ///
    /// # Examples
    ///
    /// ```
    /// use thedate::config::Config;
    ///
    /// let config = Config::from_env();
    /// let addr = config.bind_address();
    /// assert_eq!(addr, "0.0.0.0:8080");
    /// ```
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
