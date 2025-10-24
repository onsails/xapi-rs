//! Main Client struct and HTTP client abstraction
//!
//! Provides the primary interface for interacting with the X API v2

use crate::auth::AuthProvider;
use crate::error::Result;
use std::future::Future;
use std::sync::Arc;

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
///
/// This is the primary entry point for making requests to the X API.
/// Use `Client::builder()` for full configuration options.
///
/// The client uses `Arc<dyn AuthProvider>` to allow cloning while sharing
/// the authentication provider across multiple client instances.
#[derive(Clone)]
pub struct Client<H: HttpClient + Clone = ReqwestClient> {
    /// HTTP client for making requests
    http: H,

    /// Authentication provider for signing requests (Arc for cloneability)
    auth: Arc<dyn AuthProvider>,

    /// Base URL for the X API (default: https://api.twitter.com)
    base_url: String,
}

impl Client<ReqwestClient> {
    /// Create a new Client with OAuth 1.0a authentication
    ///
    /// # Arguments
    ///
    /// * `consumer_key` - OAuth consumer key
    /// * `consumer_secret` - OAuth consumer secret
    /// * `access_token` - OAuth access token
    /// * `access_token_secret` - OAuth access token secret
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use x_api_client::Client;
    ///
    /// let client = Client::new(
    ///     "consumer_key",
    ///     "consumer_secret",
    ///     "access_token",
    ///     "access_token_secret",
    /// )?;
    /// ```
    pub fn new(
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        access_token: impl Into<String>,
        access_token_secret: impl Into<String>,
    ) -> Result<Self> {
        let auth = crate::auth::oauth1::OAuth1Provider::new(
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        );

        Ok(Self {
            http: ReqwestClient::new()?,
            auth: Arc::new(auth),
            base_url: "https://api.twitter.com".to_string(),
        })
    }

    /// Create a new ClientBuilder for advanced configuration
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use x_api_client::Client;
    /// use std::time::Duration;
    ///
    /// let client = Client::builder()
    ///     .oauth1("ck", "cs", "at", "ats")
    ///     .timeout(Duration::from_secs(60))
    ///     .base_url("https://api.twitter.com")
    ///     .build()?;
    /// ```
    pub fn builder() -> ClientBuilder<ReqwestClient> {
        ClientBuilder::new()
    }
}

impl<H: HttpClient + Clone> Client<H> {
    /// Get a reference to the HTTP client
    #[allow(dead_code)] // Will be used by endpoint modules
    pub(crate) fn http_client(&self) -> &H {
        &self.http
    }

    /// Get a reference to the auth provider
    #[allow(dead_code)] // Will be used by endpoint modules
    pub(crate) fn auth_provider(&self) -> &dyn AuthProvider {
        &*self.auth
    }

    /// Get the base URL
    #[allow(dead_code)] // Will be used by endpoint modules
    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }
}

/// Builder for configuring and constructing a Client
///
/// Provides a fluent API for setting up authentication, HTTP client configuration,
/// and other client options.
pub struct ClientBuilder<H: HttpClient + Clone = ReqwestClient> {
    http: Option<H>,
    auth: Option<Arc<dyn AuthProvider>>,
    base_url: Option<String>,
    timeout: Option<std::time::Duration>,
}

impl Default for ClientBuilder<ReqwestClient> {
    fn default() -> Self {
        Self {
            http: None,
            auth: None,
            base_url: None,
            timeout: None,
        }
    }
}

impl ClientBuilder<ReqwestClient> {
    /// Create a new ClientBuilder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure OAuth 1.0a authentication
    ///
    /// # Arguments
    ///
    /// * `consumer_key` - OAuth consumer key
    /// * `consumer_secret` - OAuth consumer secret
    /// * `access_token` - OAuth access token
    /// * `access_token_secret` - OAuth access token secret
    pub fn oauth1(
        mut self,
        consumer_key: impl Into<String>,
        consumer_secret: impl Into<String>,
        access_token: impl Into<String>,
        access_token_secret: impl Into<String>,
    ) -> Self {
        self.auth = Some(Arc::new(crate::auth::oauth1::OAuth1Provider::new(
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        )));
        self
    }

    /// Set a custom authentication provider
    pub fn auth(mut self, auth: Arc<dyn AuthProvider>) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Set the base URL for the X API
    ///
    /// Default: "https://api.twitter.com"
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the HTTP request timeout
    ///
    /// Default: 30 seconds
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the Client
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No authentication provider is configured
    /// - HTTP client creation fails
    pub fn build(self) -> Result<Client<ReqwestClient>> {
        let auth = self
            .auth
            .ok_or_else(|| crate::error::Error::Config("No authentication provider configured. Use .oauth1() or .auth()".to_string()))?;

        let http = if let Some(timeout) = self.timeout {
            ReqwestClient::with_timeout(timeout)?
        } else {
            ReqwestClient::new()?
        };

        Ok(Client {
            http,
            auth,
            base_url: self.base_url.unwrap_or_else(|| "https://api.twitter.com".to_string()),
        })
    }
}

impl<H: HttpClient + Clone> ClientBuilder<H> {
    /// Set a custom HTTP client
    pub fn http_client(mut self, http: H) -> Self {
        self.http = Some(http);
        self
    }
}

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
        let client = Client::new("test_ck", "test_cs", "test_at", "test_ats");
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
        let client = Client::new("test_ck", "test_cs", "test_at", "test_ats").unwrap();
        let _cloned = client.clone();
        // Verify Client is cloneable
    }

    #[test]
    fn test_http_client_trait_bounds() {
        // Verify HttpClient is Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ReqwestClient>();
    }

    #[test]
    fn test_client_builder_oauth1() {
        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://api.twitter.com");
    }

    #[test]
    fn test_client_builder_with_base_url() {
        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .base_url("https://api.test.com")
            .build();

        assert!(client.is_ok());
        assert_eq!(client.unwrap().base_url(), "https://api.test.com");
    }

    #[test]
    fn test_client_builder_with_timeout() {
        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .timeout(std::time::Duration::from_secs(60))
            .build();

        assert!(client.is_ok());
    }

    #[test]
    fn test_client_builder_no_auth_fails() {
        let result = Client::builder().build();

        assert!(result.is_err());
        if let Err(crate::error::Error::Config(msg)) = result {
            assert!(msg.contains("No authentication provider"));
        } else {
            panic!("Expected Config error");
        }
    }

    #[test]
    fn test_client_new_with_oauth1() {
        let client = Client::new("ck", "cs", "at", "ats");
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://api.twitter.com");
    }

    #[test]
    fn test_client_accessors() {
        let client = Client::new("ck", "cs", "at", "ats").unwrap();
        
        // Verify we can access internal components
        let _http = client.http_client();
        let _auth = client.auth_provider();
        let _base = client.base_url();
    }
}
