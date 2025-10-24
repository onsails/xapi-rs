//! OAuth 1.0a authentication implementation
//!
//! This module provides OAuth 1.0a authentication for user-context endpoints
//! in the X API v2. OAuth 1.0a is required for operations that act on behalf
//! of a user, such as posting tweets, liking, retweeting, and managing follows.
//!
//! # Overview
//!
//! OAuth 1.0a uses HMAC-SHA1 signature generation to authenticate requests.
//! The implementation uses the `oauth1-request` crate for signature generation
//! and header construction.
//!
//! # Example
//!
//! ```rust,ignore
//! use x_api_client::auth::oauth1::OAuth1Provider;
//!
//! let provider = OAuth1Provider::new(
//!     "consumer_key",
//!     "consumer_secret",
//!     "access_token",
//!     "access_token_secret",
//! );
//!
//! // Use with XClient builder
//! let client = XClient::builder()
//!     .with_auth_provider(provider)
//!     .build()?;
//! ```

use async_trait::async_trait;
use oauth1_request as oauth;

use crate::auth::AuthProvider;
use crate::error::{Error, Result};

/// OAuth 1.0a authentication provider
///
/// This provider implements the OAuth 1.0a protocol for authenticating requests
/// to the X API v2. It supports user-context endpoints that require acting on
/// behalf of a specific user.
///
/// # Authentication Flow
///
/// 1. Constructs the signature base string from the HTTP request
/// 2. Generates HMAC-SHA1 signature using consumer and token secrets
/// 3. Injects the `Authorization` header with OAuth parameters
///
/// # Endpoint Support
///
/// OAuth 1.0a is required for user-context endpoints:
/// - `/2/tweets` (POST) - Creating tweets
/// - `/2/users/:id/likes` - Liking/unliking tweets
/// - `/2/users/:id/retweets` - Retweeting
/// - `/2/users/:id/following` - Following/unfollowing
/// - `/2/dm_conversations` - Direct messages
///
/// For app-only endpoints (public data), use `OAuth2BearerProvider` instead.
#[derive(Clone)]
pub struct OAuth1Provider {
    /// OAuth 1.0a credentials token
    #[allow(dead_code)] // Will be used in subtask 5.3 for signature generation
    token: oauth::Token,
}

impl OAuth1Provider {
    /// Create a new OAuth 1.0a provider with credentials
    ///
    /// # Parameters
    ///
    /// * `consumer_key` - Your app's consumer key (API key)
    /// * `consumer_secret` - Your app's consumer secret (API secret)
    /// * `access_token` - User's access token
    /// * `access_token_secret` - User's access token secret
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let provider = OAuth1Provider::new(
    ///     "your_consumer_key",
    ///     "your_consumer_secret",
    ///     "user_access_token",
    ///     "user_access_token_secret",
    /// );
    /// ```
    pub fn new(
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        access_token: impl Into<String>,
        access_token_secret: impl Into<String>,
    ) -> Self {
        let token = oauth::Token::from_parts(
            consumer_key.into(),
            consumer_secret.into(),
            access_token.into(),
            access_token_secret.into(),
        );

        Self { token }
    }

    /// Check if the given endpoint is a user-context endpoint requiring OAuth 1.0a
    ///
    /// This is a helper method used by `supports_endpoint` to determine if an
    /// endpoint requires user-context authentication.
    fn is_user_context_endpoint(endpoint: &str) -> bool {
        // App-only endpoints that should NOT use OAuth 1.0a (must check first)
        const EXCLUDED_PATTERNS: &[&str] = &[
            "/2/tweets/search",          // Tweet search endpoints
            "/2/tweets/sample",          // Sample stream
            "/2/tweets/count",           // Tweet counts
            "/2/compliance/jobs",        // Compliance
            "/2/openapi.json",           // OpenAPI spec
        ];

        // Check exclusions first
        if EXCLUDED_PATTERNS
            .iter()
            .any(|&pattern| endpoint.starts_with(pattern))
        {
            return false;
        }

        // Exact match patterns for user-context endpoints
        const EXACT_PATTERNS: &[&str] = &[
            "/2/tweets",           // Creating tweets (POST)
            "/2/dm_conversations", // Direct messages
            "/2/dm_events",        // DM events
            "/2/lists",            // List management
        ];

        // Prefix patterns for user-context endpoints
        const PREFIX_PATTERNS: &[&str] = &[
            "/2/tweets/",          // Individual tweet operations (DELETE /2/tweets/:id)
            "/2/users/",           // User actions (likes, follows, blocks, mutes)
            "/2/dm_conversations/", // DM conversation operations
            "/2/dm_events/",       // DM event operations
            "/2/lists/",           // List operations
        ];

        // Check exact matches
        if EXACT_PATTERNS.contains(&endpoint) {
            return true;
        }

        // Then check prefix matches
        PREFIX_PATTERNS
            .iter()
            .any(|pattern| endpoint.starts_with(pattern))
    }
}

#[async_trait]
impl AuthProvider for OAuth1Provider {
    async fn authenticate(&self, req: reqwest::Request) -> Result<reqwest::Request> {
        // Extract URL from the request
        let url = req.url();

        // Check if this endpoint supports OAuth 1.0a
        if !self.supports_endpoint(url.path()) {
            return Err(Error::Authentication(format!(
                "Endpoint '{}' does not support OAuth 1.0a authentication",
                url.path()
            )));
        }

        // For now, return the request unmodified
        // Signature generation will be implemented in subtask 5.3
        Ok(req)
    }

