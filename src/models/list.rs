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
