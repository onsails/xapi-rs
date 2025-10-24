//! Retry policies

use std::time::Duration;

/// Retry policy configuration for failed requests
///
/// Configures the exponential backoff retry behavior for transient errors.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (default: 3)
    pub max_retries: u32,

    /// Initial retry interval (default: 1 second)
    pub initial_interval: Duration,

    /// Maximum retry interval (default: 60 seconds)
    pub max_interval: Duration,

    /// Backoff multiplier (default: 2.0 for exponential backoff)
    pub multiplier: f64,

    /// Whether to add jitter to retry intervals (default: true)
    pub jitter: bool,
}

impl RetryPolicy {
    /// Create a new retry policy with default values
    ///
    /// Defaults:
    /// - max_retries: 3
    /// - initial_interval: 1 second
    /// - max_interval: 60 seconds
    /// - multiplier: 2.0 (exponential)
    /// - jitter: true
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a retry policy with no retries (fail fast)
    pub fn none() -> Self {
        Self {
            max_retries: 0,
            initial_interval: Duration::from_secs(1),
            max_interval: Duration::from_secs(1),
            multiplier: 1.0,
            jitter: false,
        }
    }

    /// Create a retry policy with aggressive retries
    pub fn aggressive() -> Self {
        Self {
            max_retries: 5,
            initial_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: true,
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_interval: Duration::from_secs(1),
            max_interval: Duration::from_secs(60),
            multiplier: 2.0,
            jitter: true,
        }
    }
}
