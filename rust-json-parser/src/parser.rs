//! JSON parser module for parsing JSON values.

use crate::error::JsonError;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::JsonValue;

/// Convenience function that tokenizes and parses a JSON input string.
pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    JsonParser::new(input)?.parse()
}

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
        let value = self.parse_value()?;
        if !self.is_at_end() {
            let token = self.advance();
            return Err(JsonError::UnexpectedToken {
                expected: "end of input".to_string(),
                found: format!("{:?}", token.unwrap_or(Token::Null)),
                position: self.current - 1,
            });
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.peek() {
            Some(Token::LeftBracket) => self.parse_array(),
            Some(Token::LeftBrace) => self.parse_object(),
            _ => match self.advance() {
                Some(Token::String(s)) => Ok(JsonValue::String(s)),
                Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
                Some(Token::Boolean(b)) => Ok(JsonValue::Boolean(b)),
                Some(Token::Null) => Ok(JsonValue::Null),
                Some(other) => Err(JsonError::UnexpectedToken {
                    expected: "JSON value".to_string(),
                    found: format!("{:?}", other),
                    position: self.current - 1,
                }),
                None => Err(JsonError::UnexpectedEndOfInput {
                    expected: "JSON value".to_string(),
                    position: self.current,
                }),
            },
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // consume opening '['
        let mut elements: Vec<JsonValue> = Vec::new();

        // Empty array case
        if matches!(self.peek(), Some(Token::RightBracket)) {
            self.advance(); // consume closing ']'
            return Ok(JsonValue::Array(elements));
        }

        loop {
            // Parse the next element
            let value = self.parse_value()?;
            elements.push(value);

            // Check what follows the element
            match self.peek() {
                Some(Token::Comma) => {
                    self.advance(); // consume comma
                    // Check for trailing comma
                    if matches!(self.peek(), Some(Token::RightBracket)) {
                        return Err(JsonError::UnexpectedToken {
                            expected: "JSON value".to_string(),
                            found: "]".to_string(),
                            position: self.current,
                        });
                    }
                }
                Some(Token::RightBracket) => {
                    self.advance(); // consume closing ']'
                    break;
                }
                Some(_) => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "comma or closing bracket".to_string(),
                        found: format!("{:?}", self.peek().unwrap()),
                        position: self.current,
                    });
                }
                None => {
                    return Err(JsonError::UnexpectedEndOfInput {
                        expected: "comma or closing bracket".to_string(),
                        position: self.current,
                    });
                }
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        Err(JsonError::UnexpectedToken {
            expected: "JSON value".to_string(),
            found: "{".to_string(),
            position: self.current,
        })
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.is_at_end() {
            None
        } else {
            Some(&self.tokens[self.current])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_json free function ---

    #[test]
    fn test_parse_json_string() {
        let result = parse_json(r#""hello""#).unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_json_number() {
        let result = parse_json("42").unwrap();
        assert_eq!(result, JsonValue::Number(42.0));
    }

    #[test]
    fn test_parse_json_error() {
        let result = parse_json("@");
        assert!(result.is_err());
    }

    // --- Primitive parsing ---

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
        assert_eq!(result, JsonValue::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_parse_string_with_tab() {
        let result = JsonParser::new(r#""col1\tcol2""#).unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::String("col1\tcol2".to_string()));
    }

    #[test]
    fn test_parse_string_with_quotes() {
        let result = JsonParser::new(r#""say \"hi\"""#).unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::String("say \"hi\"".to_string()));
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
        assert_eq!(
            result,
            JsonValue::String("line1\nline2\t\"quoted\"".to_string())
        );
    }

    #[test]
    fn test_parse_negative_number() {
        let result = JsonParser::new("-3.14").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(-3.14));
    }

    #[test]
    fn test_parse_boolean_true() {
        let result = JsonParser::new("true").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(true));
    }

    #[test]
    fn test_parse_boolean_false() {
        let result = JsonParser::new("false").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_simple_string() {
        let result = JsonParser::new(r#""hello""#).unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));
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

    // --- peek() helper method ---

    #[test]
    fn test_peek_returns_reference() {
        let parser = JsonParser::new("42").unwrap();
        let peeked = parser.peek();
        assert_eq!(peeked, Some(&Token::Number(42.0)));
    }

    #[test]
    fn test_peek_does_not_advance() {
        let parser = JsonParser::new("42").unwrap();
        let first = parser.peek();
        let second = parser.peek();
        assert_eq!(first, second);
    }

    #[test]
    fn test_peek_at_end() {
        let mut parser = JsonParser::new("42").unwrap();
        parser.advance();
        assert_eq!(parser.peek(), None);
    }

    // --- Trailing tokens ---

    #[test]
    fn test_parse_rejects_trailing_tokens() {
        let result = JsonParser::new("42 true").unwrap().parse();
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    // --- Array parsing ---

    #[test]
    fn test_parse_empty_array() {
        let result = parse_json("[]").unwrap();
        assert_eq!(result, JsonValue::Array(vec![]));
    }

    #[test]
    fn test_parse_array_single_element() {
        let result = parse_json("[42]").unwrap();
        assert_eq!(result, JsonValue::Array(vec![JsonValue::Number(42.0)]));
    }

    #[test]
    fn test_parse_array_multiple_numbers() {
        let result = parse_json("[1, 2, 3]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_array_mixed_types() {
        let result = parse_json(r#"[1, "two", true, null]"#).unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::String("two".to_string()),
                JsonValue::Boolean(true),
                JsonValue::Null,
            ])
        );
    }

    #[test]
    fn test_parse_array_with_whitespace() {
        let result = parse_json("[ 1 , 2 , 3 ]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_nested_arrays() {
        let result = parse_json("[[1, 2], [3, 4]]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
                JsonValue::Array(vec![JsonValue::Number(3.0), JsonValue::Number(4.0)]),
            ])
        );
    }

    #[test]
    fn test_parse_deeply_nested_arrays() {
        let result = parse_json("[[[1]]]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Array(vec![
                JsonValue::Number(1.0)
            ])])])
        );
    }

    #[test]
    fn test_parse_array_of_strings() {
        let result = parse_json(r#"["a", "b", "c"]"#).unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::String("a".to_string()),
                JsonValue::String("b".to_string()),
                JsonValue::String("c".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_array_of_booleans() {
        let result = parse_json("[true, false, true]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::Boolean(true),
                JsonValue::Boolean(false),
                JsonValue::Boolean(true),
            ])
        );
    }

    #[test]
    fn test_parse_array_with_null() {
        let result = parse_json("[null, null]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![JsonValue::Null, JsonValue::Null])
        );
    }

    #[test]
    fn test_parse_array_single_null() {
        let result = parse_json("[null]").unwrap();
        assert_eq!(result, JsonValue::Array(vec![JsonValue::Null]));
    }

    #[test]
    fn test_parse_array_nested_empty() {
        let result = parse_json("[[], []]").unwrap();
        assert_eq!(
            result,
            JsonValue::Array(vec![JsonValue::Array(vec![]), JsonValue::Array(vec![]),])
        );
    }

    // --- Array error cases ---

    #[test]
    fn test_parse_array_unclosed() {
        let result = parse_json("[1, 2");
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }

    #[test]
    fn test_parse_array_trailing_comma() {
        let result = parse_json("[1, 2,]");
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_array_missing_comma() {
        let result = parse_json("[1 2]");
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_array_leading_comma() {
        let result = parse_json("[, 1]");
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_array_double_comma() {
        let result = parse_json("[1,, 2]");
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_array_unclosed_nested() {
        let result = parse_json("[[1, 2]");
        assert!(result.is_err());
    }

    // --- Array with value helpers ---

    #[test]
    fn test_parse_array_as_array() {
        let result = parse_json("[1, 2, 3]").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], JsonValue::Number(1.0));
    }

    #[test]
    fn test_parse_array_get_index() {
        let result = parse_json(r#"["a", "b", "c"]"#).unwrap();
        assert_eq!(
            result.get_index(1),
            Some(&JsonValue::String("b".to_string()))
        );
        assert_eq!(result.get_index(5), None);
    }

    // --- parse_object stub ---

    #[test]
    fn test_parse_object_stub_returns_error() {
        let result = parse_json(r#"{"key": "value"}"#);
        assert!(result.is_err());
    }
}
