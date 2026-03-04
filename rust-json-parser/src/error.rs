//! Error types for JSON parsing with position information.
//!
//! This module defines [`JsonError`](crate::error::JsonError), the error enum
//! returned by the tokenizer and parser when they encounter invalid JSON input.
//! Every variant carries a `position` field indicating the byte offset in the
//! input where the error was detected, making it straightforward to report
//! precise error locations to the caller.

use std::error;
use std::fmt;

/// Represents errors that can occur during JSON tokenization and parsing.
///
/// `JsonError` is an enum with five variants, each describing a different
/// category of failure. All variants carry a `position` field that records
/// the byte offset in the input string where the error was detected, which
/// is included in the human-readable message produced by the `Display`
/// implementation.
///
/// This type implements both `Display` and `std::error::Error`, so it can
/// be used with the `?` operator and integrated into broader error-handling
/// chains.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    /// The parser encountered a token it did not expect at the current position.
    ///
    /// This error occurs when the token stream contains a valid token that
    /// appears in an invalid context. For example, finding a comma where a
    /// value is expected, or encountering an unrecognized character like `@`
    /// at the start of a value.
    UnexpectedToken {
        /// A description of what the parser expected to find at this position
        /// (e.g., `"value"`, `"closing bracket"`).
        expected: String,
        /// The token or character that was actually found (e.g., `","`, `"@"`).
        found: String,
        /// Byte position in the input where the unexpected token was encountered.
        position: usize,
    },
    /// The JSON input ended before the parser finished reading a complete value.
    ///
    /// This error occurs when the input is truncated or incomplete. Common
    /// causes include an unclosed string literal, an array or object missing
    /// its closing bracket or brace, or an input that ends right after a
    /// colon in an object.
    UnexpectedEndOfInput {
        /// A description of what the parser expected to find before the input
        /// ended (e.g., `"closing quote"`, `"']'"`).
        expected: String,
        /// Byte position in the input where the end of input was reached.
        position: usize,
    },
    /// A numeric literal in the JSON could not be parsed as a valid number.
    ///
    /// This error occurs when the tokenizer finds a sequence of characters
    /// that looks like a number but fails to parse as a valid `f64`. Examples
    /// include numbers with multiple decimal points (`12.34.56`) or other
    /// malformed numeric formats.
    InvalidNumber {
        /// The raw string that could not be parsed as a valid number
        /// (e.g., `"12.34.56"`).
        value: String,
        /// Byte position in the input where the invalid number starts.
        position: usize,
    },
    /// An invalid escape sequence was found inside a JSON string.
    ///
    /// JSON permits eight basic escape sequences (`\"`, `\\`, `\/`, `\b`,
    /// `\f`, `\n`, `\r`, `\t`) and the unicode escape `\uXXXX`. Any other
    /// character following a backslash (e.g., `\q` or `\a`) triggers this
    /// error.
    InvalidEscape {
        /// The character that appeared after the backslash (e.g., `'q'` for
        /// the invalid sequence `\q`).
        char: char,
        /// Byte position in the input where the backslash of the invalid
        /// escape sequence is located.
        position: usize,
    },
    /// A `\uXXXX` unicode escape sequence contains invalid hexadecimal digits.
    ///
    /// This error occurs when the four characters following `\u` are not all
    /// valid hexadecimal digits (`0`-`9`, `a`-`f`, `A`-`F`). For example,
    /// `\uZZZZ` or `\uGHIJ` would trigger this error.
    InvalidUnicode {
        /// The four-character sequence that was found after `\u` (e.g.,
        /// `"ZZZZ"`). May be fewer than four characters if the input ended
        /// prematurely.
        sequence: String,
        /// Byte position in the input where the `\u` escape begins.
        position: usize,
    },
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Unexpected token at position {}: expected {}, found {}",
                    position, expected, found
                )
            }
            JsonError::UnexpectedEndOfInput { expected, position } => {
                write!(
                    f,
                    "Unexpected end of input at position {}: expected {}",
                    position, expected
                )
            }
            JsonError::InvalidNumber { value, position } => {
                write!(f, "Invalid number at position {}: {}", position, value)
            }
            JsonError::InvalidEscape { char, position } => {
                write!(
                    f,
                    "Invalid escape sequence '\\{}' at position {}",
                    char, position
                )
            }
            JsonError::InvalidUnicode { sequence, position } => {
                write!(
                    f,
                    "Invalid unicode escape '\\u{}' at position {}",
                    sequence, position
                )
            }
        }
    }
}

impl error::Error for JsonError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "@".to_string(),
            position: 5,
        };

        // Error should be Debug-printable
        assert!(format!("{:?}", error).contains("UnexpectedToken"));
    }

    #[test]
    fn test_error_display() {
        let error = JsonError::UnexpectedToken {
            expected: "valid JSON".to_string(),
            found: "@".to_string(),
            position: 0,
        };

        let message = format!("{}", error);
        assert!(message.contains("position 0"));
        assert!(message.contains("valid JSON"));
        assert!(message.contains("@"));
    }

    #[test]
    fn test_error_variants() {
        let token_error = JsonError::UnexpectedToken {
            expected: "number".to_string(),
            found: "x".to_string(),
            position: 3,
        };

        let eof_error = JsonError::UnexpectedEndOfInput {
            expected: "closing quote".to_string(),
            position: 10,
        };

        let num_error = JsonError::InvalidNumber {
            value: "12.34.56".to_string(),
            position: 0,
        };

        // All variants should be Debug-printable
        let _ = format!("{:?}", token_error);
        let _ = format!("{:?}", eof_error);
        let _ = format!("{:?}", num_error);
    }

    #[test]
    fn test_invalid_escape_display() {
        let error = JsonError::InvalidEscape {
            char: 'x',
            position: 7,
        };

        let message = format!("{}", error);
        assert!(message.contains("Invalid escape sequence"));
        assert!(message.contains("\\x"));
        assert!(message.contains("position 7"));
    }

    #[test]
    fn test_invalid_unicode_display() {
        let error = JsonError::InvalidUnicode {
            sequence: "ZZZZ".to_string(),
            position: 3,
        };

        let message = format!("{}", error);
        assert!(message.contains("Invalid unicode escape"));
        assert!(message.contains("\\uZZZZ"));
        assert!(message.contains("position 3"));
    }

    #[test]
    fn test_error_is_std_error() {
        use std::error::Error;

        let error = JsonError::UnexpectedToken {
            expected: "value".to_string(),
            found: "@".to_string(),
            position: 0,
        };
        assert!(error.source().is_none());

        let error = JsonError::UnexpectedEndOfInput {
            expected: "closing quote".to_string(),
            position: 5,
        };
        assert!(error.source().is_none());

        let error = JsonError::InvalidNumber {
            value: "1.2.3".to_string(),
            position: 0,
        };
        assert!(error.source().is_none());

        let error = JsonError::InvalidEscape {
            char: 'q',
            position: 2,
        };
        assert!(error.source().is_none());

        let error = JsonError::InvalidUnicode {
            sequence: "GHIJ".to_string(),
            position: 4,
        };
        assert!(error.source().is_none());
    }
}
