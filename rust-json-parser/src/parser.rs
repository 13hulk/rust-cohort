//! JSON parser module for parsing primitive values.

use crate::error::JsonError;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::JsonValue;

/// Holds tokens and current position for parsing.
pub struct JsonParser {
    tokens: Vec<Token>,
    current: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Result<Self, JsonError> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        Ok(Self { tokens, current: 0 })
    }

    pub fn parse(&mut self) -> Result<JsonValue, JsonError> {
        if self.tokens.is_empty() {
            return Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: 0,
            });
        }

        match self.advance() {
            Some(Token::String(s)) => Ok(JsonValue::String(s)),
            Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
            Some(Token::Boolean(b)) => Ok(JsonValue::Boolean(b)),
            Some(Token::Null) => Ok(JsonValue::Null),
            Some(other) => Err(JsonError::UnexpectedToken {
                expected: "primitive JSON value".to_string(),
                found: format!("{:?}", other),
                position: 0,
            }),
            None => Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: 0,
            }),
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let result = JsonParser::new(r#""hello world""#)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(result, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let result = JsonParser::new("42.5").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(42.5));

        let result = JsonParser::new("0").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(0.0));

        let result = JsonParser::new("-10").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(-10.0));
    }

    #[test]
    fn test_parse_boolean() {
        let result = JsonParser::new("true").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(true));

        let result = JsonParser::new("false").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_null() {
        let result = JsonParser::new("null").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_error_empty() {
        let mut parser = JsonParser::new("").unwrap();
        let result = parser.parse();
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
        let result = JsonParser::new("@");
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
            let result = JsonParser::new(input);
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
            let result = JsonParser::new(input).unwrap().parse().unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_parse_with_whitespace() {
        let result = JsonParser::new("  42  ").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(42.0));

        let result = JsonParser::new("\n\ttrue\n").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(true));
    }

    #[test]
    fn test_result_pattern_matching() {
        let result = JsonParser::new("42").unwrap().parse();

        match result {
            Ok(JsonValue::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected successful number parse"),
        }

        let result = JsonParser::new("@invalid@");

        match result {
            Err(JsonError::UnexpectedToken { .. }) => {} // Expected
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    // New tests

    #[test]
    fn test_parser_creation() {
        let parser = JsonParser::new("42");
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parser_creation_tokenize_error() {
        let result = JsonParser::new("@");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_string_with_newline() {
        let result = JsonParser::new(r#""hello\nworld""#)
            .unwrap()
            .parse()
            .unwrap();
        if let JsonValue::String(s) = result {
            assert!(s.contains('\n'));
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_parse_string_with_tab() {
        let result = JsonParser::new(r#""col1\tcol2""#).unwrap().parse().unwrap();
        if let JsonValue::String(s) = result {
            assert!(s.contains('\t'));
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_parse_string_with_quotes() {
        let result = JsonParser::new(r#""say \"hi\"""#).unwrap().parse().unwrap();
        if let JsonValue::String(s) = result {
            assert!(s.contains('"'));
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_parse_string_with_unicode() {
        let result = JsonParser::new(r#""\u0048ello""#).unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::String("Hello".to_string()));
    }

    #[test]
    fn test_parse_complex_escapes() {
        let result = JsonParser::new(r#""line1\nline2\t\"quoted\"""#)
            .unwrap()
            .parse()
            .unwrap();
        if let JsonValue::String(s) = result {
            assert!(s.contains('\n'));
            assert!(s.contains('\t'));
            assert!(s.contains('"'));
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_parse_empty_input() {
        let mut parser = JsonParser::new("").unwrap();
        let result = parser.parse();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }

    #[test]
    fn test_parse_whitespace_only() {
        let mut parser = JsonParser::new("   ").unwrap();
        let result = parser.parse();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }
}
