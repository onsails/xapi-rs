//! Tweet data structures
//!
//! # Visibility Strategy
//!
//! Struct fields are intentionally `pub` to support Serde serialization/deserialization.
//! See module documentation in `models/common.rs` for rationale.

use crate::models::common::{ReplySettings, TweetId, UserId, Withheld};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Tweet object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweet {
    /// Unique identifier of this Tweet
    pub id: TweetId,

    /// The actual UTF-8 text of the Tweet
    pub text: String,

    /// Unique identifiers indicating all versions of a Tweet (for edited tweets)
    #[serde(default)]
    pub edit_history_tweet_ids: Vec<TweetId>,

    // Optional fields via TweetFields expansion
    /// Unique identifier of the user who posted this Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<UserId>,

    /// Creation time of the Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The Tweet ID of the original Tweet of the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<TweetId>,

    /// Public engagement metrics for the Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<TweetMetrics>,

    /// Non-public engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<TweetMetrics>,

    /// Organic engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<TweetMetrics>,

    /// Promoted engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<TweetMetrics>,

    /// Specifies the type of attachments present in this Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,

    /// Contains details about text that has a special meaning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,

    /// Contains details about geographic information tagged by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    /// If the Tweet is a reply, this field will contain the original Tweet's author ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_user_id: Option<UserId>,

    /// Language of the Tweet (BCP47 language tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Indicates if this Tweet contains URLs marked as possibly sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,

    /// A list of Tweets this Tweet refers to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,

    /// Shows who can reply to this Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,

    /// The name of the app used to post this Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Contains withholding details for content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,

    /// Context annotations (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_annotations: Option<Vec<ContextAnnotation>>,

    /// Edit controls information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_controls: Option<EditControls>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Engagement metrics for a Tweet
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TweetMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweet_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_count: Option<u64>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Tweet attachments (media, polls, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Attachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_keys: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_ids: Option<Vec<String>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Entities within a Tweet (URLs, hashtags, mentions, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Entities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<HashtagEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<MentionEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<CashtagEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// URL entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEntity {
    pub start: usize,
    pub end: usize,
    pub url: String,
    pub expanded_url: Option<String>,
    pub display_url: Option<String>,
    pub unwound_url: Option<String>,
}

/// Hashtag entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

/// Mention entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionEntity {
    pub start: usize,
    pub end: usize,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<UserId>,
}

/// Cashtag entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

/// Annotation (entity recognition)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub start: usize,
    pub end: usize,
    pub probability: f64,
    #[serde(rename = "type")]
    pub annotation_type: String,
    pub normalized_text: String,
}

/// Geographic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GeoPoint>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Geographic point coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    #[serde(rename = "type")]
    pub point_type: String,
    pub coordinates: Vec<f64>, // [longitude, latitude]
}

/// Referenced tweet (quote, retweet, reply)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencedTweet {
    pub id: TweetId,
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
}

/// Type of tweet reference
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReferenceType {
    Retweeted,
    Quoted,
    RepliedTo,
}

/// Context annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: ContextDomain,
    pub entity: ContextEntity,
}

/// Context domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDomain {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Context entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEntity {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Edit controls information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditControls {
    pub edits_remaining: u32,
    pub is_edit_eligible: bool,
    pub editable_until: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tweet_minimal_roundtrip() {
        let json = r#"{
            "id": "1234567890",
            "text": "Hello, world!",
            "edit_history_tweet_ids": ["1234567890"]
        }"#;

        let tweet: Tweet = serde_json::from_str(json).unwrap();
        assert_eq!(tweet.id, "1234567890");
        assert_eq!(tweet.text, "Hello, world!");

        let serialized = serde_json::to_string(&tweet).unwrap();
        let roundtrip: Tweet = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tweet.id, roundtrip.id);
        assert_eq!(tweet.text, roundtrip.text);
    }

    #[test]
    fn test_tweet_unknown_fields_captured() {
        let json = r#"{
            "id": "123",
            "text": "test",
            "edit_history_tweet_ids": [],
            "unknown_future_field": "some_value",
            "experimental_metric": 42
        }"#;

        let tweet: Tweet = serde_json::from_str(json).unwrap();
        assert_eq!(tweet.additional_fields.len(), 2);
        assert!(tweet.additional_fields.contains_key("unknown_future_field"));
        assert!(tweet.additional_fields.contains_key("experimental_metric"));

        // Verify roundtrip preserves unknown fields
        let serialized = serde_json::to_string(&tweet).unwrap();
        let roundtrip: Tweet = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtrip.additional_fields.len(), 2);
    }

    #[test]
    fn test_tweet_optional_fields_missing() {
        let json = r#"{
            "id": "123",
            "text": "test",
            "edit_history_tweet_ids": []
        }"#;

        let tweet: Tweet = serde_json::from_str(json).unwrap();
        assert!(tweet.author_id.is_none());
        assert!(tweet.created_at.is_none());
        assert!(tweet.public_metrics.is_none());
        assert!(tweet.entities.is_none());
    }

    #[test]
    fn test_tweet_metrics_unknown_fields() {
        let json = r#"{
            "like_count": 100,
            "retweet_count": 50,
            "future_engagement_score": 95.5
        }"#;

        let metrics: TweetMetrics = serde_json::from_str(json).unwrap();
        assert_eq!(metrics.like_count, Some(100));
        assert_eq!(metrics.additional_fields.len(), 1);
        assert!(
            metrics
                .additional_fields
                .contains_key("future_engagement_score")
        );
    }

    #[test]
    fn test_reference_type_serialization() {
        let ref_type = ReferenceType::Retweeted;
        let json = serde_json::to_string(&ref_type).unwrap();
        assert_eq!(json, r#""retweeted""#);

        let ref_type = ReferenceType::Quoted;
        let json = serde_json::to_string(&ref_type).unwrap();
        assert_eq!(json, r#""quoted""#);
    }

    #[test]
    fn test_entities_roundtrip() {
        let json = r#"{
            "urls": [
                {"start": 0, "end": 23, "url": "https://t.co/abc", "expanded_url": "https://example.com", "display_url": "example.com", "unwound_url": null}
            ],
            "hashtags": [
                {"start": 24, "end": 30, "tag": "rust"}
            ]
        }"#;

        let entities: Entities = serde_json::from_str(json).unwrap();
        assert!(entities.urls.is_some());
        assert_eq!(entities.urls.as_ref().unwrap().len(), 1);
        assert!(entities.hashtags.is_some());
        assert_eq!(entities.hashtags.as_ref().unwrap()[0].tag, "rust");
    }
}
