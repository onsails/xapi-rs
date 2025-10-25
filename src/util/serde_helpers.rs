//! Custom Serde deserializers for X API v2 quirks
//!
//! The X API v2 has several inconsistencies that require custom deserialization logic:
//! - IDs that may be strings or numbers
//! - Booleans that may be strings or actual booleans
//! - Empty strings vs null vs missing fields
//! - Arrays that may be null or missing

use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Deserialize an ID field that may be a string or number
///
/// The X API sometimes returns numeric IDs as numbers instead of strings,
/// especially for older data or certain endpoints. This deserializer handles both.
///
/// # Examples
/// ```
/// # use serde::{Deserialize};
/// #[derive(Deserialize)]
/// struct Example {
///     #[serde(deserialize_with = "x_api_client::util::serde_helpers::deserialize_flexible_id")]
///     id: String,
/// }
/// ```
pub fn deserialize_flexible_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        other => Err(serde::de::Error::custom(format!(
            "expected string or number for ID, got {:?}",
            other
        ))),
    }
}

/// Deserialize an optional ID field that may be a string, number, or null
pub fn deserialize_optional_flexible_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        Some(Value::String(s)) => Ok(Some(s)),
        Some(Value::Number(n)) => Ok(Some(n.to_string())),
        Some(Value::Null) | None => Ok(None),
        other => Err(serde::de::Error::custom(format!(
            "expected string, number, or null for ID, got {:?}",
            other
        ))),
    }
}

/// Deserialize a boolean that may be a string ("true"/"false") or actual boolean
pub fn deserialize_flexible_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Bool(b) => Ok(b),
        Value::String(s) => match s.as_str() {
            "true" | "True" | "TRUE" | "1" => Ok(true),
            "false" | "False" | "FALSE" | "0" | "" => Ok(false),
            _ => Err(serde::de::Error::custom(format!(
                "invalid boolean string: {}",
                s
            ))),
        },
        Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        other => Err(serde::de::Error::custom(format!(
            "expected boolean or string, got {:?}",
            other
        ))),
    }
}

/// Deserialize an optional boolean that may be a string, bool, null, or missing
pub fn deserialize_optional_flexible_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        Some(Value::Bool(b)) => Ok(Some(b)),
        Some(Value::String(s)) => match s.as_str() {
            "true" | "True" | "TRUE" | "1" => Ok(Some(true)),
            "false" | "False" | "FALSE" | "0" | "" => Ok(Some(false)),
            _ => Err(serde::de::Error::custom(format!(
                "invalid boolean string: {}",
                s
            ))),
        },
        Some(Value::Number(n)) => Ok(Some(n.as_i64().unwrap_or(0) != 0)),
        Some(Value::Null) | None => Ok(None),
        other => Err(serde::de::Error::custom(format!(
            "expected boolean, string, or null, got {:?}",
            other
        ))),
    }
}

/// Deserialize an optional string, treating empty strings as None
///
/// The X API sometimes returns empty strings where null would be more appropriate.
/// This deserializer normalizes both to None.
pub fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) if s.trim().is_empty() => Ok(None),
        other => Ok(other),
    }
}

/// Deserialize a number that may be a string or actual number
pub fn deserialize_flexible_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("number out of u64 range")),
        Value::String(s) => s
            .parse::<u64>()
            .map_err(|e| serde::de::Error::custom(format!("failed to parse number: {}", e))),
        other => Err(serde::de::Error::custom(format!(
            "expected number or string, got {:?}",
            other
        ))),
    }
}

/// Deserialize an optional number that may be a string, number, null, or missing
pub fn deserialize_optional_flexible_number<'de, D>(
    deserializer: D,
) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<Value>::deserialize(deserializer)? {
        Some(Value::Number(n)) => Ok(n.as_u64()),
        Some(Value::String(s)) if !s.trim().is_empty() => {
            Ok(Some(s.parse::<u64>().map_err(|e| {
                serde::de::Error::custom(format!("failed to parse number: {}", e))
            })?))
        }
        Some(Value::String(_)) | Some(Value::Null) | None => Ok(None),
        other => Err(serde::de::Error::custom(format!(
            "expected number, string, or null, got {:?}",
            other
        ))),
    }
}

/// Deserialize a vector, treating null and missing as empty vector
pub fn deserialize_null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match Option::<Vec<T>>::deserialize(deserializer)? {
        Some(vec) => Ok(vec),
        None => Ok(Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_flexible_id_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_id")]
            id: String,
        }

        let json = r#"{"id": "1234567890"}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.id, "1234567890");
    }

    #[test]
    fn test_flexible_id_number() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_id")]
            id: String,
        }

        let json = r#"{"id": 1234567890}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.id, "1234567890");
    }

    #[test]
    fn test_flexible_bool_from_bool() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_bool")]
            flag: bool,
        }

        let json = r#"{"flag": true}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert!(result.flag);
    }

    #[test]
    fn test_flexible_bool_from_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_bool")]
            flag: bool,
        }

        let json = r#"{"flag": "true"}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert!(result.flag);

        let json = r#"{"flag": "false"}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert!(!result.flag);
    }

    #[test]
    fn test_optional_string_empty() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_optional_string")]
            value: Option<String>,
        }

        let json = r#"{"value": ""}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert!(result.value.is_none());

        let json = r#"{"value": "   "}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert!(result.value.is_none());

        let json = r#"{"value": "actual value"}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, Some("actual value".to_string()));
    }

    #[test]
    fn test_flexible_number_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_number")]
            count: u64,
        }

        let json = r#"{"count": "42"}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.count, 42);
    }

    #[test]
    fn test_flexible_number_actual() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_flexible_number")]
            count: u64,
        }

        let json = r#"{"count": 42}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.count, 42);
    }

    #[test]
    fn test_null_as_empty_vec() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_null_as_empty_vec")]
            items: Vec<String>,
        }

        let json = r#"{"items": null}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.items.len(), 0);

        let json = r#"{"items": ["a", "b"]}"#;
        let result: Test = serde_json::from_str(json).unwrap();
        assert_eq!(result.items.len(), 2);
    }
}
