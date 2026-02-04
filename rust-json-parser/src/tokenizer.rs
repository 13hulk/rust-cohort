//! JSON tokenizer module.

use crate::error::JsonError;

/// Represents a single JSON token.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Structural tokens
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :

    // Value tokens
    String(String), // e.g., "hello"
    Number(f64),    // e.g., 42, 3.14, -10
    Boolean(bool),  // true, false
    Null,           // null
}

/// Parses a JSON string and returns a list of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, JsonError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut position = 0;

    while let Some(&ch) = chars.peek() {
        match ch {
            // Structural tokens
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
                position += 1;
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
                position += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
                position += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
                position += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
                position += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
                position += 1;
            }

            // Whitespace: skip
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
                position += 1;
            }

            // String: parse
            '"' => {
                chars.next(); // consume opening quote
                position += 1;
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '"' => {
                            chars.next(); // closing quote: end string
                            position += 1;
                            break;
                        }
                        _ => {
                            s.push(c); // any other char: collect
                            chars.next();
                            position += 1;
                        }
                    }
                }
                tokens.push(Token::String(s));
            }

            // Keywords: parse true, false, null
            't' | 'f' | 'n' => {
                let start_position = position;
                let mut word = String::new();
                // Collect: lowercase letters only
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' => {
                            word.push(c);
                            chars.next();
                            position += 1;
                        }
                        _ => break, // non-letter: stop collecting
                    }
                }
                // Match: keyword to token
                match word.as_str() {
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    "null" => tokens.push(Token::Null),
                    _ => {
                        return Err(JsonError::UnexpectedToken {
                            expected: "valid JSON token".to_string(),
                            found: word,
                            position: start_position,
                        });
                    }
                }
            }

            // Number: parse (starts with digit or minus sign)
            '0'..='9' | '-' => {
                let start_position = position;
                let mut num_str = String::new();
                // Collect: digits, decimal point, minus sign
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' | '.' | '-' => {
                            num_str.push(c);
                            chars.next();
                            position += 1;
                        }
                        _ => break, // non-numeric char: stop collecting
                    }
                }
                // Convert: string to f64
                match num_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => {
                        return Err(JsonError::InvalidNumber {
                            value: num_str,
                            position: start_position,
                        });
                    }
                }
            }

            // Standalone decimal point - not a valid JSON number
            '.' => {
                return Err(JsonError::UnexpectedToken {
                    expected: "valid JSON token".to_string(),
                    found: ch.to_string(),
                    position,
                });
            }

            // Unknown: return error
            _ => {
                return Err(JsonError::UnexpectedToken {
                    expected: "valid JSON token".to_string(),
                    found: ch.to_string(),
                    position,
                });
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, JsonError>;

    #[test]
    fn test_empty_braces() -> Result<()> {
        let tokens = tokenize("{}")?;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
        Ok(())
    }

    #[test]
    fn test_simple_string() -> Result<()> {
        let tokens = tokenize(r#""hello""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let tokens = tokenize("42")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
        Ok(())
    }

    #[test]
    fn test_number_negative() -> Result<()> {
        let tokens = tokenize("-42")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_number_simple_decimal() -> Result<()> {
        let tokens = tokenize("3.14")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(3.14));
        Ok(())
    }

    #[test]
    fn test_number_negative_decimal() -> Result<()> {
        let tokens = tokenize("-0.99")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-0.99));
        Ok(())
    }

    #[test]
    fn test_tokenize_string() -> Result<()> {
        let tokens = tokenize(r#""hello world""#)?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
        Ok(())
    }

    #[test]
    fn test_boolean_and_null() -> Result<()> {
        let tokens = tokenize("true false null")?;
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
        Ok(())
    }

    #[test]
    fn test_simple_object() -> Result<()> {
        let tokens = tokenize(r#"{"name": "Alice"}"#)?;
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("name".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::String("Alice".to_string()));
        assert_eq!(tokens[4], Token::RightBrace);
        Ok(())
    }

    #[test]
    fn test_multiple_values() -> Result<()> {
        let tokens = tokenize(r#"{"age": 30, "active": true}"#)?;

        // Verify we have the right tokens
        assert!(tokens.contains(&Token::String("age".to_string())));
        assert!(tokens.contains(&Token::Number(30.0)));
        assert!(tokens.contains(&Token::Comma));
        assert!(tokens.contains(&Token::String("active".to_string())));
        assert!(tokens.contains(&Token::Boolean(true)));
        Ok(())
    }

    // New tests from Week 2 curriculum

    #[test]
    fn test_empty_string() -> Result<()> {
        // Outer boundary: adjacent quotes with no inner content
        let tokens = tokenize(r#""""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        // Inner handling: JSON delimiters inside strings don't break tokenization
        let tokens = tokenize(r#""{key: value}""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        // Inner handling: "true", "false", "null" inside strings stay as string content
        let tokens = tokenize(r#""not true or false""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        // Inner handling: numeric content inside strings doesn't become Number tokens
        let tokens = tokenize(r#""phone: 555-1234""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
        Ok(())
    }

    #[test]
    fn test_negative_number() -> Result<()> {
        let tokens = tokenize("-42")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_decimal_number() -> Result<()> {
        let tokens = tokenize("0.5")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
        Ok(())
    }

    #[test]
    fn test_leading_decimal_not_a_number() -> Result<()> {
        // .5 is invalid JSON - numbers must have leading digit (0.5 is valid)
        let tokens = tokenize(".5");
        // Should NOT be interpreted as 0.5
        assert!(tokens.is_err() || !tokens.unwrap().contains(&Token::Number(0.5)));
        Ok(())
    }

    #[test]
    fn test_invalid_character_error() {
        let result = tokenize("@");
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
    fn test_invalid_word_error() {
        let result = tokenize("trueblue");
        assert!(result.is_err());
        match result {
            Err(JsonError::UnexpectedToken { found, .. }) => {
                assert_eq!(found, "trueblue");
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }
}
