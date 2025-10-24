//! Automatic retry logic
//!
//! Provides configurable retry strategies with:
//! - Intelligent error classification
//! - Exponential backoff with jitter
//! - Request deduplication

pub mod policy;
pub mod classifier;
