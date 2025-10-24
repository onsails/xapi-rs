//! Main Client struct and HTTP client abstraction
//!
//! Provides the primary interface for interacting with the X API v2

use crate::auth::AuthProvider;
use crate::error::Result;
use crate::rate_limit::RateLimitConfig;
use crate::retry::policy::RetryPolicy;
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

    /// Rate limiting configuration
    rate_limit_config: RateLimitConfig,

    /// Retry policy for failed requests
    retry_policy: RetryPolicy,

    /// Base URL for the X API (default: <https://api.twitter.com>)
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
            rate_limit_config: RateLimitConfig::default(),
            retry_policy: RetryPolicy::default(),
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

    /// Get the rate limit configuration
    #[allow(dead_code)] // Will be used by rate limit modules
    pub(crate) fn rate_limit_config(&self) -> &RateLimitConfig {
        &self.rate_limit_config
    }

    /// Get the retry policy
    #[allow(dead_code)] // Will be used by retry modules
    pub(crate) fn retry_policy(&self) -> &RetryPolicy {
        &self.retry_policy
    }
}

/// Builder for configuring and constructing a Client
///
/// Provides a fluent API for setting up authentication, HTTP client configuration,
/// and other client options.
pub struct ClientBuilder<H: HttpClient + Clone = ReqwestClient> {
    http: Option<H>,
    auth: Option<Arc<dyn AuthProvider>>,
    rate_limit_config: Option<RateLimitConfig>,
    retry_policy: Option<RetryPolicy>,
    base_url: Option<String>,
    timeout: Option<std::time::Duration>,
}

impl Default for ClientBuilder<ReqwestClient> {
    fn default() -> Self {
        Self {
            http: None,
            auth: None,
            rate_limit_config: None,
            retry_policy: None,
            base_url: None,
            timeout: None,
        }
    }
}

impl<H: HttpClient + Clone> ClientBuilder<H> {
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
    /// Default: `"https://api.twitter.com"`
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Configure rate limiting behavior
    ///
    /// Default: Per-endpoint tracking enabled, auto-wait enabled
    pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit_config = Some(config);
        self
    }

    /// Configure retry policy for failed requests
    ///
    /// Default: 3 retries with exponential backoff
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Set a custom HTTP client
    ///
    /// Note: If you set a custom HTTP client, the `timeout()` configuration will be ignored.
    /// Configure timeout on your custom HTTP client instead.
    pub fn http_client(mut self, http: H) -> Self {
        self.http = Some(http);
        self
    }
}

