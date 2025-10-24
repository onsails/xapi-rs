//! Main Client struct and HTTP client abstraction
//!
//! Provides the primary interface for interacting with the X API v2

use crate::error::Result;
use std::future::Future;

/// HTTP client trait abstraction for testability and flexibility
///
/// This trait abstracts over the HTTP client implementation, allowing for:
/// - Testing with mock implementations
/// - Swapping HTTP backends if needed
/// - Request/response middleware integration
///
/// The trait requires Send + Sync for use across async tasks.
pub trait HttpClient: Send + Sync {
    /// Send an HTTP request and return the response
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails at the network level.
    /// Note: HTTP errors (4xx, 5xx) should be returned as successful
    /// responses with error status codes.
    fn execute(
        &self,
        request: reqwest::Request,
    ) -> impl Future<Output = Result<reqwest::Response>> + Send;
}

/// Default HTTP client implementation using reqwest
///
/// Note: `reqwest::Client` is already wrapped in Arc internally and is Clone-able,
/// so we use it directly without additional Arc wrapping.
#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    /// Create a new ReqwestClient with default configuration
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }

    /// Create a new ReqwestClient with custom reqwest::Client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Create a new ReqwestClient with custom timeout
    pub fn with_timeout(timeout: std::time::Duration) -> Result<Self> {
        let client = reqwest::Client::builder().timeout(timeout).build()?;

        Ok(Self { client })
    }

    /// Get a reference to the underlying reqwest::Client
    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }
}

// Note: No Default implementation to avoid potential panics in library code.
// Use ReqwestClient::new() instead, which returns Result for proper error handling.

impl HttpClient for ReqwestClient {
    async fn execute(&self, request: reqwest::Request) -> Result<reqwest::Response> {
        let response = self.client.execute(request).await?;
        Ok(response)
    }
}

/// The main client for interacting with the X API v2
#[derive(Clone)]
pub struct Client<H: HttpClient + Clone = ReqwestClient> {
    #[allow(dead_code)] // Will be used in endpoint implementations
    http: H,
}

impl Client<ReqwestClient> {
    /// Create a new Client with default HTTP client
    pub fn new() -> Result<Self> {
        Ok(Self {
            http: ReqwestClient::new()?,
        })
    }
}

impl<H: HttpClient + Clone> Client<H> {
    /// Create a new Client with a custom HTTP client
    pub fn with_http_client(http: H) -> Self {
        Self { http }
    }

    /// Get a reference to the HTTP client
    #[allow(dead_code)] // Will be used by endpoint modules
    pub(crate) fn http_client(&self) -> &H {
        &self.http
    }
}

// Note: No Default implementation to avoid potential panics in library code.
// Use Client::new() instead, which returns Result for proper error handling.

#[cfg(test)]
mod tests {
    //! # Testing with HttpClient
    //!
    //! For testing code that uses `HttpClient`, we recommend using the `wiremock` or `mockito`
    //! crates to create HTTP mock servers. These provide proper HTTP response mocking with
    //! full control over status codes, headers, and body content.
    //!
    //! Example with wiremock (add to dev-dependencies):
    //! ```toml
    //! [dev-dependencies]
    //! wiremock = "0.6"
    //! ```

    use super::*;

    #[tokio::test]
    async fn test_reqwest_client_creation() {
        let client = ReqwestClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_reqwest_client_with_client() {
        let reqwest_client = reqwest::Client::new();
        let _client = ReqwestClient::with_client(reqwest_client);
        // Should compile and create successfully
    }

    #[tokio::test]
    async fn test_reqwest_client_with_timeout() {
        let client = ReqwestClient::with_timeout(std::time::Duration::from_secs(60));
        assert!(client.is_ok());
    }

    #[test]
    fn test_reqwest_client_inner() {
        let client = ReqwestClient::new().unwrap();
        let _inner = client.inner();
        // Verify we can access the inner client
    }

    #[test]
    fn test_reqwest_client_clone() {
        let client = ReqwestClient::new().unwrap();
        let cloned = client.clone();
        // Both should work independently (Clone is cheap for reqwest::Client)
        let _inner1 = client.inner();
        let _inner2 = cloned.inner();
    }

    #[test]
    fn test_client_clone() {
        let client = Client::new().unwrap();
        let _cloned = client.clone();
        // Verify Client is cloneable
    }

    #[test]
    fn test_http_client_trait_bounds() {
        // Verify HttpClient is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ReqwestClient>();
    }
}
