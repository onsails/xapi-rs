//! Shared types across models
//!
//! # Visibility Strategy
//!
//! Struct fields in this module are intentionally `pub` to support Serde
//! serialization/deserialization. These are pure data transfer objects (DTOs)
//! that represent the X API v2 response format.
//!
//! While this violates typical encapsulation practices, it's the recommended
//! approach for API client DTOs to:
//! - Enable zero-cost Serde derives
//! - Allow flexible field access patterns
//! - Maintain forward compatibility via `additional_fields`
//!
//! Validation should be performed at the API boundary, not on individual models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for a Tweet (64-bit integer represented as string)
///
/// X API v2 represents all IDs as strings to avoid precision loss in JSON parsing
pub type TweetId = String;

/// Unique identifier for a User (64-bit integer represented as string)
pub type UserId = String;

/// Unique identifier for a Media attachment (64-bit integer represented as string)
pub type MediaId = String;

/// Unique identifier for a Space (alphanumeric string)
pub type SpaceId = String;

/// Unique identifier for a List (64-bit integer represented as string)
pub type ListId = String;

/// Unique identifier for a filtered stream Rule (alphanumeric string)
pub type RuleId = String;

/// Unique identifier for a conversation thread (64-bit integer represented as string)
pub type ConversationId = String;

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Primary response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Expanded objects (users, tweets, media, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Includes>,

    /// Pagination and metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,

    /// Partial errors (non-fatal)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
}

/// Expanded objects included in responses
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Includes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::user::User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweets: Option<Vec<crate::models::tweet::Tweet>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<crate::models::media::Media>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<Place>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polls: Option<Vec<Poll>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Pagination and response metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ResponseMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_id: Option<TweetId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_id: Option<TweetId>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// API error detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code (CAPS_CASE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Human-readable message
    pub message: String,

    /// Problematic parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,

    /// Problematic value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Problem type URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_uri: Option<String>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Geographic place information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Place {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<GeoCoordinates>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Geographic coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub geo_type: String,

    pub coordinates: Vec<f64>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Poll information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Poll {
    pub id: String,

    pub options: Vec<PollOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_datetime: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_status: Option<String>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Poll option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub position: u32,
    pub label: String,
    pub votes: u32,
}

/// Reply settings for tweets
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReplySettings {
    Everyone,
    MentionedUsers,
    Following,
}

/// Visibility/withheld information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Withheld {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}
