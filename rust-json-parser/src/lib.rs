//! JSON parser library.

pub mod error;
pub mod parser;
pub mod tokenizer;
pub mod value;

#[cfg(test)]
mod integration_tests {
    use crate::error::JsonError;
    use crate::parser::JsonParser;
    use crate::tokenizer::{Token, Tokenizer};
    use crate::value::JsonValue;

    #[test]
    fn test_parse_string_value() {
        let result = JsonParser::new(r#""hello""#).unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_number_value() {
        let result = JsonParser::new("42.5").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Number(42.5));
    }

    #[test]
    fn test_parse_boolean_value() {
        let result = JsonParser::new("true").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Boolean(true));
    }

    #[test]
    fn test_parse_null_value() {
        let result = JsonParser::new("null").unwrap().parse().unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_error_propagation() {
        let result = JsonParser::new("@invalid");
        assert!(result.is_err());
        match result {
            Err(JsonError::UnexpectedToken { found, .. }) => {
                assert_eq!(found, "@");
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_tokenizer_direct_usage() {
        let tokens = Tokenizer::new(r#"{"key": 123}"#).tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("key".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::Number(123.0));
        assert_eq!(tokens[4], Token::RightBrace);
    }
}
