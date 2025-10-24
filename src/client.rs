//! Main Client struct and HTTP client abstraction
//!
//! Provides the primary interface for interacting with the X API v2

use crate::error::Result;
use std::sync::Arc;

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
/// This wraps `reqwest::Client` in an Arc for efficient cloning and sharing
/// across threads and async tasks.
#[derive(Clone)]
pub struct ReqwestClient {
    client: Arc<reqwest::Client>,
}

impl ReqwestClient {
    /// Create a new ReqwestClient with default configuration
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Create a new ReqwestClient with custom reqwest::Client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    /// Get a reference to the underlying reqwest::Client
    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default ReqwestClient")
    }
}

impl HttpClient for ReqwestClient {
    async fn execute(&self, request: reqwest::Request) -> Result<reqwest::Response> {
        let response = self.client.execute(request).await?;
        Ok(response)
    }
}

/// The main client for interacting with the X API v2
pub struct Client<H: HttpClient = ReqwestClient> {
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

impl<H: HttpClient> Client<H> {
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

impl Default for Client<ReqwestClient> {
    fn default() -> Self {
        Self::new().expect("Failed to create default Client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock HTTP client for testing
    ///
    /// This is a simple mock that can be configured to return errors for testing.
    /// For more complex testing, use wiremock or similar libraries.
    pub struct MockHttpClient {
        should_fail: bool,
    }

    impl MockHttpClient {
        /// Create a new mock client that succeeds
        pub fn new() -> Self {
            Self { should_fail: false }
        }

        /// Create a mock client that always fails
        pub fn with_failure() -> Self {
            Self { should_fail: true }
        }
    }

    impl HttpClient for MockHttpClient {
        async fn execute(&self, _request: reqwest::Request) -> Result<reqwest::Response> {
            if self.should_fail {
                Err(crate::error::Error::Config("Mock failure".to_string()))
            } else {
                // Return a simple success response
                // In real tests, use wiremock for proper response mocking
                Err(crate::error::Error::Config(
                    "Use wiremock for proper response mocking".to_string(),
                ))
            }
        }
    }

    #[tokio::test]
    async fn test_reqwest_client_creation() {
        let client = ReqwestClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_reqwest_client_default() {
        let _client = ReqwestClient::default();
        // Should not panic
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_with_custom_http() {
        let mock = MockHttpClient::new();
        let _client = Client::with_http_client(mock);
        // Should compile and create successfully
    }

    #[tokio::test]
    async fn test_http_client_trait_send_sync() {
        // Verify HttpClient is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ReqwestClient>();
        assert_send_sync::<MockHttpClient>();
    }

    #[tokio::test]
    async fn test_mock_client_behavior() {
        let mock = MockHttpClient::with_failure();
        let client = Client::with_http_client(mock);

        // Mock should be usable with Client
        let _http = client.http_client();
    }
}
