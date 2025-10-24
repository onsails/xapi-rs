//! Rate limit management
//!
//! Provides intelligent rate limit tracking and request queuing:
//! - Per-endpoint limit monitoring
//! - Automatic request pacing
//! - Proactive tracking using x-rate-limit-* headers

pub mod tracker;
pub mod queue;
pub mod middleware;

/// Rate limit configuration
///
/// Configures how the client handles rate limiting from the X API.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Global concurrent request limit (default: None)
    pub global_limit: Option<u32>,

    /// Enable per-endpoint rate limit tracking (default: true)
    pub per_endpoint_tracking: bool,

    /// Automatically wait when rate limit is reached (default: true)
    pub auto_wait: bool,
}

impl RateLimitConfig {
    /// Create a new rate limit configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration that disables rate limiting
    pub fn disabled() -> Self {
        Self {
            global_limit: None,
            per_endpoint_tracking: false,
            auto_wait: false,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            global_limit: None,
            per_endpoint_tracking: true,
            auto_wait: true,
        }
    }
}
