//! Request builders for X API endpoints

use crate::models::common::{ReplySettings, TweetId};
use serde::Serialize;

/// Request to create a new tweet
///
/// Use `TweetRequest::builder()` for ergonomic construction.
#[derive(Debug, Clone, Serialize)]
pub struct TweetRequest {
    /// The text content of the tweet (required)
    pub text: String,

    /// Who can reply to this tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,

    /// Direct message deep link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_deep_link: Option<String>,

    /// Whether this tweet is for super followers only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_super_followers_only: Option<bool>,

    /// Media attachment IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,

    /// ID of tweet being quoted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tweet_id: Option<TweetId>,

    /// ID of tweet being replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in_reply_to_tweet_id")]
    pub reply_to_tweet_id: Option<TweetId>,
}

impl TweetRequest {
    /// Create a new tweet request with just text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reply_settings: None,
            direct_message_deep_link: None,
            for_super_followers_only: None,
            media_ids: None,
            quote_tweet_id: None,
            reply_to_tweet_id: None,
        }
    }

    /// Create a builder for constructing a tweet request
    pub fn builder() -> TweetRequestBuilder {
        TweetRequestBuilder::default()
    }
}

/// Builder for TweetRequest
#[derive(Debug, Default)]
pub struct TweetRequestBuilder {
    text: Option<String>,
    reply_settings: Option<ReplySettings>,
    direct_message_deep_link: Option<String>,
    for_super_followers_only: Option<bool>,
    media_ids: Option<Vec<String>>,
    quote_tweet_id: Option<TweetId>,
    reply_to_tweet_id: Option<TweetId>,
}

impl TweetRequestBuilder {
    /// Set the tweet text (required)
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set reply settings
    pub fn reply_settings(mut self, settings: ReplySettings) -> Self {
        self.reply_settings = Some(settings);
        self
    }

    /// Set as reply to another tweet
    pub fn reply_to(mut self, tweet_id: impl Into<TweetId>) -> Self {
        self.reply_to_tweet_id = Some(tweet_id.into());
        self
    }

    /// Quote another tweet
    pub fn quote(mut self, tweet_id: impl Into<TweetId>) -> Self {
        self.quote_tweet_id = Some(tweet_id.into());
        self
    }

    /// Add media attachments
    pub fn media(mut self, media_ids: Vec<String>) -> Self {
        self.media_ids = Some(media_ids);
        self
    }

    /// Mark as super followers only
    pub fn super_followers_only(mut self) -> Self {
        self.for_super_followers_only = Some(true);
        self
    }

    /// Set direct message deep link
    pub fn direct_message_deep_link(mut self, link: impl Into<String>) -> Self {
        self.direct_message_deep_link = Some(link.into());
        self
    }

    /// Build the tweet request
    ///
    /// # Errors
    ///
    /// Returns an error if text is not set
    pub fn build(self) -> crate::error::Result<TweetRequest> {
        let text = self.text.ok_or_else(|| {
            crate::error::Error::Config("Tweet text is required".to_string())
        })?;

        Ok(TweetRequest {
            text,
            reply_settings: self.reply_settings,
            direct_message_deep_link: self.direct_message_deep_link,
            for_super_followers_only: self.for_super_followers_only,
            media_ids: self.media_ids,
            quote_tweet_id: self.quote_tweet_id,
            reply_to_tweet_id: self.reply_to_tweet_id,
        })
    }
}

/// Response from deleting a tweet
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct DeleteResponse {
    /// Whether the deletion was successful
    pub deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tweet_request_new() {
        let request = TweetRequest::new("Hello, world!");
        assert_eq!(request.text, "Hello, world!");
        assert!(request.reply_settings.is_none());
    }

    #[test]
    fn test_tweet_request_builder() {
        let request = TweetRequest::builder()
            .text("Test tweet")
            .reply_settings(ReplySettings::MentionedUsers)
            .build();

        assert!(request.is_ok());
        let request = request.unwrap();
        assert_eq!(request.text, "Test tweet");
        assert_eq!(request.reply_settings, Some(ReplySettings::MentionedUsers));
    }

    #[test]
    fn test_tweet_request_builder_missing_text() {
        let result = TweetRequest::builder()
            .reply_settings(ReplySettings::Everyone)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_tweet_request_builder_with_reply() {
        let request = TweetRequest::builder()
            .text("Reply tweet")
            .reply_to("1234567890")
            .build()
            .unwrap();

        assert_eq!(request.reply_to_tweet_id, Some("1234567890".to_string()));
    }

    #[test]
    fn test_tweet_request_serialization() {
        let request = TweetRequest::new("Test");
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("text"));
    }
}
