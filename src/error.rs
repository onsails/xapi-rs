//! Error types for the X API client

use thiserror::Error;

/// Main error type for the X API client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error: {message}")]
    Api {
        /// Error message from the API
        message: String,
        /// Optional error code
        code: Option<String>,
    },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded, reset at {reset_at:?}")]
    RateLimit {
        /// When the rate limit resets
        reset_at: chrono::DateTime<chrono::Utc>,
        /// The endpoint that was rate limited
        endpoint: String,
    },

    /// Invalid response format
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Streaming connection error
    #[error("Streaming error: {0}")]
    Stream(String),

    /// General I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;
