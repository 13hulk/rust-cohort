//! JSON parser library.

pub mod error;
pub mod parser;
pub mod tokenizer;
pub mod value;

#[cfg(test)]
mod integration_tests {
    use crate::error::JsonError;
    use crate::parser::JsonParser;
    use crate::parser::parse_json;
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

    #[test]
    fn test_parse_json_object_with_mixed_types() {
        let result =
            parse_json(r#"{"name": "Alice", "age": 30, "active": true, "data": null}"#).unwrap();

        assert_eq!(
            result.get("name"),
            Some(&JsonValue::String("Alice".to_string()))
        );
        assert_eq!(result.get("age"), Some(&JsonValue::Number(30.0)));
        assert_eq!(result.get("active"), Some(&JsonValue::Boolean(true)));
        assert_eq!(result.get("data"), Some(&JsonValue::Null));

        let obj = result.as_object().unwrap();
        assert_eq!(obj.len(), 4);
    }

    #[test]
    fn test_parse_nested_structure() {
        let result = parse_json(r#"{"items": [1, 2, 3], "meta": {"count": 3}}"#).unwrap();

        // Verify "items" is an array of length 3
        let items = result.get("items").unwrap();
        let items_arr = items.as_array().unwrap();
        assert_eq!(items_arr.len(), 3);
        assert_eq!(items_arr[0], JsonValue::Number(1.0));
        assert_eq!(items_arr[1], JsonValue::Number(2.0));
        assert_eq!(items_arr[2], JsonValue::Number(3.0));

        // Verify "meta" is an object with "count" = 3
        let meta = result.get("meta").unwrap();
        assert_eq!(meta.get("count"), Some(&JsonValue::Number(3.0)));
    }

    #[test]
    fn test_parse_array_of_objects() {
        let result = parse_json(r#"[{"id": 1}, {"id": 2}]"#).unwrap();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        assert_eq!(arr[0].get("id"), Some(&JsonValue::Number(1.0)));
        assert_eq!(arr[1].get("id"), Some(&JsonValue::Number(2.0)));
    }

    #[test]
    fn test_display_round_trip() {
        // Array: deterministic ordering, use exact match
        let array_result = parse_json(r#"[1, "two", true, null]"#).unwrap();
        let array_output = array_result.to_string();
        assert_eq!(array_output, r#"[1,"two",true,null]"#);

        // Object: HashMap ordering is non-deterministic, use contains
        let object_result = parse_json(r#"{"name": "Alice", "age": 30}"#).unwrap();
        let object_output = object_result.to_string();
        assert!(object_output.contains(r#""name":"Alice""#));
        assert!(object_output.contains(r#""age":30"#));

        // Verify the Display output can be re-parsed
        let reparsed = parse_json(&array_output).unwrap();
        assert_eq!(reparsed, array_result);
    }

    #[test]
    fn test_tokenize_and_parse_object() {
        let input = r#"{"key": [1, 2]}"#;

        // Tokenize and verify token count and types
        let tokens = Tokenizer::new(input).tokenize().unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("key".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::LeftBracket);
        assert_eq!(tokens[4], Token::Number(1.0));
        assert_eq!(tokens[5], Token::Comma);
        assert_eq!(tokens[6], Token::Number(2.0));
        assert_eq!(tokens[7], Token::RightBracket);
        assert_eq!(tokens[8], Token::RightBrace);

        // Parse and verify the result
        let result = parse_json(input).unwrap();
        let items = result.get("key").unwrap();
        let arr = items.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], JsonValue::Number(1.0));
        assert_eq!(arr[1], JsonValue::Number(2.0));
    }
}
