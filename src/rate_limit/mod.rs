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
///
/// Fields are private to maintain encapsulation and allow future changes.
/// Use preset methods (`new()`, `disabled()`) or builder methods to configure.
#[derive(Debug, Clone, PartialEq)]
pub struct RateLimitConfig {
    /// Global concurrent request limit (default: None)
    global_limit: Option<u32>,

    /// Enable per-endpoint rate limit tracking (default: true)
    per_endpoint_tracking: bool,

    /// Automatically wait when rate limit is reached (default: true)
    auto_wait: bool,
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

    /// Get the global concurrent request limit
    pub fn global_limit(&self) -> Option<u32> {
        self.global_limit
    }

    /// Check if per-endpoint tracking is enabled
    pub fn per_endpoint_tracking(&self) -> bool {
        self.per_endpoint_tracking
    }

    /// Check if auto-wait is enabled
    pub fn auto_wait(&self) -> bool {
        self.auto_wait
    }

    /// Create a custom rate limit configuration with builder pattern
    pub fn custom() -> RateLimitConfigBuilder {
        RateLimitConfigBuilder::default()
    }
}

/// Builder for creating custom rate limit configurations
#[derive(Debug)]
pub struct RateLimitConfigBuilder {
    global_limit: Option<u32>,
    per_endpoint_tracking: bool,
    auto_wait: bool,
}

impl Default for RateLimitConfigBuilder {
    fn default() -> Self {
        Self {
            global_limit: None,
            per_endpoint_tracking: true,
            auto_wait: true,
        }
    }
}

impl RateLimitConfigBuilder {
    /// Set a global concurrent request limit
    pub fn global_limit(mut self, limit: u32) -> Self {
        self.global_limit = Some(limit);
        self
    }

    /// Enable or disable per-endpoint tracking
    pub fn per_endpoint_tracking(mut self, enabled: bool) -> Self {
        self.per_endpoint_tracking = enabled;
        self
    }

    /// Enable or disable auto-wait
    pub fn auto_wait(mut self, enabled: bool) -> Self {
        self.auto_wait = enabled;
        self
    }

    /// Build the rate limit configuration
    pub fn build(self) -> RateLimitConfig {
        RateLimitConfig {
            global_limit: self.global_limit,
            per_endpoint_tracking: self.per_endpoint_tracking,
            auto_wait: self.auto_wait,
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
