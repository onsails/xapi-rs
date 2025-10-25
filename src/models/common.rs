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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_roundtrip() {
        let json = r#"{"data":{"id":"123"},"meta":{"result_count":1}}"#;
        let response: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ApiResponse<serde_json::Value> =
            serde_json::from_str(&serialized).unwrap();

        assert!(response.data.is_some());
        assert!(deserialized.data.is_some());
    }

    #[test]
    fn test_response_meta_roundtrip() {
        let json = r#"{
            "result_count": 10,
            "next_token": "abc123",
            "newest_id": "999",
            "oldest_id": "111"
        }"#;

        let meta: ResponseMeta = serde_json::from_str(json).unwrap();
        assert_eq!(meta.result_count, Some(10));
        assert_eq!(meta.next_token, Some("abc123".to_string()));

        let serialized = serde_json::to_string(&meta).unwrap();
        let roundtrip: ResponseMeta = serde_json::from_str(&serialized).unwrap();
        assert_eq!(meta.result_count, roundtrip.result_count);
    }

    #[test]
    fn test_response_meta_unknown_fields_captured() {
        let json = r#"{
            "result_count": 5,
            "future_field": "new_value",
            "experimental_count": 100
        }"#;

        let meta: ResponseMeta = serde_json::from_str(json).unwrap();
        assert_eq!(meta.result_count, Some(5));
        assert_eq!(meta.additional_fields.len(), 2);
        assert!(meta.additional_fields.contains_key("future_field"));
        assert!(meta.additional_fields.contains_key("experimental_count"));

        // Verify unknown fields are preserved in roundtrip
        let serialized = serde_json::to_string(&meta).unwrap();
        let roundtrip: ResponseMeta = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtrip.additional_fields.len(), 2);
    }

    #[test]
    fn test_api_error_roundtrip() {
        let json = r#"{
            "message": "Rate limit exceeded",
            "code": "RATE_LIMIT_EXCEEDED",
            "parameter": "max_results"
        }"#;

        let error: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "Rate limit exceeded");
        assert_eq!(error.code, Some("RATE_LIMIT_EXCEEDED".to_string()));

        let serialized = serde_json::to_string(&error).unwrap();
        let roundtrip: ApiError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(error.message, roundtrip.message);
    }

    #[test]
    fn test_place_unknown_fields_captured() {
        let json = r#"{
            "id": "place123",
            "full_name": "San Francisco, CA",
            "future_timezone": "America/Los_Angeles",
            "new_metadata": {"key": "value"}
        }"#;

        let place: Place = serde_json::from_str(json).unwrap();
        assert_eq!(place.id, "place123");
        assert_eq!(place.additional_fields.len(), 2);
        assert!(place.additional_fields.contains_key("future_timezone"));
        assert!(place.additional_fields.contains_key("new_metadata"));
    }

    #[test]
    fn test_poll_roundtrip() {
        let json = r#"{
            "id": "poll123",
            "options": [
                {"position": 1, "label": "Yes", "votes": 10},
                {"position": 2, "label": "No", "votes": 5}
            ],
            "duration_minutes": 1440,
            "voting_status": "closed"
        }"#;

        let poll: Poll = serde_json::from_str(json).unwrap();
        assert_eq!(poll.id, "poll123");
        assert_eq!(poll.options.len(), 2);
        assert_eq!(poll.options[0].votes, 10);

        let serialized = serde_json::to_string(&poll).unwrap();
        let roundtrip: Poll = serde_json::from_str(&serialized).unwrap();
        assert_eq!(poll.id, roundtrip.id);
        assert_eq!(poll.options.len(), roundtrip.options.len());
    }

    #[test]
    fn test_reply_settings_serialization() {
        let settings = ReplySettings::Everyone;
        let json = serde_json::to_string(&settings).unwrap();
        assert_eq!(json, r#""everyone""#);

        let settings = ReplySettings::MentionedUsers;
        let json = serde_json::to_string(&settings).unwrap();
        assert_eq!(json, r#""mentioned_users""#);
    }

    #[test]
    fn test_reply_settings_deserialization() {
        let json = r#""everyone""#;
        let settings: ReplySettings = serde_json::from_str(json).unwrap();
        assert!(matches!(settings, ReplySettings::Everyone));

        let json = r#""following""#;
        let settings: ReplySettings = serde_json::from_str(json).unwrap();
        assert!(matches!(settings, ReplySettings::Following));
    }

    #[test]
    fn test_withheld_default_fields() {
        let json = r#"{}"#;
        let withheld: Withheld = serde_json::from_str(json).unwrap();

        assert!(withheld.copyright.is_none());
        assert!(withheld.country_codes.is_none());
        assert!(withheld.scope.is_none());
        assert_eq!(withheld.additional_fields.len(), 0);
    }

    #[test]
    fn test_includes_forward_compatibility() {
        let json = r#"{
            "users": [{"id": "123", "name": "Test", "username": "test"}],
            "future_includes": ["item1", "item2"]
        }"#;

        let includes: Includes = serde_json::from_str(json).unwrap();
        assert!(includes.users.is_some());
        assert_eq!(includes.additional_fields.len(), 1);
        assert!(includes.additional_fields.contains_key("future_includes"));
    }
}
