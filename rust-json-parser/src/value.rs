//! JSON value types for parsed JSON data.

use std::collections::HashMap;
use std::fmt;

/// Represents a parsed JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Returns true if this value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Returns the string value if this is a String variant.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the numeric value if this is a Number variant.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the boolean value if this is a Boolean variant.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns a reference to the array if this is an Array variant.
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns a reference to the object if this is an Object variant.
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Returns a reference to the value at the given key if this is an Object variant.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    /// Returns a reference to the value at the given index if this is an Array variant.
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
}

/// Escapes special characters in a string for JSON output.
fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }
    result
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => write!(f, "\"{}\"", escape_string(s)),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in map {
                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;
                    write!(f, "\"{}\":{}", escape_string(key), value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_variants() {
        let null_val = JsonValue::Null;
        let bool_val = JsonValue::Boolean(true);
        let num_val = JsonValue::Number(42.5);
        let str_val = JsonValue::String("hello".to_string());

        assert!(matches!(null_val, JsonValue::Null));
        assert!(matches!(bool_val, JsonValue::Boolean(true)));
        assert!(matches!(num_val, JsonValue::Number(n) if n == 42.5));
        assert!(matches!(str_val, JsonValue::String(ref s) if s == "hello"));
    }

    #[test]
    fn test_json_value_equality() {
        assert_eq!(JsonValue::Null, JsonValue::Null);
        assert_eq!(JsonValue::Boolean(true), JsonValue::Boolean(true));
        assert_eq!(JsonValue::Number(42.0), JsonValue::Number(42.0));
        assert_eq!(
            JsonValue::String("test".to_string()),
            JsonValue::String("test".to_string())
        );

        assert_ne!(JsonValue::Null, JsonValue::Boolean(false));
        assert_ne!(JsonValue::Number(1.0), JsonValue::Number(2.0));
    }

    #[test]
    fn test_is_null() {
        assert!(JsonValue::Null.is_null());
        assert!(!JsonValue::Boolean(true).is_null());
        assert!(!JsonValue::Number(42.0).is_null());
        assert!(!JsonValue::String("test".to_string()).is_null());
    }

    #[test]
    fn test_as_str() {
        let string_val = JsonValue::String("hello".to_string());
        assert_eq!(string_val.as_str(), Some("hello"));

        assert!(JsonValue::Null.as_str().is_none());
        assert!(JsonValue::Boolean(true).as_str().is_none());
        assert!(JsonValue::Number(42.0).as_str().is_none());
    }

    #[test]
    fn test_as_f64() {
        let num_val = JsonValue::Number(3.14);
        assert_eq!(num_val.as_f64(), Some(3.14));

        assert!(JsonValue::Null.as_f64().is_none());
        assert!(JsonValue::Boolean(true).as_f64().is_none());
        assert!(JsonValue::String("test".to_string()).as_f64().is_none());
    }

    #[test]
    fn test_as_bool() {
        assert_eq!(JsonValue::Boolean(true).as_bool(), Some(true));
        assert_eq!(JsonValue::Boolean(false).as_bool(), Some(false));

        assert!(JsonValue::Null.as_bool().is_none());
        assert!(JsonValue::Number(42.0).as_bool().is_none());
        assert!(JsonValue::String("test".to_string()).as_bool().is_none());
    }

    #[test]
    fn test_helper_methods_with_option_combinators() {
        // Demonstrate using helper methods with Option combinators
        let value = JsonValue::String("hello".to_string());

        // Using unwrap_or with as_str
        let text = value.as_str().unwrap_or("default");
        assert_eq!(text, "hello");

        // Using unwrap_or with as_f64 on wrong type
        let num = value.as_f64().unwrap_or(0.0);
        assert_eq!(num, 0.0);

        // Using is_some/is_none
        assert!(value.as_str().is_some());
        assert!(value.as_f64().is_none());
    }

    #[test]
    fn test_as_array() {
        let array_val = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);

        let arr = array_val.as_array();
        assert!(arr.is_some());
        assert_eq!(arr.unwrap().len(), 3);
        assert_eq!(arr.unwrap()[0], JsonValue::Number(1.0));
        assert_eq!(arr.unwrap()[1], JsonValue::Number(2.0));
        assert_eq!(arr.unwrap()[2], JsonValue::Number(3.0));

        // Non-array variants return None
        assert!(JsonValue::Null.as_array().is_none());
        assert!(JsonValue::Boolean(true).as_array().is_none());
        assert!(JsonValue::Number(42.0).as_array().is_none());
        assert!(JsonValue::String("test".to_string()).as_array().is_none());
    }

    #[test]
    fn test_as_object() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), JsonValue::String("Alice".to_string()));
        map.insert("age".to_string(), JsonValue::Number(30.0));
        let object_val = JsonValue::Object(map);

        let obj = object_val.as_object();
        assert!(obj.is_some());
        assert_eq!(obj.unwrap().len(), 2);
        assert_eq!(
            obj.unwrap().get("name"),
            Some(&JsonValue::String("Alice".to_string()))
        );
        assert_eq!(obj.unwrap().get("age"), Some(&JsonValue::Number(30.0)));

        // Non-object variants return None
        assert!(JsonValue::Null.as_object().is_none());
        assert!(JsonValue::Boolean(false).as_object().is_none());
        assert!(JsonValue::Number(1.0).as_object().is_none());
        assert!(JsonValue::String("test".to_string()).as_object().is_none());
    }

    #[test]
    fn test_get() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), JsonValue::String("value1".to_string()));
        map.insert("key2".to_string(), JsonValue::Boolean(true));
        let object_val = JsonValue::Object(map);

        // Existing keys return correct values
        assert_eq!(
            object_val.get("key1"),
            Some(&JsonValue::String("value1".to_string()))
        );
        assert_eq!(object_val.get("key2"), Some(&JsonValue::Boolean(true)));

        // Missing key returns None
        assert_eq!(object_val.get("missing"), None);

        // Non-object variants return None
        assert_eq!(JsonValue::Null.get("key"), None);
        assert_eq!(JsonValue::Number(42.0).get("key"), None);
        assert_eq!(
            JsonValue::Array(vec![JsonValue::Number(1.0)]).get("key"),
            None
        );
    }

    #[test]
    fn test_get_index() {
        let array_val = JsonValue::Array(vec![
            JsonValue::String("first".to_string()),
            JsonValue::Boolean(false),
            JsonValue::Null,
        ]);

        // Valid indices return correct values
        assert_eq!(
            array_val.get_index(0),
            Some(&JsonValue::String("first".to_string()))
        );
        assert_eq!(array_val.get_index(1), Some(&JsonValue::Boolean(false)));
        assert_eq!(array_val.get_index(2), Some(&JsonValue::Null));

        // Out-of-bounds index returns None
        assert_eq!(array_val.get_index(3), None);
        assert_eq!(array_val.get_index(99), None);

        // Non-array variants return None
        assert_eq!(JsonValue::Null.get_index(0), None);
        assert_eq!(JsonValue::Number(42.0).get_index(0), None);

        let mut map = HashMap::new();
        map.insert("key".to_string(), JsonValue::Number(1.0));
        assert_eq!(JsonValue::Object(map).get_index(0), None);
    }

    #[test]
    fn test_option_methods() {
        // Demonstrate Option<T> methods from Week 2 curriculum
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        // .is_some() and .is_none()
        assert!(some_value.is_some());
        assert!(!some_value.is_none());
        assert!(none_value.is_none());
        assert!(!none_value.is_some());

        // .unwrap_or() for default values
        assert_eq!(some_value.unwrap_or(0), 42);
        assert_eq!(none_value.unwrap_or(99), 99);

        // if let pattern matching
        if let Some(x) = some_value {
            assert_eq!(x, 42);
        } else {
            panic!("Expected Some value");
        }

        // match on Option
        let result = match none_value {
            Some(x) => x,
            None => -1,
        };
        assert_eq!(result, -1);
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;
    use crate::parser::parse_json;

    #[test]
    fn test_display_primitives() {
        assert_eq!(JsonValue::Null.to_string(), "null");
        assert_eq!(JsonValue::Boolean(true).to_string(), "true");
        assert_eq!(JsonValue::Boolean(false).to_string(), "false");
        assert_eq!(JsonValue::Number(42.0).to_string(), "42");
        assert_eq!(JsonValue::Number(3.14).to_string(), "3.14");
        assert_eq!(
            JsonValue::String("hello".to_string()).to_string(),
            "\"hello\""
        );
    }

    #[test]
    fn test_display_array() {
        let array = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]);
        assert_eq!(array.to_string(), "[1,2]");
    }

    #[test]
    fn test_display_empty_containers() {
        assert_eq!(JsonValue::Array(vec![]).to_string(), "[]");
        assert_eq!(JsonValue::Object(HashMap::new()).to_string(), "{}");
    }

    #[test]
    fn test_display_escape_string() {
        let value = JsonValue::String("hello\nworld".to_string());
        assert_eq!(value.to_string(), "\"hello\\nworld\"");
    }

    #[test]
    fn test_display_escape_quotes() {
        let value = JsonValue::String("say \"hi\"".to_string());
        assert_eq!(value.to_string(), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_display_nested() {
        let result = parse_json(r#"{"arr": [1, 2]}"#).unwrap();
        let output = result.to_string();
        assert!(output.contains("\"arr\""));
        assert!(output.contains("[1,2]"));
    }
}
