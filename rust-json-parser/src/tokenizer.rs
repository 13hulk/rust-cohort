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

/// Holds the input and current position for tokenization.
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, JsonError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                // Structural tokens
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }

                // Whitespace: skip
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                }

                // String: parse
                '"' => {
                    let string_start = self.position;
                    self.advance(); // consume opening quote
                    let mut s = String::new();
                    loop {
                        match self.peek() {
                            Some('"') => {
                                self.advance(); // closing quote
                                break;
                            }
                            Some('\\') => {
                                self.advance(); // consume backslash
                                match self.peek() {
                                    Some('"') => {
                                        s.push('"');
                                        self.advance();
                                    }
                                    Some('\\') => {
                                        s.push('\\');
                                        self.advance();
                                    }
                                    Some('/') => {
                                        s.push('/');
                                        self.advance();
                                    }
                                    Some('b') => {
                                        s.push('\u{0008}');
                                        self.advance();
                                    }
                                    Some('f') => {
                                        s.push('\u{000C}');
                                        self.advance();
                                    }
                                    Some('n') => {
                                        s.push('\n');
                                        self.advance();
                                    }
                                    Some('r') => {
                                        s.push('\r');
                                        self.advance();
                                    }
                                    Some('t') => {
                                        s.push('\t');
                                        self.advance();
                                    }
                                    Some('u') => {
                                        self.advance(); // consume 'u'
                                        let hex_start = self.position;
                                        let mut hex_str = String::new();
                                        for _ in 0..4 {
                                            match self.peek() {
                                                Some(h) => {
                                                    hex_str.push(h);
                                                    self.advance();
                                                }
                                                None => {
                                                    return Err(JsonError::InvalidUnicode {
                                                        sequence: hex_str,
                                                        position: hex_start,
                                                    });
                                                }
                                            }
                                        }
                                        match u32::from_str_radix(&hex_str, 16) {
                                            Ok(code_point) => match char::from_u32(code_point) {
                                                Some(unicode_char) => s.push(unicode_char),
                                                None => {
                                                    return Err(JsonError::InvalidUnicode {
                                                        sequence: hex_str,
                                                        position: hex_start,
                                                    });
                                                }
                                            },
                                            Err(_) => {
                                                return Err(JsonError::InvalidUnicode {
                                                    sequence: hex_str,
                                                    position: hex_start,
                                                });
                                            }
                                        }
                                    }
                                    Some(ch) => {
                                        return Err(JsonError::InvalidEscape {
                                            char: ch,
                                            position: self.position,
                                        });
                                    }
                                    None => {
                                        return Err(JsonError::UnexpectedEndOfInput {
                                            expected: "escape character".to_string(),
                                            position: self.position,
                                        });
                                    }
                                }
                            }
                            Some(c) => {
                                s.push(c);
                                self.advance();
                            }
                            None => {
                                return Err(JsonError::UnexpectedEndOfInput {
                                    expected: "closing quote".to_string(),
                                    position: string_start,
                                });
                            }
                        }
                    }
                    tokens.push(Token::String(s));
                }

                // Keywords: parse true, false, null
                't' | 'f' | 'n' => {
                    let start_position = self.position;
                    let mut word = String::new();
                    // Collect: lowercase letters only
                    while let Some(c) = self.peek() {
                        match c {
                            'a'..='z' => {
                                word.push(c);
                                self.advance();
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

                // Number: parse (starts with digit, minus sign, or decimal point)
                '0'..='9' | '-' | '.' => {
                    let start_position = self.position;
                    let mut num_str = String::new();
                    // Collect: digits, decimal point, minus sign
                    while let Some(c) = self.peek() {
                        match c {
                            '0'..='9' | '.' | '-' => {
                                num_str.push(c);
                                self.advance();
                            }
                            _ => break, // non-numeric char: stop collecting
                        }
                    }
                    // Check for invalid number formats:
                    // - ".5" (leading decimal without digit)
                    // - "-.5" (minus followed by decimal without digit)
                    if num_str.starts_with('.') || num_str.starts_with("-.") {
                        return Err(JsonError::UnexpectedToken {
                            expected: "valid JSON token".to_string(),
                            found: num_str,
                            position: start_position,
                        });
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

                // Unknown: return error
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "valid JSON token".to_string(),
                        found: ch.to_string(),
                        position: self.position,
                    });
                }
            }
        }

        Ok(tokens)
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, JsonError>;

    #[test]
    fn test_empty_braces() -> Result<()> {
        let tokens = Tokenizer::new("{}").tokenize()?;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
        Ok(())
    }

    #[test]
    fn test_simple_string() -> Result<()> {
        let tokens = Tokenizer::new(r#""hello""#).tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let tokens = Tokenizer::new("42").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
        Ok(())
    }

    #[test]
    fn test_number_negative() -> Result<()> {
        let tokens = Tokenizer::new("-42").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_number_simple_decimal() -> Result<()> {
        let tokens = Tokenizer::new("3.14").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(3.14));
        Ok(())
    }

    #[test]
    fn test_number_negative_decimal() -> Result<()> {
        let tokens = Tokenizer::new("-0.99").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-0.99));
        Ok(())
    }

    #[test]
    fn test_tokenize_string() -> Result<()> {
        let tokens = Tokenizer::new(r#""hello world""#).tokenize()?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
        Ok(())
    }

    #[test]
    fn test_boolean_and_null() -> Result<()> {
        let tokens = Tokenizer::new("true false null").tokenize()?;
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
        Ok(())
    }

    #[test]
    fn test_simple_object() -> Result<()> {
        let tokens = Tokenizer::new(r#"{"name": "Alice"}"#).tokenize()?;
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
        let tokens = Tokenizer::new(r#"{"age": 30, "active": true}"#).tokenize()?;

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
        let tokens = Tokenizer::new(r#""""#).tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        // Inner handling: JSON delimiters inside strings don't break tokenization
        let tokens = Tokenizer::new(r#""{key: value}""#).tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        // Inner handling: "true", "false", "null" inside strings stay as string content
        let tokens = Tokenizer::new(r#""not true or false""#).tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        // Inner handling: numeric content inside strings doesn't become Number tokens
        let tokens = Tokenizer::new(r#""phone: 555-1234""#).tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
        Ok(())
    }

    #[test]
    fn test_negative_number() -> Result<()> {
        let tokens = Tokenizer::new("-42").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_decimal_number() -> Result<()> {
        let tokens = Tokenizer::new("0.5").tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
        Ok(())
    }

    #[test]
    fn test_leading_decimal_not_a_number() -> Result<()> {
        // .5 is invalid JSON - numbers must have leading digit (0.5 is valid)
        let result = Tokenizer::new(".5").tokenize();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_minus_leading_decimal_not_a_number() -> Result<()> {
        // -.5 is invalid JSON - must be -0.5
        let result = Tokenizer::new("-.5").tokenize();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_invalid_character_error() {
        let result = Tokenizer::new("@").tokenize();
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
        let result = Tokenizer::new("trueblue").tokenize();
        assert!(result.is_err());
        match result {
            Err(JsonError::UnexpectedToken { found, .. }) => {
                assert_eq!(found, "trueblue");
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_tokenizer_struct_creation() {
        let mut tokenizer = Tokenizer::new("42");
        let result = tokenizer.tokenize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_tokenizer_multiple_tokens() {
        let mut tokenizer = Tokenizer::new(r#"{"key": 42}"#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("key".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::Number(42.0));
        assert_eq!(tokens[4], Token::RightBrace);
    }

    #[test]
    fn test_escape_newline() {
        let tokens = Tokenizer::new(r#""hello\nworld""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_escape_tab() {
        let tokens = Tokenizer::new(r#""hello\tworld""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("hello\tworld".to_string()));
    }

    #[test]
    fn test_escape_quote() {
        let tokens = Tokenizer::new(r#""say \"hi\"""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("say \"hi\"".to_string()));
    }

    #[test]
    fn test_escape_backslash() {
        let tokens = Tokenizer::new(r#""back\\slash""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("back\\slash".to_string()));
    }

    #[test]
    fn test_escape_forward_slash() {
        let tokens = Tokenizer::new(r#""a\/b""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("a/b".to_string()));
    }

    #[test]
    fn test_escape_carriage_return() {
        let tokens = Tokenizer::new(r#""line\rone""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("line\rone".to_string()));
    }

    #[test]
    fn test_escape_backspace_formfeed() {
        let tokens = Tokenizer::new(r#""\b\f""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("\u{0008}\u{000C}".to_string()));
    }

    #[test]
    fn test_multiple_escapes() {
        let tokens = Tokenizer::new(r#""line1\nline2\ttab""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("line1\nline2\ttab".to_string()));
    }

    #[test]
    fn test_unicode_escape_basic() {
        let tokens = Tokenizer::new(r#""\u0041""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("A".to_string()));
    }

    #[test]
    fn test_unicode_escape_multiple() {
        let tokens = Tokenizer::new(r#""\u0048\u0065\u006C\u006C\u006F""#)
            .tokenize()
            .unwrap();
        assert_eq!(tokens[0], Token::String("Hello".to_string()));
    }

    #[test]
    fn test_unicode_escape_mixed() {
        let tokens = Tokenizer::new(r#""Hello \u0057orld""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("Hello World".to_string()));
    }

    #[test]
    fn test_unicode_escape_lowercase() {
        let tokens = Tokenizer::new(r#""\u00e9""#).tokenize().unwrap();
        assert_eq!(tokens[0], Token::String("\u{00e9}".to_string()));
    }

    #[test]
    fn test_invalid_escape_sequence() {
        let result = Tokenizer::new(r#""test\x""#).tokenize();
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::InvalidEscape { .. })));
    }

    #[test]
    fn test_invalid_unicode_too_short() {
        let result = Tokenizer::new(r#""\u00""#).tokenize();
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_invalid_unicode_bad_hex() {
        let result = Tokenizer::new(r#""\uGGGG""#).tokenize();
        assert!(result.is_err());
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_unterminated_string_with_escape() {
        let result = Tokenizer::new(r#""test\n"#).tokenize();
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }
}
