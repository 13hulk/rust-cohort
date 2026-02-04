//! JSON value types for parsed JSON data.

/// Represents a parsed JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
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
