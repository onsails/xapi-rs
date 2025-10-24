//! # X API v2 Client Library
//!
//! A type-safe, async-first Rust library for the X (Twitter) API v2.
//!
//! This library provides complete access to the X API v2 with:
//! - Multi-authentication support (OAuth 1.0a, OAuth 2.0, Bearer tokens)
//! - Intelligent rate limit management
//! - Automatic retry logic with exponential backoff
//! - Production-ready streaming support
//! - Type-safe request/response models
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! // Example usage will be added as implementation progresses
//! ```

pub mod auth;
pub mod builder;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod pagination;
pub mod rate_limit;
pub mod retry;
pub mod streaming;

// Re-export commonly used types
pub use client::Client;
pub use error::Error;
