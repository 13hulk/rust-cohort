//! JSON tokenizer module.
//!
//! First phase of the two-phase parsing pipeline. Scans JSON text byte by byte
//! and produces a flat `Vec<Token>`. The parser in [`crate::parser`] then
//! builds a [`JsonValue`](crate::value::JsonValue) tree from these tokens.
//!
//! Handles all JSON lexical elements: structural characters, string escape
//! sequences (8 basic + `\uXXXX`), numbers, booleans, and null.

use crate::error::JsonError;

/// Represents a single semantic token produced by the JSON tokenizer.
///
/// Tokens fall into three categories:
///
/// - **Structural tokens** -- delimiters and separators that define JSON
///   structure (`LeftBrace`, `RightBrace`, `LeftBracket`, `RightBracket`,
///   `Colon`, `Comma`).
/// - **Literal tokens** -- the JSON keywords `true`, `false`, and `null`
///   (`Boolean`, `Null`).
/// - **Value tokens** -- strings and numbers that carry parsed data
///   (`String`, `Number`).
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// The `{` character, opening a JSON object.
    LeftBrace,
    /// The `}` character, closing a JSON object.
    RightBrace,
    /// The `[` character, opening a JSON array.
    LeftBracket,
    /// The `]` character, closing a JSON array.
    RightBracket,
    /// The `,` character, separating elements in arrays and key-value pairs
    /// in objects.
    Comma,
    /// The `:` character, separating a key from its value inside an object.
    Colon,

    /// A JSON string value with all escape sequences resolved.
    ///
    /// The contained `String` holds the unescaped content (without the
    /// surrounding double quotes). For example, the JSON text `"hello\nworld"`
    /// produces `Token::String("hello\nworld".to_string())`.
    String(String),
    /// A JSON number value parsed as a 64-bit floating point.
    ///
    /// Integers, decimals, and negative numbers are all represented as `f64`.
    /// For example, `42` becomes `Token::Number(42.0)`.
    Number(f64),
    /// A JSON boolean value (`true` or `false`).
    Boolean(bool),
    /// The JSON `null` literal.
    Null,
}

