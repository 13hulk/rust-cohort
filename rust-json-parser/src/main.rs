//! JSON parser example.

use rust_json_parser::parser::parse_json;
use rust_json_parser::tokenizer::tokenize;

fn main() {
    // 1. Parsing primitive values
    println!("--- Parsing Primitive Values ---\n");
    let primitives = [r#""hello world""#, "42", "-3.14", "true", "false", "null"];
    for input in primitives {
        match parse_json(input) {
            Ok(value) => println!("{:>15} => {:?}", input, value),
            Err(e) => println!("{:>15} => Error: {}", input, e),
        }
    }

    // 2. Parsing simple JSON (currently only supports primitives)
    println!("\n--- Parsing Simple JSON ---\n");
    let simple_json = r#"{"name": "Alice", "age": 28}"#;
    println!("Input: {}\n", simple_json);
    match parse_json(simple_json) {
        Ok(value) => println!("Parsed: {:?}", value),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Tokenizing complex JSON
    println!("\n--- Tokenizing Complex JSON ---\n");
    let complex_json = r#"{
        "name": "Alice Johnson",
        "age": 28,
        "email": "alice@example.com",
        "active": true,
        "verified": false,
        "middle_name": null,
        "balance": 1250.75,
        "debt": -499.99,
        "temperature": -4,
        "zero": 0,
        "tiny": 0.001,
        "preferences": {
            "theme": "dark",
            "notifications": true,
            "language": "en"
        },
        "tags": ["developer", "rust", "python"],
        "metadata": {
            "created": "2023-01-15T10:30:00Z",
            "updated": "2023-12-01T15:45:30Z"
        }
    }"#;
    println!("Input: {}\n", complex_json);
    println!("Tokens:");
    match tokenize(complex_json) {
        Ok(tokens) => {
            for token in &tokens {
                println!("  {:?}", token);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
