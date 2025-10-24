//! Media objects
//!
//! # Visibility Strategy
//!
//! Struct fields are intentionally `pub` to support Serde serialization/deserialization.
//! See module documentation in `models/common.rs` for rationale.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Media object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    /// Unique identifier for this media object
    pub media_key: String,

    /// Type of media (photo, video, animated_gif)
    #[serde(rename = "type")]
    pub media_type: MediaType,

    // Optional fields
    /// Direct URL to the media file (photos only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Duration in milliseconds (video/gif only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,

    /// Height in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Preview image URL (video/gif only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<String>,

    /// Public engagement metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<MediaMetrics>,

    /// Non-public engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<MediaMetrics>,

    /// Organic engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<MediaMetrics>,

    /// Promoted engagement metrics (requires elevated access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<MediaMetrics>,

    /// Alternative text description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,

    /// Video variants (different bitrates/formats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<MediaVariant>>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Type of media
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MediaType {
    Photo,
    Video,
    AnimatedGif,
}

/// Media engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MediaMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<u64>,

    /// Forward compatibility: capture unknown fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Video variant (different quality/format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaVariant {
    /// Bitrate in bits per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<u64>,

    /// Content type (video/mp4, etc.)
    pub content_type: String,

    /// Direct URL to this variant
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_roundtrip() {
        let json = r#"{
            "media_key": "media123",
            "type": "photo",
            "url": "https://pbs.twimg.com/media/abc.jpg",
            "width": 1200,
            "height": 800
        }"#;

        let media: Media = serde_json::from_str(json).unwrap();
        assert_eq!(media.media_key, "media123");
        assert!(matches!(media.media_type, MediaType::Photo));

        let serialized = serde_json::to_string(&media).unwrap();
        let roundtrip: Media = serde_json::from_str(&serialized).unwrap();
        assert_eq!(media.media_key, roundtrip.media_key);
    }

    #[test]
    fn test_media_unknown_fields_captured() {
        let json = r#"{
            "media_key": "123",
            "type": "video",
            "future_quality": "4k",
            "ai_generated": true
        }"#;

        let media: Media = serde_json::from_str(json).unwrap();
        assert_eq!(media.additional_fields.len(), 2);
        assert!(media.additional_fields.contains_key("future_quality"));
    }

    #[test]
    fn test_media_type_deserialization() {
        let json = r#""animated_gif""#;
        let mtype: MediaType = serde_json::from_str(json).unwrap();
        assert!(matches!(mtype, MediaType::AnimatedGif));
    }
}
