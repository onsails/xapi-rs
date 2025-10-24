//! Error types for the X API client
//!
//! This module defines the error hierarchy for the X API client, including:
//! - Main error enum with thiserror integration
//! - API error detail structure for rich error context
//! - Retry classification helpers
//! - Error conversion implementations

use chrono::{DateTime, Utc};
use thiserror::Error;

/// Main error type for the X API client
///
/// This enum covers all possible error conditions that can occur when
/// interacting with the X API v2. Errors are classified for retry logic
/// and include rich context for debugging.
#[derive(Error, Debug)]
pub enum Error {
    /// Network or HTTP request failed
    #[error("HTTP request failed: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// API returned an error response with details
    #[error("API error: {0}")]
    Api(#[from] ApiErrorDetail),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Authorization failed (valid auth but insufficient permissions)
    #[error("Authorization failed: {0}")]
    Authorization(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded for endpoint '{endpoint}', resets at {reset_at}")]
    RateLimitExceeded {
        /// When the rate limit resets
        reset_at: DateTime<Utc>,
        /// The endpoint that was rate limited
        endpoint: String,
        /// Number of requests remaining (usually 0)
        remaining: u32,
        /// Total limit for this endpoint
        limit: u32,
    },

    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Resource not found (404)
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Streaming connection error
    #[error("Streaming connection error: {0}")]
    StreamConnection(String),

    /// Streaming disconnection error
    #[error("Stream disconnected: {0}")]
    StreamDisconnected(String),

    /// Invalid response format from API
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// OAuth error
    #[error("OAuth error: {0}")]
    OAuth(String),

    /// General I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Timeout error
    #[error("Request timeout after {0:?}")]
    Timeout(std::time::Duration),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Detailed error information from the X API
///
/// When the API returns an error response, it includes structured error details
/// with error codes, messages, and context about what went wrong.
#[derive(Error, Debug, Clone)]
#[error("{message} (code: {code})")]
pub struct ApiErrorDetail {
    /// CAPS_CASE error code from the API
    pub code: String,

    /// Human-readable error message
    pub message: String,

    /// The problematic parameter (if applicable)
    pub parameter: Option<String>,

    /// The problematic value (if applicable)
    pub value: Option<String>,

    /// Problem type URI (if provided)
    pub type_uri: Option<String>,

    /// HTTP status code
    pub status: Option<u16>,
}

impl Error {
    /// Check if this error is retryable
    ///
    /// Returns `true` for transient errors that may succeed on retry:
    /// - Network errors
    /// - Rate limit errors (after waiting)
    /// - Server errors (5xx)
    /// - Temporary streaming disconnections
    ///
    /// Returns `false` for permanent errors:
    /// - Authentication/authorization failures
    /// - Invalid requests (4xx except 429)
    /// - Not found errors
    pub fn is_retryable(&self) -> bool {
        match self {
            // Network errors are retryable
            Error::Network(_) => true,

            // Rate limits are retryable after waiting
            Error::RateLimitExceeded { .. } => true,

            // Stream disconnections are retryable
            Error::StreamDisconnected(_) => true,

            // Timeouts are retryable
            Error::Timeout(_) => true,

            // Invalid response might be transient
            Error::InvalidResponse(_) => true,

            // API errors depend on status code
            Error::Api(detail) => {
                if let Some(status) = detail.status {
                    // 5xx errors are retryable
                    (500..600).contains(&status)
                } else {
                    false
                }
            }

            // These are permanent errors
            Error::Authentication(_)
            | Error::Authorization(_)
            | Error::InvalidRequest(_)
            | Error::NotFound(_)
            | Error::OAuth(_)
            | Error::Config(_)
            | Error::Serialization(_)
            | Error::StreamConnection(_)
            | Error::Io(_) => false,
        }
    }

    /// Get the duration to wait before retrying
    ///
    /// Returns `Some(Duration)` for errors that should be retried after a specific wait:
    /// - Rate limit errors: wait until reset time
    /// - API errors with Retry-After header
    ///
    /// Returns `None` for errors without specific retry timing (use exponential backoff)
    pub fn retry_after(&self) -> Option<std::time::Duration> {
        match self {
            Error::RateLimitExceeded { reset_at, .. } => {
                let now = Utc::now();
                if *reset_at > now {
                    (*reset_at - now).to_std().ok()
                } else {
                    Some(std::time::Duration::from_secs(0))
                }
            }
            // For other retryable errors, use exponential backoff (return None)
            _ => None,
        }
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Error::RateLimitExceeded { .. })
    }

    /// Check if this is an authentication/authorization error
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            Error::Authentication(_) | Error::Authorization(_) | Error::OAuth(_)
        )
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        match self {
            Error::Api(detail) => {
                if let Some(status) = detail.status {
                    (400..500).contains(&status)
                } else {
                    false
                }
            }
            Error::InvalidRequest(_) | Error::NotFound(_) => true,
            _ => false,
        }
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        match self {
            Error::Api(detail) => {
                if let Some(status) = detail.status {
                    (500..600).contains(&status)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl ApiErrorDetail {
    /// Create a new API error detail
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            parameter: None,
            value: None,
            type_uri: None,
            status: None,
        }
    }

    /// Set the HTTP status code
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the problematic parameter
    pub fn with_parameter(mut self, parameter: impl Into<String>) -> Self {
        self.parameter = Some(parameter.into());
        self
    }

    /// Set the problematic value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the problem type URI
    pub fn with_type_uri(mut self, type_uri: impl Into<String>) -> Self {
        self.type_uri = Some(type_uri.into());
        self
    }
}

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_error_io_is_retryable() {
        let err = Error::Io(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "connection refused",
        ));
        assert!(!err.is_retryable()); // IO errors are not retryable by default
    }

    #[test]
    fn test_error_rate_limit_is_retryable() {
        let reset_at = Utc::now() + Duration::seconds(60);
        let err = Error::RateLimitExceeded {
            reset_at,
            endpoint: "/2/tweets".to_string(),
            remaining: 0,
            limit: 100,
        };

        assert!(err.is_retryable());
        assert!(err.is_rate_limit());
        assert!(err.retry_after().is_some());

        let retry_duration = err.retry_after().unwrap();
        assert!(retry_duration.as_secs() > 0 && retry_duration.as_secs() <= 60);
    }

    #[test]
    fn test_error_authentication_not_retryable() {
        let err = Error::Authentication("Invalid credentials".to_string());
        assert!(!err.is_retryable());
        assert!(err.is_auth_error());
    }

    #[test]
    fn test_error_api_5xx_is_retryable() {
        let detail = ApiErrorDetail::new("INTERNAL_ERROR", "Server error").with_status(500);
        let err = Error::Api(detail);

        assert!(err.is_retryable());
        assert!(err.is_server_error());
        assert!(!err.is_client_error());
    }

    #[test]
    fn test_error_api_4xx_not_retryable() {
        let detail = ApiErrorDetail::new("BAD_REQUEST", "Invalid parameter").with_status(400);
        let err = Error::Api(detail);

        assert!(!err.is_retryable());
        assert!(err.is_client_error());
        assert!(!err.is_server_error());
    }

    #[test]
    fn test_api_error_detail_builder() {
        let detail = ApiErrorDetail::new("INVALID_PARAM", "Bad value")
            .with_status(400)
            .with_parameter("max_results")
            .with_value("1000")
            .with_type_uri("https://api.twitter.com/2/problems/invalid-request");

        assert_eq!(detail.code, "INVALID_PARAM");
        assert_eq!(detail.message, "Bad value");
        assert_eq!(detail.status, Some(400));
        assert_eq!(detail.parameter, Some("max_results".to_string()));
        assert_eq!(detail.value, Some("1000".to_string()));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout");
        let err: Error = io_err.into();

        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: Error = json_err.into();

        assert!(matches!(err, Error::Serialization(_)));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let err = Error::NotFound("Tweet ID 123".to_string());
        let display = format!("{}", err);
        assert!(display.contains("not found"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_api_error_detail_display() {
        let detail = ApiErrorDetail::new("TEST_CODE", "Test message");
        let display = format!("{}", detail);
        assert!(display.contains("Test message"));
        assert!(display.contains("TEST_CODE"));
    }

    #[test]
    fn test_stream_disconnected_is_retryable() {
        let err = Error::StreamDisconnected("Connection lost".to_string());
        assert!(err.is_retryable());
        assert!(err.retry_after().is_none());
    }

    #[test]
    fn test_timeout_is_retryable() {
        let err = Error::Timeout(std::time::Duration::from_secs(30));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_authorization_error_classification() {
        let err = Error::Authorization("Insufficient permissions".to_string());
        assert!(!err.is_retryable());
        assert!(err.is_auth_error());
    }
}
