//! List objects
//!
//! # Visibility Strategy
//!
//! Struct fields are intentionally `pub` to support Serde serialization/deserialization.
//! See module documentation in `models/common.rs` for rationale.

use crate::models::common::{ListId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A List object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// Unique identifier of this List
    pub id: ListId,

    /// The name of the List
    pub name: String,

    // Optional fields
    /// Creation time of this List
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    /// Description of the List
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Number of followers this List has
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<u32>,

    /// Number of members in this List
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,

    /// User ID of the List owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,

    /// Whether this List is private
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_roundtrip() {
        let json = r#"{
            "id": "list123",
            "name": "Rust Developers",
            "description": "Cool Rust people",
            "member_count": 100,
            "private": false
        }"#;

        let list: List = serde_json::from_str(json).unwrap();
        assert_eq!(list.id, "list123");
        assert_eq!(list.name, "Rust Developers");
        assert_eq!(list.member_count, Some(100));

        let serialized = serde_json::to_string(&list).unwrap();
        let roundtrip: List = serde_json::from_str(&serialized).unwrap();
        assert_eq!(list.id, roundtrip.id);
    }

    #[test]
    fn test_list_unknown_fields_captured() {
        let json = r#"{
            "id": "123",
            "name": "Test List",
            "future_category": "technology",
            "ai_curated": true
        }"#;

        let list: List = serde_json::from_str(json).unwrap();
        assert_eq!(list.additional_fields.len(), 2);
        assert!(list.additional_fields.contains_key("future_category"));
    }
}