impl ClientBuilder<ReqwestClient> {
    /// Create a new ClientBuilder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the HTTP request timeout
    ///
    /// Default: 30 seconds
    ///
    /// Note: This method is only available when using the default `ReqwestClient`.
    /// If you provide a custom HTTP client via `http_client()`, configure timeout on that client instead.
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
    /// - Both timeout and custom HTTP client are configured (conflicting options)
    pub fn build(self) -> Result<Client<ReqwestClient>> {
        let auth = self.auth.ok_or_else(|| {
            crate::error::Error::Config(
                "No authentication provider configured. Use .oauth1() or .auth()".to_string(),
            )
        })?;

        // Check for conflicting configuration
        if self.http.is_some() && self.timeout.is_some() {
            return Err(crate::error::Error::Config(
                "Cannot set both custom HTTP client and timeout. Configure timeout on your custom client instead.".to_string()
            ));
        }

        let http = if let Some(http) = self.http {
            http
        } else if let Some(timeout) = self.timeout {
            ReqwestClient::with_timeout(timeout)?
        } else {
            ReqwestClient::new()?
        };

        Ok(Client {
            http,
            auth,
            rate_limit_config: self.rate_limit_config.unwrap_or_default(),
            retry_policy: self.retry_policy.unwrap_or_default(),
            base_url: self
                .base_url
                .unwrap_or_else(|| "https://api.twitter.com".to_string()),
        })
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
        let client = Client::builder().oauth1("ck", "cs", "at", "ats").build();

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

    #[test]
    fn test_client_builder_with_custom_auth() {
        let auth = Arc::new(crate::auth::oauth1::OAuth1Provider::new(
            "ck", "cs", "at", "ats",
        ));
        let client = Client::builder().auth(auth).build();
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_builder_timeout_and_http_client_conflict() {
        let http = ReqwestClient::new().unwrap();
        let result = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .http_client(http)
            .timeout(std::time::Duration::from_secs(60))
            .build();

        assert!(result.is_err());
        if let Err(crate::error::Error::Config(msg)) = result {
            assert!(msg.contains("Cannot set both"));
        } else {
            panic!("Expected Config error for conflicting options");
        }
    }

    #[test]
    fn test_client_builder_with_rate_limit() {
        let rate_config = RateLimitConfig::custom()
            .global_limit(100)
            .per_endpoint_tracking(true)
            .auto_wait(true)
            .build()
            .unwrap();

        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .rate_limit(rate_config.clone())
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.rate_limit_config().global_limit(), Some(100));
    }

    #[test]
    fn test_client_builder_with_retry_policy() {
        let retry_policy = RetryPolicy::custom()
            .max_retries(5)
            .initial_interval(std::time::Duration::from_millis(500))
            .max_interval(std::time::Duration::from_secs(30))
            .multiplier(2.0)
            .jitter(true)
            .build();

        assert!(retry_policy.is_ok());
        let retry_policy = retry_policy.unwrap();

        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .retry_policy(retry_policy.clone())
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.retry_policy().max_retries(), 5);
    }

    #[test]
    fn test_client_builder_full_configuration() {
        let client = Client::builder()
            .oauth1("ck", "cs", "at", "ats")
            .base_url("https://api.test.com")
            .timeout(std::time::Duration::from_secs(45))
            .rate_limit(RateLimitConfig::default())
            .retry_policy(RetryPolicy::aggressive())
            .build();

        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url(), "https://api.test.com");
        assert_eq!(client.retry_policy().max_retries(), 5); // aggressive = 5 retries
    }

    #[test]
    fn test_retry_policy_presets() {
        let default_policy = RetryPolicy::new();
        assert_eq!(default_policy.max_retries(), 3);

        let none_policy = RetryPolicy::none();
        assert_eq!(none_policy.max_retries(), 0);

        let aggressive_policy = RetryPolicy::aggressive();
        assert_eq!(aggressive_policy.max_retries(), 5);
    }

    #[test]
    fn test_rate_limit_config_presets() {
        let default_config = RateLimitConfig::new();
        assert!(default_config.per_endpoint_tracking());
        assert!(default_config.auto_wait());

        let disabled_config = RateLimitConfig::disabled();
        assert!(!disabled_config.per_endpoint_tracking());
        assert!(!disabled_config.auto_wait());
    }

    #[test]
    fn test_retry_policy_builder_validation() {
        // Test that validation works
        let result = RetryPolicy::custom().multiplier(-1.0).build();
        assert!(result.is_err());

        let result = RetryPolicy::custom()
            .initial_interval(std::time::Duration::from_secs(100))
            .max_interval(std::time::Duration::from_secs(10))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limit_config_builder() {
        let config = RateLimitConfig::custom()
            .global_limit(50)
            .per_endpoint_tracking(false)
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.global_limit(), Some(50));
        assert!(!config.per_endpoint_tracking());
    }

    #[test]
    fn test_rate_limit_config_builder_validation() {
        // Test that zero global_limit is rejected
        let result = RateLimitConfig::custom().global_limit(0).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_retry_policy_builder_nan_validation() {
        // Test that NaN is rejected
        let result = RetryPolicy::custom().multiplier(f64::NAN).build();
        assert!(result.is_err());

        // Test that Infinity is rejected
        let result = RetryPolicy::custom().multiplier(f64::INFINITY).build();
        assert!(result.is_err());
    }
}
