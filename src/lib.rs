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
//! use x_api_client::Client;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client with OAuth 1.0a authentication
//! let client = Client::builder()
//!     .oauth1("consumer_key", "consumer_secret", "access_token", "token_secret")
//!     .build()?;
//! # Ok(())
//! # }
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
pub mod util;

// Re-export commonly used types
pub use client::Client;
pub use error::Error;
