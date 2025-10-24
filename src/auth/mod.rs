//! Authentication providers for X API v2
//!
//! This module provides authentication implementations for:
//! - OAuth 1.0a user context
//! - OAuth 2.0 bearer tokens (app-only)
//! - OAuth 2.0 PKCE with fine-grained scopes
//!
//! The [`AuthProvider`] trait provides a common interface for all authentication
//! methods, enabling flexible authentication strategies via dependency injection
//! and trait objects.

use crate::error::Result;
use async_trait::async_trait;

/// Authentication provider trait for X API v2
///
/// This trait abstracts over different authentication methods (OAuth 1.0a, OAuth 2.0, etc.)
/// and provides a uniform interface for authenticating requests.
///
/// # Design
///
/// The trait is designed to be:
/// - **Async-first**: Authentication may require network calls or cryptographic operations
/// - **Object-safe**: Supports dynamic dispatch via `Box<dyn AuthProvider>`
/// - **Send + Sync**: Safe to share across async tasks
///
/// # Implementation Notes
///
/// Implementations should:
/// - Modify the request to include authentication headers/parameters
/// - Return errors for unsupported endpoints via `supports_endpoint` check
/// - Be stateless where possible (or use interior mutability for caching)
///
/// # Examples
///
/// ```rust,ignore
/// use async_trait::async_trait;
/// use x_api_client::auth::AuthProvider;
/// use x_api_client::error::Result;
///
/// struct MyAuthProvider {
///     token: String,
/// }
///
/// #[async_trait]
/// impl AuthProvider for MyAuthProvider {
///     async fn authenticate(&self, mut req: reqwest::Request) -> Result<reqwest::Request> {
///         req.headers_mut().insert(
///             "Authorization",
///             format!("Bearer {}", self.token).parse().unwrap()
///         );
///         Ok(req)
///     }
///
///     fn supports_endpoint(&self, endpoint: &str) -> bool {
///         // All endpoints support bearer tokens
///         true
///     }
/// }
/// ```
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Authenticate an HTTP request by modifying it (typically adding headers)
    ///
    /// This method takes ownership of the request, modifies it to include
    /// authentication credentials, and returns the modified request.
    ///
    /// # Parameters
    ///
    /// * `req` - The HTTP request to authenticate
    ///
    /// # Returns
    ///
    /// * `Ok(reqwest::Request)` - The authenticated request
    /// * `Err(Error)` - Authentication error (invalid credentials, signature failure, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Credentials are invalid or expired
    /// - Signature generation fails (for OAuth 1.0a)
    /// - The endpoint is not supported by this authentication method
    async fn authenticate(&self, req: reqwest::Request) -> Result<reqwest::Request>;

    /// Check if this authentication provider supports the given endpoint
    ///
    /// Different authentication methods support different endpoints:
    /// - OAuth 1.0a: User-context endpoints (tweets, likes, follows, DMs)
    /// - OAuth 2.0 Bearer: App-only endpoints (public data)
    /// - OAuth 2.0 PKCE: Fine-grained user-context endpoints
    ///
    /// # Parameters
    ///
    /// * `endpoint` - The API endpoint path (e.g., "/2/tweets")
    ///
    /// # Returns
    ///
    /// * `true` - This provider can authenticate requests to this endpoint
    /// * `false` - This provider does not support this endpoint
    fn supports_endpoint(&self, endpoint: &str) -> bool;
}

pub mod bearer;
pub mod oauth1;
pub mod oauth2;

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock authentication provider for testing
    struct MockAuthProvider {
        should_fail: bool,
        supported_endpoints: Vec<String>,
    }

    #[async_trait]
    impl AuthProvider for MockAuthProvider {
        async fn authenticate(&self, mut req: reqwest::Request) -> Result<reqwest::Request> {
            if self.should_fail {
                return Err(crate::error::Error::Authentication(
                    "Mock authentication failure".to_string(),
                ));
            }

            // Add a mock Authorization header
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_static("Bearer mock_token"),
            );

            Ok(req)
        }

        fn supports_endpoint(&self, endpoint: &str) -> bool {
            self.supported_endpoints
                .iter()
                .any(|e| endpoint.starts_with(e))
        }
    }

    #[tokio::test]
    async fn test_auth_provider_trait_object_safe() {
        // Verify that AuthProvider can be used as a trait object
        let provider: Box<dyn AuthProvider> = Box::new(MockAuthProvider {
            should_fail: false,
            supported_endpoints: vec!["/2/tweets".to_string()],
        });

        // Verify supports_endpoint works via trait object
        assert!(provider.supports_endpoint("/2/tweets"));
        assert!(provider.supports_endpoint("/2/tweets/123"));
        assert!(!provider.supports_endpoint("/2/users"));
    }

    #[tokio::test]
    async fn test_auth_provider_authenticate_success() {
        let provider = MockAuthProvider {
            should_fail: false,
            supported_endpoints: vec!["/2/tweets".to_string()],
        };

        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "https://api.twitter.com/2/tweets".parse().unwrap(),
        );

        let authenticated_req = provider.authenticate(req).await.unwrap();

        // Verify the Authorization header was added
        assert!(
            authenticated_req
                .headers()
                .contains_key(reqwest::header::AUTHORIZATION)
        );
        assert_eq!(
            authenticated_req
                .headers()
                .get(reqwest::header::AUTHORIZATION)
                .unwrap(),
            "Bearer mock_token"
        );
    }

    #[tokio::test]
    async fn test_auth_provider_authenticate_failure() {
        let provider = MockAuthProvider {
            should_fail: true,
            supported_endpoints: vec!["/2/tweets".to_string()],
        };

        let req = reqwest::Request::new(
            reqwest::Method::GET,
            "https://api.twitter.com/2/tweets".parse().unwrap(),
        );

        let result = provider.authenticate(req).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::Error::Authentication(_)
        ));
    }

    #[tokio::test]
    async fn test_auth_provider_supports_endpoint() {
        let provider = MockAuthProvider {
            should_fail: false,
            supported_endpoints: vec!["/2/tweets".to_string(), "/2/users/".to_string()],
        };

        // Supported endpoints
        assert!(provider.supports_endpoint("/2/tweets"));
        assert!(provider.supports_endpoint("/2/tweets/123"));
        assert!(provider.supports_endpoint("/2/users/123"));
        assert!(provider.supports_endpoint("/2/users/123/likes"));

        // Unsupported endpoints (exact match "/2/users" without trailing slash)
        assert!(!provider.supports_endpoint("/2/spaces"));
        assert!(!provider.supports_endpoint("/2/lists"));
    }

    #[test]
    fn test_auth_provider_send_sync() {
        // Verify AuthProvider is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Box<dyn AuthProvider>>();
    }
}
