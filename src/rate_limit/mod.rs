//! Rate limit management
//!
//! Provides intelligent rate limit tracking and request queuing:
//! - Per-endpoint limit monitoring
//! - Automatic request pacing
//! - Proactive tracking using x-rate-limit-* headers

pub mod tracker;
pub mod queue;
pub mod middleware;
