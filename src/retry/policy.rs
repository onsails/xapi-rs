//! Retry policies

use std::time::Duration;

/// Retry policy configuration for failed requests
///
/// Configures the exponential backoff retry behavior for transient errors.
///
/// Fields are private to enforce validation and maintain invariants.
/// Use preset methods (`new()`, `none()`, `aggressive()`) or builder methods
/// to configure.
#[derive(Debug, Clone, PartialEq)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (default: 3)
    max_retries: u32,

    /// Initial retry interval (default: 1 second)
    initial_interval: Duration,

    /// Maximum retry interval (default: 60 seconds)
    max_interval: Duration,

    /// Backoff multiplier (default: 2.0 for exponential backoff)
    multiplier: f64,

    /// Whether to add jitter to retry intervals (default: true)
    jitter: bool,
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

    /// Get the maximum number of retries
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    /// Get the initial retry interval
    pub fn initial_interval(&self) -> Duration {
        self.initial_interval
    }

    /// Get the maximum retry interval
    pub fn max_interval(&self) -> Duration {
        self.max_interval
    }

    /// Get the backoff multiplier
    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    /// Check if jitter is enabled
    pub fn jitter(&self) -> bool {
        self.jitter
    }

    /// Create a custom retry policy with builder pattern
    pub fn custom() -> RetryPolicyBuilder {
        RetryPolicyBuilder::default()
    }
}

/// Builder for creating custom retry policies with validation
#[derive(Debug)]
pub struct RetryPolicyBuilder {
    max_retries: u32,
    initial_interval: Duration,
    max_interval: Duration,
    multiplier: f64,
    jitter: bool,
}

impl Default for RetryPolicyBuilder {
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

impl RetryPolicyBuilder {
    /// Set the maximum number of retries
    pub fn max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// Set the initial retry interval
    pub fn initial_interval(mut self, interval: Duration) -> Self {
        self.initial_interval = interval;
        self
    }

    /// Set the maximum retry interval
    pub fn max_interval(mut self, interval: Duration) -> Self {
        self.max_interval = interval;
        self
    }

    /// Set the backoff multiplier
    pub fn multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Enable or disable jitter
    pub fn jitter(mut self, enabled: bool) -> Self {
        self.jitter = enabled;
        self
    }

    /// Build the retry policy
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - multiplier is <= 0, NaN, or Infinity
    /// - initial_interval > max_interval
    pub fn build(self) -> crate::error::Result<RetryPolicy> {
        if self.multiplier <= 0.0 || !self.multiplier.is_finite() {
            return Err(crate::error::Error::Config(
                "Retry multiplier must be positive and finite".to_string(),
            ));
        }

        if self.initial_interval > self.max_interval {
            return Err(crate::error::Error::Config(
                "Initial interval must be <= max interval".to_string(),
            ));
        }

        Ok(RetryPolicy {
            max_retries: self.max_retries,
            initial_interval: self.initial_interval,
            max_interval: self.max_interval,
            multiplier: self.multiplier,
            jitter: self.jitter,
        })
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
