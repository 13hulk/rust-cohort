//! JSON tokenizer module.

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
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Structural tokens
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }

            // Whitespace: skip
            ' ' | '\n' | '\t' => {
                chars.next();
            }

            // String: parse
            '"' => {
                chars.next(); // consume opening quote
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '"' => {
                            chars.next(); // closing quote: end string
                            break;
                        }
                        _ => {
                            s.push(c); // any other char: collect
                            chars.next();
                        }
                    }
                }
                tokens.push(Token::String(s));
            }

            // Keywords: parse true, false, null
            't' | 'f' | 'n' => {
                let mut word = String::new();
                // Collect: lowercase letters only
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' => {
                            word.push(c);
                            chars.next();
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
                        println!("Skipping unknown word: {}", word);
                    }
                }
            }

            // Number: parse
            '0'..='9' | '.' | '-' => {
                let mut num_str = String::new();
                // Collect: digits, decimal point, minus sign
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' | '.' | '-' => {
                            num_str.push(c);
                            chars.next();
                        }
                        _ => break, // non-numeric char: stop collecting
                    }
                }
                // match num_str.parse::<f64>().ok() {
                //     Some(n) => tokens.push(Token::Number(n)),
                //     _ => {}
                // }
                // Convert: string to f64
                if let Ok(n) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(n))
                }
            }

            // Unknown: skip
            _ => {
                println!("Skipping unknown character: {}", ch);
                chars.next();
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_braces() {
        let tokens = tokenize("{}");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
    }

    #[test]
    fn test_simple_string() {
        let tokens = tokenize(r#""hello""#);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
    }

    #[test]
    fn test_number() {
        let tokens = tokenize("42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
    }

    #[test]
    fn test_number_negative() {
        let tokens = tokenize("-42");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
    }

    #[test]
    fn test_number_simple_decimal() {
        let tokens = tokenize("3.14");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(3.14));
    }

    #[test]
    fn test_number_negative_decimal() {
        let tokens = tokenize("-0.99");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-0.99));
    }

    #[test]
    fn test_tokenize_string() {
        let tokens = tokenize(r#""hello world""#);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
    }

    #[test]
    fn test_boolean_and_null() {
        let tokens = tokenize("true false null");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
    }

    #[test]
    fn test_simple_object() {
        let tokens = tokenize(r#"{"name": "Alice"}"#);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("name".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::String("Alice".to_string()));
        assert_eq!(tokens[4], Token::RightBrace);
    }

    #[test]
    fn test_multiple_values() {
        let tokens = tokenize(r#"{"age": 30, "active": true}"#);

        // Verify we have the right tokens
        assert!(tokens.contains(&Token::String("age".to_string())));
        assert!(tokens.contains(&Token::Number(30.0)));
        assert!(tokens.contains(&Token::Comma));
        assert!(tokens.contains(&Token::String("active".to_string())));
        assert!(tokens.contains(&Token::Boolean(true)));
    }
}
