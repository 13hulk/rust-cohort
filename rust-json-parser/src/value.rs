//! JSON value types for parsed JSON data.

use std::collections::HashMap;

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