/// Scans JSON text byte by byte and produces a `Vec<Token>`.
///
/// Owns the input as a `String` and uses `.as_bytes()` for scanning.
///
/// # Examples
///
/// ```
/// use rust_json_parser::tokenizer::Tokenizer;
///
/// let tokens = Tokenizer::new("[1, 2, 3]").tokenize()?;
/// assert_eq!(tokens.len(), 7); // [ 1 , 2 , 3 ]
/// # Ok::<(), rust_json_parser::error::JsonError>(())
/// ```
pub struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer from a JSON input string.
    ///
    /// No validation is performed until [`tokenize`](Self::tokenize) is
    /// called.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
        }
    }

    /// Scans the input and produces a vector of [`Token`] values.
    ///
    /// This method consumes the entire input string, skipping whitespace and
    /// converting each JSON lexical element into the corresponding token.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_json_parser::tokenizer::{Token, Tokenizer};
    ///
    /// let tokens = Tokenizer::new(r#"{"key": true}"#).tokenize()?;
    /// assert_eq!(tokens[0], Token::LeftBrace);
    /// assert_eq!(tokens[1], Token::String("key".to_string()));
    /// assert_eq!(tokens[2], Token::Colon);
    /// assert_eq!(tokens[3], Token::Boolean(true));
    /// assert_eq!(tokens[4], Token::RightBrace);
    /// # Ok::<(), rust_json_parser::error::JsonError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`JsonError`] if the input contains
    /// invalid characters, malformed strings, invalid escape sequences,
    /// invalid numbers, or unrecognized keywords.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, JsonError> {
        // TODO: estimate, assumes ~3 bytes per token (1-char delimiters + short strings/numbers)
        let mut tokens = Vec::with_capacity(self.input.len() / 3);
        self.tokenize_into(&mut tokens)?;
        Ok(tokens)
    }

    /// Re-tokenizes new input, reusing the internal string buffer.
    ///
    /// Clears the stored input (keeping its heap allocation) and copies
    /// the new input into it, then tokenizes into the provided token buffer.
    /// On repeated calls with similar-sized inputs, this avoids reallocating
    /// the input string on every parse.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_json_parser::tokenizer::{Token, Tokenizer};
    ///
    /// let mut tokenizer = Tokenizer::new("");
    /// let mut tokens = Vec::new();
    ///
    /// tokenizer.retokenize("[1, 2]", &mut tokens)?;
    /// assert_eq!(tokens.len(), 5);
    ///
    /// tokens.clear();
    /// tokenizer.retokenize("true", &mut tokens)?;
    /// assert_eq!(tokens.len(), 1);
    /// assert_eq!(tokens[0], Token::Boolean(true));
    /// # Ok::<(), rust_json_parser::error::JsonError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`JsonError`] if the input contains
    /// invalid characters, malformed strings, invalid escape sequences,
    /// invalid numbers, or unrecognized keywords.
    pub fn retokenize(&mut self, input: &str, tokens: &mut Vec<Token>) -> Result<(), JsonError> {
        self.input.clear();
        self.input.push_str(input);
        self.position = 0;
        self.tokenize_into(tokens)
    }

    /// Scans the input into the provided token buffer.
    ///
    /// This allows callers to reuse a token buffer across multiple calls,
    /// avoiding repeated heap allocation. The caller is responsible for
    /// clearing the buffer before each call if reuse is desired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_json_parser::tokenizer::{Token, Tokenizer};
    ///
    /// let mut tokens = Vec::new();
    /// Tokenizer::new("[1, 2]").tokenize_into(&mut tokens)?;
    /// assert_eq!(tokens.len(), 5);
    ///
    /// tokens.clear();
    /// Tokenizer::new("true").tokenize_into(&mut tokens)?;
    /// assert_eq!(tokens.len(), 1);
    /// assert_eq!(tokens[0], Token::Boolean(true));
    /// # Ok::<(), rust_json_parser::error::JsonError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`JsonError`] if the input contains
    /// invalid characters, malformed strings, invalid escape sequences,
    /// invalid numbers, or unrecognized keywords.
    pub fn tokenize_into(&mut self, tokens: &mut Vec<Token>) -> Result<(), JsonError> {
        while let Some(b) = self.peek() {
            match b {
                // Structural tokens
                b'{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                b'}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                b'[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                b']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                b':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                b',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }

                // Whitespace: skip
                b' ' | b'\n' | b'\t' | b'\r' => {
                    self.advance();
                }

                // String: parse
                b'"' => {
                    let s = self.parse_string()?;
                    tokens.push(Token::String(s));
                }

                // Keywords: parse true, false, null
                b't' | b'f' | b'n' => {
                    let token = self.parse_keyword()?;
                    tokens.push(token);
                }

                // Number: parse (starts with digit, minus sign, or decimal point)
                b'0'..=b'9' | b'-' | b'.' => {
                    let n = self.parse_number()?;
                    tokens.push(Token::Number(n));
                }

                // Unknown: return error
                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "valid JSON token".to_string(),
                        found: (other as char).to_string(),
                        position: self.position,
                    });
                }
            }
        }

        Ok(())
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        let string_start = self.position;
        self.advance(); // consume opening quote
        // TODO: estimate, most strings are short keys/values so 32 should be enough
        let mut s = String::with_capacity(32);
        let mut seg_start = self.position;
        loop {
            match self.peek() {
                Some(b'"') => {
                    // Flush the current unescaped segment
                    if seg_start < self.position {
                        s.push_str(&self.input[seg_start..self.position]);
                    }
                    self.advance();
                    return Ok(s);
                }
                Some(b'\\') => {
                    // Flush the segment before the backslash
                    if seg_start < self.position {
                        s.push_str(&self.input[seg_start..self.position]);
                    }
                    self.advance(); // consume backslash
                    let ch = self.parse_escape_sequence()?;
                    s.push(ch);
                    seg_start = self.position;
                }
                Some(_) => {
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
    }

    fn parse_escape_sequence(&mut self) -> Result<char, JsonError> {
        match self.peek() {
            Some(b'"') => {
                self.advance();
                Ok('"')
            }
            Some(b'\\') => {
                self.advance();
                Ok('\\')
            }
            Some(b'/') => {
                self.advance();
                Ok('/')
            }
            Some(b'b') => {
                self.advance();
                Ok('\u{0008}')
            }
            Some(b'f') => {
                self.advance();
                Ok('\u{000C}')
            }
            Some(b'n') => {
                self.advance();
                Ok('\n')
            }
            Some(b'r') => {
                self.advance();
                Ok('\r')
            }
            Some(b't') => {
                self.advance();
                Ok('\t')
            }
            Some(b'u') => {
                self.advance();
                self.parse_unicode_escape()
            }
            Some(b) => Err(JsonError::InvalidEscape {
                char: b as char,
                position: self.position,
            }),
            None => Err(JsonError::UnexpectedEndOfInput {
                expected: "escape character".to_string(),
                position: self.position,
            }),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char, JsonError> {
        let hex_start = self.position;
        // \uXXXX = 4 hex digits, check we have enough bytes
        if self.position + 4 > self.input.len() {
            let available = &self.input[self.position..];
            return Err(JsonError::InvalidUnicode {
                sequence: available.to_string(),
                position: hex_start,
            });
        }
        let hex_str = &self.input[self.position..self.position + 4];
        self.position += 4;
        match u32::from_str_radix(hex_str, 16) {
            Ok(code_point) => match char::from_u32(code_point) {
                Some(unicode_char) => Ok(unicode_char),
                None => Err(JsonError::InvalidUnicode {
                    sequence: hex_str.to_string(),
                    position: hex_start,
                }),
            },
            Err(_) => Err(JsonError::InvalidUnicode {
                sequence: hex_str.to_string(),
                position: hex_start,
            }),
        }
    }

    fn parse_keyword(&mut self) -> Result<Token, JsonError> {
        let start = self.position;
        while let Some(b) = self.peek() {
            match b {
                b'a'..=b'z' => {
                    self.advance();
                }
                _ => break,
            }
        }
        let word = &self.input[start..self.position];
        match word {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Err(JsonError::UnexpectedToken {
                expected: "valid JSON token".to_string(),
                found: word.to_string(),
                position: start,
            }),
        }
    }

    fn parse_number(&mut self) -> Result<f64, JsonError> {
        let start = self.position;
        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' | b'.' | b'-' => {
                    self.advance();
                }
                _ => break,
            }
        }
        let num_str = &self.input[start..self.position];
        if num_str.starts_with('.') || num_str.starts_with("-.") {
            return Err(JsonError::UnexpectedToken {
                expected: "valid JSON token".to_string(),
                found: num_str.to_string(),
                position: start,
            });
        }
        match num_str.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err(JsonError::InvalidNumber {
                value: num_str.to_string(),
                position: start,
            }),
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            let b = self.input.as_bytes()[self.position];
            self.position += 1;
            Some(b)
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.as_bytes().get(self.position).copied()
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

    // Direct tests for extracted helper methods

    #[test]
    fn test_parse_string_simple() -> Result<()> {
        let mut t = Tokenizer::new(r#""hello""#);
        let s = t.parse_string()?;
        assert_eq!(s, "hello");
        Ok(())
    }

    #[test]
    fn test_parse_string_empty() -> Result<()> {
        let mut t = Tokenizer::new(r#""""#);
        let s = t.parse_string()?;
        assert_eq!(s, "");
        Ok(())
    }

    #[test]
    fn test_parse_string_with_escapes() -> Result<()> {
        let mut t = Tokenizer::new(r#""line1\nline2""#);
        let s = t.parse_string()?;
        assert_eq!(s, "line1\nline2");
        Ok(())
    }

    #[test]
    fn test_parse_string_unterminated() {
        let mut t = Tokenizer::new(r#""no end"#);
        let result = t.parse_string();
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }

    #[test]
    fn test_parse_escape_sequence_basic() -> Result<()> {
        let mut t = Tokenizer::new("n");
        assert_eq!(t.parse_escape_sequence()?, '\n');
        Ok(())
    }

    #[test]
    fn test_parse_escape_sequence_tab() -> Result<()> {
        let mut t = Tokenizer::new("t");
        assert_eq!(t.parse_escape_sequence()?, '\t');
        Ok(())
    }

    #[test]
    fn test_parse_escape_sequence_quote() -> Result<()> {
        let mut t = Tokenizer::new("\"");
        assert_eq!(t.parse_escape_sequence()?, '"');
        Ok(())
    }

    #[test]
    fn test_parse_escape_sequence_backslash() -> Result<()> {
        let mut t = Tokenizer::new("\\");
        assert_eq!(t.parse_escape_sequence()?, '\\');
        Ok(())
    }

    #[test]
    fn test_parse_escape_sequence_invalid() {
        let mut t = Tokenizer::new("x");
        let result = t.parse_escape_sequence();
        assert!(matches!(result, Err(JsonError::InvalidEscape { .. })));
    }

    #[test]
    fn test_parse_escape_sequence_eof() {
        let mut t = Tokenizer::new("");
        let result = t.parse_escape_sequence();
        assert!(matches!(
            result,
            Err(JsonError::UnexpectedEndOfInput { .. })
        ));
    }

    #[test]
    fn test_parse_unicode_escape_letter() -> Result<()> {
        let mut t = Tokenizer::new("0041");
        assert_eq!(t.parse_unicode_escape()?, 'A');
        Ok(())
    }

    #[test]
    fn test_parse_unicode_escape_accented() -> Result<()> {
        let mut t = Tokenizer::new("00e9");
        assert_eq!(t.parse_unicode_escape()?, '\u{00e9}');
        Ok(())
    }

    #[test]
    fn test_parse_unicode_escape_too_short() {
        let mut t = Tokenizer::new("00");
        let result = t.parse_unicode_escape();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_parse_unicode_escape_bad_hex() {
        let mut t = Tokenizer::new("ZZZZ");
        let result = t.parse_unicode_escape();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_parse_keyword_true() -> Result<()> {
        let mut t = Tokenizer::new("true");
        assert_eq!(t.parse_keyword()?, Token::Boolean(true));
        Ok(())
    }

    #[test]
    fn test_parse_keyword_false() -> Result<()> {
        let mut t = Tokenizer::new("false");
        assert_eq!(t.parse_keyword()?, Token::Boolean(false));
        Ok(())
    }

    #[test]
    fn test_parse_keyword_null() -> Result<()> {
        let mut t = Tokenizer::new("null");
        assert_eq!(t.parse_keyword()?, Token::Null);
        Ok(())
    }

    #[test]
    fn test_parse_keyword_invalid() {
        let mut t = Tokenizer::new("trueblue");
        let result = t.parse_keyword();
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_number_integer() -> Result<()> {
        let mut t = Tokenizer::new("42");
        assert_eq!(t.parse_number()?, 42.0);
        Ok(())
    }

    #[test]
    fn test_parse_number_negative() -> Result<()> {
        let mut t = Tokenizer::new("-7");
        assert_eq!(t.parse_number()?, -7.0);
        Ok(())
    }

    #[test]
    fn test_parse_number_decimal() -> Result<()> {
        let mut t = Tokenizer::new("3.14");
        assert_eq!(t.parse_number()?, 3.14);
        Ok(())
    }

    #[test]
    fn test_parse_number_leading_dot() {
        let mut t = Tokenizer::new(".5");
        let result = t.parse_number();
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn test_parse_number_minus_dot() {
        let mut t = Tokenizer::new("-.5");
        let result = t.parse_number();
        assert!(matches!(result, Err(JsonError::UnexpectedToken { .. })));
    }
}
