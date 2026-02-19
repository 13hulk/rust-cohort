//! JSON parser demo showing tokenization, parsing, and value access.

use rust_json_parser::parser::parse_json;
use rust_json_parser::tokenizer::Tokenizer;

fn main() {
    // 1. Parse a complete JSON document with nested structures
    println!("--- Parsing a Complete JSON Document ---\n");
    let json_input = r#"{
        "name": "Alice Johnson",
        "age": 28,
        "active": true,
        "score": 95.5,
        "nickname": null,
        "tags": ["developer", "rust", "json"],
        "address": {
            "city": "Portland",
            "state": "OR"
        }
    }"#;

    println!("Input: {}\n", json_input);

    match parse_json(json_input) {
        Ok(value) => {
            // Display output via Display trait
            println!("Display: {}\n", value);

            // Access object fields with .get()
            println!("--- Accessing Fields with .get() ---\n");
            if let Some(name) = value.get("name") {
                println!("  name     => {}", name);
            }
            if let Some(age) = value.get("age") {
                println!("  age      => {}", age);
            }
            if let Some(active) = value.get("active") {
                println!("  active   => {}", active);
            }
            if let Some(nickname) = value.get("nickname") {
                println!("  nickname => {}", nickname);
            }

            // Access nested object
            println!("\n--- Nested Object Access ---\n");
            if let Some(address) = value.get("address") {
                println!("  address  => {}", address);
                if let Some(city) = address.get("city") {
                    println!("  city     => {}", city);
                }
            }

            // Access array elements with .get_index() and .as_array()
            println!("\n--- Array Access ---\n");
            if let Some(tags) = value.get("tags") {
                println!("  tags     => {}", tags);
                if let Some(arr) = tags.as_array() {
                    println!("  count    => {}", arr.len());
                }
                if let Some(first) = tags.get_index(0) {
                    println!("  first    => {}", first);
                }
                if let Some(last) = tags.get_index(2) {
                    println!("  last     => {}", last);
                }
            }

            // Access the full object via .as_object()
            println!("\n--- Object Keys via .as_object() ---\n");
            if let Some(obj) = value.as_object() {
                println!("  total keys: {}", obj.len());
            }

            // Use accessor helpers for primitive extraction
            println!("\n--- Primitive Accessors ---\n");
            if let Some(age_val) = value.get("age") {
                println!("  age as f64: {:?}", age_val.as_f64());
            }
            if let Some(name_val) = value.get("name") {
                println!("  name as str: {:?}", name_val.as_str());
            }
            if let Some(active_val) = value.get("active") {
                println!("  active as bool: {:?}", active_val.as_bool());
            }
            if let Some(nick_val) = value.get("nickname") {
                println!("  nickname is_null: {}", nick_val.is_null());
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }

    // 2. Parse a JSON array of objects
    println!("\n--- Parsing an Array of Objects ---\n");
    let array_json = r#"[
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
        {"id": 3, "name": "Charlie"}
    ]"#;
    println!("Input: {}\n", array_json);

    match parse_json(array_json) {
        Ok(value) => {
            println!("Display: {}\n", value);
            if let Some(arr) = value.as_array() {
                for (i, item) in arr.iter().enumerate() {
                    if let Some(name) = item.get("name") {
                        println!("  [{}] name => {}", i, name);
                    }
                }
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }

    // 3. Tokenize a JSON document to show the token stream
    println!("\n--- Tokenizing JSON ---\n");
    let token_input = r#"{"items": [1, 2], "ok": true}"#;
    println!("Input: {}\n", token_input);
    println!("Tokens:");
    match Tokenizer::new(token_input).tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("  {:?}", token);
            }
        }
        Err(e) => println!("Tokenize error: {}", e),
    }
}
