//! Space objects
//!
//! # Visibility Strategy
//!
//! Struct fields are intentionally `pub` to support Serde serialization/deserialization.
//! See module documentation in `models/common.rs` for rationale.

use crate::models::common::{SpaceId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Space object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    /// Unique identifier of this Space
    pub id: SpaceId,

    /// Current state of the Space
    pub state: SpaceState,

    // Optional fields
    /// Creation time of this Space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// User ID of the Space creator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,

    /// Time when the Space ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<Utc>>,

    /// User IDs of the Space hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ids: Option<Vec<UserId>>,

    /// User IDs of invited users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_ids: Option<Vec<UserId>>,

    /// Whether this is a ticketed Space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ticketed: Option<bool>,

    /// Primary language of audio in the Space (BCP47)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Number of participants
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<u32>,

    /// Scheduled start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<DateTime<Utc>>,

    /// User IDs of current speakers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_ids: Option<Vec<UserId>>,

    /// Actual start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// Number of ticket subscribers (ticketed Spaces only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u32>,

    /// Title of the Space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Topic IDs associated with the Space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<Vec<String>>,

    /// Last update time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// State of a Space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum SpaceState {
    Live,
    Scheduled,
    Ended,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_roundtrip() {
        let json = r#"{
            "id": "space123",
            "state": "Live",
            "title": "Tech Talk",
            "participant_count": 42
        }"#;

        let space: Space = serde_json::from_str(json).unwrap();
        assert_eq!(space.id, "space123");
        assert!(matches!(space.state, SpaceState::Live));

        let serialized = serde_json::to_string(&space).unwrap();
        let roundtrip: Space = serde_json::from_str(&serialized).unwrap();
        assert_eq!(space.id, roundtrip.id);
    }

    #[test]
    fn test_space_unknown_fields_captured() {
        let json = r#"{
            "id": "123",
            "state": "Scheduled",
            "future_feature": "recording",
            "ai_moderation": true
        }"#;

        let space: Space = serde_json::from_str(json).unwrap();
        assert_eq!(space.additional_fields.len(), 2);
        assert!(space.additional_fields.contains_key("future_feature"));
    }

    #[test]
    fn test_space_state_serialization() {
        let state = SpaceState::Ended;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, r#""Ended""#);
    }
}
