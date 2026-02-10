//! Custom error types for JSON parsing.

use std::fmt;

/// Represents errors that can occur during JSON parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: String,
        position: usize,
    },
    InvalidNumber {
        value: String,
        position: usize,
    },
    InvalidEscape {
        char: char,
        position: usize,
    },
    InvalidUnicode {
        sequence: String,
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

impl std::error::Error for JsonError {}

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
        let errors: Vec<Box<dyn std::error::Error>> = vec![
            Box::new(JsonError::UnexpectedToken {
                expected: "value".to_string(),
                found: "@".to_string(),
                position: 0,
            }),
            Box::new(JsonError::UnexpectedEndOfInput {
                expected: "closing quote".to_string(),
                position: 5,
            }),
            Box::new(JsonError::InvalidNumber {
                value: "1.2.3".to_string(),
                position: 0,
            }),
            Box::new(JsonError::InvalidEscape {
                char: 'q',
                position: 2,
            }),
            Box::new(JsonError::InvalidUnicode {
                sequence: "GHIJ".to_string(),
                position: 4,
            }),
        ];

        for error in &errors {
            // All variants implement std::error::Error, so .source() is callable
            assert!(error.source().is_none());
        }
    }
}
