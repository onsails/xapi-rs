//! User data structures
//!
//! # Visibility Strategy
//!
//! Struct fields are intentionally `pub` to support Serde serialization/deserialization.
//! See module documentation in `models/common.rs` for rationale.

use crate::models::common::{TweetId, UserId, Withheld};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A User object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier of this user
    pub id: UserId,

    /// The Twitter handle (screen name) of this user
    pub name: String,

    /// The username of the user (without @ symbol)
    pub username: String,

    // Optional fields via UserFields expansion
    /// Creation time of this account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// The text of this user's profile description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Contains details about text that has a special meaning in the user's description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<UserEntities>,

    /// The location specified in the user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Unique identifier of the user's pinned Tweet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tweet_id: Option<TweetId>,

    /// The URL to the profile image for this user (normal size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,

    /// Indicates if this user has chosen to protect their Tweets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,

    /// Public engagement metrics for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<UserMetrics>,

    /// The URL specified in the user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Indicates if this user is a verified user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// The type of verification for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_type: Option<VerifiedType>,

    /// Contains withholding details for content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// User engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<u64>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Entities in user profile (URLs, description)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<UserUrlEntity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<UserDescriptionEntity>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// URL entities in user profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUrlEntity {
    pub urls: Vec<crate::models::tweet::UrlEntity>,
}

/// Description entities in user profile
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserDescriptionEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<crate::models::tweet::UrlEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<crate::models::tweet::HashtagEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<crate::models::tweet::MentionEntity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<crate::models::tweet::CashtagEntity>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Type of user verification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum VerifiedType {
    /// Blue checkmark (verified)
    Blue,
    /// Government account
    Government,
    /// Business account
    Business,
    /// No verification
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_minimal_roundtrip() {
        let json = r#"{
            "id": "123456",
            "name": "Test User",
            "username": "testuser"
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, "123456");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.username, "testuser");

        let serialized = serde_json::to_string(&user).unwrap();
        let roundtrip: User = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user.id, roundtrip.id);
    }

    #[test]
    fn test_user_unknown_fields_captured() {
        let json = r#"{
            "id": "123",
            "name": "Test",
            "username": "test",
            "future_badge": "premium",
            "experimental_score": 98.5
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.additional_fields.len(), 2);
        assert!(user.additional_fields.contains_key("future_badge"));
        assert!(user.additional_fields.contains_key("experimental_score"));

        // Verify roundtrip
        let serialized = serde_json::to_string(&user).unwrap();
        let roundtrip: User = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtrip.additional_fields.len(), 2);
    }

    #[test]
    fn test_user_metrics_roundtrip() {
        let json = r#"{
            "followers_count": 1000,
            "following_count": 500,
            "tweet_count": 2500
        }"#;

        let metrics: UserMetrics = serde_json::from_str(json).unwrap();
        assert_eq!(metrics.followers_count, Some(1000));
        assert_eq!(metrics.following_count, Some(500));
    }

    #[test]
    fn test_verified_type_serialization() {
        let vtype = VerifiedType::Blue;
        let json = serde_json::to_string(&vtype).unwrap();
        assert_eq!(json, r#""blue""#);

        let vtype = VerifiedType::Government;
        let json = serde_json::to_string(&vtype).unwrap();
        assert_eq!(json, r#""government""#);
    }
}
