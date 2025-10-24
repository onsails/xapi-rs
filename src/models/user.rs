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