    fn supports_endpoint(&self, endpoint: &str) -> bool {
        Self::is_user_context_endpoint(endpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth1_provider_creation() {
        let provider = OAuth1Provider::new(
            "test_consumer_key",
            "test_consumer_secret",
            "test_access_token",
            "test_access_token_secret",
        );

        // Verify provider is created successfully
        assert!(!provider.token.client.identifier.is_empty());
    }

    #[test]
    fn test_oauth1_provider_clone() {
        let provider = OAuth1Provider::new(
            "test_consumer_key",
            "test_consumer_secret",
            "test_access_token",
            "test_access_token_secret",
        );

        let cloned = provider.clone();
        // Both should have the same credentials
        assert_eq!(
            provider.token.client.identifier,
            cloned.token.client.identifier
        );
    }

    #[test]
    fn test_supports_user_context_endpoints() {
        let provider = OAuth1Provider::new("ck", "cs", "at", "ats");

        // User-context endpoints (should support)
        assert!(provider.supports_endpoint("/2/tweets"));
        assert!(provider.supports_endpoint("/2/tweets/123"));
        assert!(provider.supports_endpoint("/2/users/123/likes"));
        assert!(provider.supports_endpoint("/2/users/456/following"));
        assert!(provider.supports_endpoint("/2/dm_conversations"));
        assert!(provider.supports_endpoint("/2/dm_events"));
        assert!(provider.supports_endpoint("/2/lists"));
        assert!(provider.supports_endpoint("/2/lists/123/members"));
    }

    #[test]
    fn test_does_not_support_app_only_endpoints() {
        let provider = OAuth1Provider::new("ck", "cs", "at", "ats");

        // App-only endpoints (should NOT support)
        assert!(!provider.supports_endpoint("/2/tweets/search/recent"));
        assert!(!provider.supports_endpoint("/2/compliance/jobs"));
        assert!(!provider.supports_endpoint("/2/openapi.json"));
    }

    #[tokio::test]
    async fn test_authenticate_supported_endpoint() {
        let provider = OAuth1Provider::new("ck", "cs", "at", "ats");

        let req = reqwest::Request::new(
            reqwest::Method::POST,
            "https://api.twitter.com/2/tweets".parse().unwrap(),
        );

        // Should not error for supported endpoint
        // (actual signature generation will be tested in subtask 5.3)
        let result = provider.authenticate(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_unsupported_endpoint() {
        let provider = OAuth1Provider::new("ck", "cs", "at", "ats");

        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "https://api.twitter.com/2/tweets/search/recent"
                .parse()
                .unwrap(),
        );

        // Should error for unsupported endpoint
        let result = provider.authenticate(req).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Authentication(_)));
    }

    #[test]
    fn test_auth_provider_trait_object() {
        let provider: Box<dyn AuthProvider> = Box::new(OAuth1Provider::new("ck", "cs", "at", "ats"));

        // Verify trait object works
        assert!(provider.supports_endpoint("/2/tweets"));
        assert!(!provider.supports_endpoint("/2/tweets/search/recent"));
    }
}
