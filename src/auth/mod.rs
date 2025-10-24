//! Authentication providers for X API v2
//!
//! This module provides authentication implementations for:
//! - OAuth 1.0a user context
//! - OAuth 2.0 bearer tokens (app-only)
//! - OAuth 2.0 PKCE with fine-grained scopes

pub mod oauth1;
pub mod oauth2;
pub mod bearer;
