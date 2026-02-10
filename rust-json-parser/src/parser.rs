//! JSON parser module for parsing primitive values.

use crate::error::JsonError;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::JsonValue;

/// Result type alias for convenience.
type Result<T> = std::result::Result<T, JsonError>;

/// Parses a JSON string and returns a JsonValue.
pub fn parse_json(input: &str) -> Result<JsonValue> {
    let tokens = Tokenizer::new(input).tokenize()?;

    if tokens.is_empty() {
        return Err(JsonError::UnexpectedEndOfInput {
            expected: "JSON value".to_string(),
            position: 0,
        });
    }

    match &tokens[0] {
        Token::String(s) => Ok(JsonValue::String(s.clone())),
        Token::Number(n) => Ok(JsonValue::Number(*n)),
        Token::Boolean(b) => Ok(JsonValue::Boolean(*b)),
        Token::Null => Ok(JsonValue::Null),
        other => Err(JsonError::UnexpectedToken {
            expected: "primitive JSON value".to_string(),
            found: format!("{:?}", other),
            position: 0,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let result = parse_json(r#""hello world""#).unwrap();
        assert_eq!(result, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let result = parse_json("42.5").unwrap();
        assert_eq!(result, JsonValue::Number(42.5));

        let result = parse_json("0").unwrap();
        assert_eq!(result, JsonValue::Number(0.0));

        let result = parse_json("-10").unwrap();
        assert_eq!(result, JsonValue::Number(-10.0));
    }

    #[test]
    fn test_parse_boolean() {
        let result = parse_json("true").unwrap();
        assert_eq!(result, JsonValue::Boolean(true));

        let result = parse_json("false").unwrap();
        assert_eq!(result, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_null() {
        let result = parse_json("null").unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_error_empty() {
        let result = parse_json("");
        assert!(result.is_err());

        match result {
            Err(JsonError::UnexpectedEndOfInput { expected, position }) => {
                assert_eq!(expected, "JSON value");
                assert_eq!(position, 0);
            }
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_parse_error_invalid_token() {
        let result = parse_json("@");
        assert!(result.is_err());
        match result {
            Err(JsonError::UnexpectedToken {
                found, position, ..
            }) => {
                assert_eq!(found, "@");
                assert_eq!(position, 0);
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_invalid_characters() {
        for input in ["@", "$", "%", "^"] {
            let result = parse_json(input);
            assert!(result.is_err(), "Should fail for: {}", input);
            assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
        }
    }

    #[test]
    fn test_all_primitives() {
        let test_cases = vec![
            ("null", JsonValue::Null),
            ("true", JsonValue::Boolean(true)),
            ("false", JsonValue::Boolean(false)),
            ("42", JsonValue::Number(42.0)),
            ("-3.14", JsonValue::Number(-3.14)),
            ("0", JsonValue::Number(0.0)),
            (r#""hello""#, JsonValue::String("hello".to_string())),
        ];

        for (input, expected) in test_cases {
            let result = parse_json(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_parse_with_whitespace() {
        let result = parse_json("  42  ").unwrap();
        assert_eq!(result, JsonValue::Number(42.0));

        let result = parse_json("\n\ttrue\n").unwrap();
        assert_eq!(result, JsonValue::Boolean(true));
    }

    #[test]
    fn test_result_pattern_matching() {
        let result = parse_json("42");

        match result {
            Ok(JsonValue::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected successful number parse"),
        }

        let result = parse_json("@invalid@");

        match result {
            Err(JsonError::UnexpectedToken { .. }) => {} // Expected
            _ => panic!("Expected UnexpectedToken error"),
        }
    }
}
