//! JSON parser demo showcasing all features.

use rust_json_parser::error::JsonError;
use rust_json_parser::parser::{JsonParser, parse_json};
use rust_json_parser::tokenizer::Tokenizer;

fn main() {
    // 1. Parse a complex JSON document with all value types
    println!("=== 1. Parsing a Complete JSON Document ===\n");
    let input = r#"{
        "name": "Alice",
        "age": 28,
        "score": 95.5,
        "active": true,
        "nickname": null,
        "tags": ["developer", "rust"],
        "address": {"city": "Portland", "state": "OR"}
    }"#;
    let value = parse_json(input).unwrap();
    println!("Input:\n{}\n", input);
    println!("Display output: {}\n", value);

    // 2. Object field access with .get()
    println!("=== 2. Object Field Access ===\n");
    println!("  .get(\"name\")     => {:?}", value.get("name"));
    println!("  .get(\"age\")      => {:?}", value.get("age"));
    println!("  .get(\"active\")   => {:?}", value.get("active"));
    println!("  .get(\"nickname\") => {:?}", value.get("nickname"));
    println!("  .get(\"missing\")  => {:?}", value.get("missing"));

    // Nested access
    let address = value.get("address").unwrap();
    println!(
        "  .get(\"address\").get(\"city\") => {:?}",
        address.get("city")
    );

    // 3. Array access with .get_index() and .as_array()
    println!("\n=== 3. Array Access ===\n");
    let tags = value.get("tags").unwrap();
    println!("  tags: {}", tags);
    if let Some(arr) = tags.as_array() {
        println!("  .as_array().len()  => {}", arr.len());
        for (i, item) in arr.iter().enumerate() {
            println!("  [{}] => {}", i, item);
        }
    }
    println!("  .get_index(0) => {:?}", tags.get_index(0));
    println!("  .get_index(5) => {:?}", tags.get_index(5));

    // 4. Primitive accessors — success and failure cases
    println!("\n=== 4. Primitive Accessors ===\n");
    let name_val = value.get("name").unwrap();
    let age_val = value.get("age").unwrap();
    let active_val = value.get("active").unwrap();
    let null_val = value.get("nickname").unwrap();

    println!("  String \"Alice\":");
    println!("    .as_str()  => {:?}", name_val.as_str());
    println!("    .as_f64()  => {:?}", name_val.as_f64());
    println!("    .as_bool() => {:?}", name_val.as_bool());
    println!("    .is_null() => {}", name_val.is_null());

    println!("  Number 28:");
    println!("    .as_f64()  => {:?}", age_val.as_f64());
    println!("    .as_str()  => {:?}", age_val.as_str());

    println!("  Boolean true:");
    println!("    .as_bool() => {:?}", active_val.as_bool());
    println!("    .as_f64()  => {:?}", active_val.as_f64());

    println!("  Null:");
    println!("    .is_null() => {}", null_val.is_null());
    println!("    .as_str()  => {:?}", null_val.as_str());

    // Calling object accessors on non-objects
    println!("  String .get(\"key\")     => {:?}", name_val.get("key"));
    println!("  Number .get_index(0)   => {:?}", age_val.get_index(0));
    println!("  String .as_array()     => {:?}", name_val.as_array());
    println!("  String .as_object()    => {:?}", name_val.as_object());

    // 5. Display round-trip: parse -> to_string -> re-parse
    println!("\n=== 5. Display Round-Trip ===\n");
    let array_input = r#"[1,"two",true,null]"#;
    let parsed = parse_json(array_input).unwrap();
    let serialized = parsed.to_string();
    let reparsed = parse_json(&serialized).unwrap();
    println!("  Original:   {}", array_input);
    println!("  Serialized: {}", serialized);
    println!("  Re-parsed:  {}", reparsed);
    println!("  Match: {}", parsed == reparsed);

    // 6. Edge cases
    println!("\n=== 6. Edge Cases ===\n");
    let cases = [
        ("Empty array", "[]"),
        ("Empty object", "{}"),
        ("Nested empty", r#"{"a": [], "b": {}}"#),
        ("Deeply nested", "[[[1]]]"),
        ("Single string", r#""hello""#),
        ("Single number", "42"),
        ("Single boolean", "true"),
        ("Single null", "null"),
        ("Negative number", "-3.14"),
    ];
    for (label, json) in &cases {
        let result = parse_json(json).unwrap();
        println!("  {:<16} {} => {}", label, json, result);
    }

    // 7. Escape sequences
    println!("\n=== 7. Escape Sequences ===\n");
    let escape_input = r#"{"tab": "a\tb", "newline": "a\nb", "quote": "a\"b", "backslash": "a\\b", "unicode": "\u0041\u0042\u0043"}"#;
    let escaped = parse_json(escape_input).unwrap();
    println!("  Input: {}", escape_input);
    if let Some(tab) = escaped.get("tab") {
        println!(
            "  tab:       {:?} (Display: {})",
            tab.as_str().unwrap(),
            tab
        );
    }
    if let Some(nl) = escaped.get("newline") {
        println!("  newline:   {:?} (Display: {})", nl.as_str().unwrap(), nl);
    }
    if let Some(q) = escaped.get("quote") {
        println!("  quote:     {:?} (Display: {})", q.as_str().unwrap(), q);
    }
    if let Some(bs) = escaped.get("backslash") {
        println!("  backslash: {:?} (Display: {})", bs.as_str().unwrap(), bs);
    }
    if let Some(uni) = escaped.get("unicode") {
        println!(
            "  unicode:   {:?} (Display: {})",
            uni.as_str().unwrap(),
            uni
        );
    }

    // 8. Tokenization
    println!("\n=== 8. Tokenization ===\n");
    let token_input = r#"{"items": [1, true], "ok": null}"#;
    println!("  Input: {}\n  Tokens:", token_input);
    match Tokenizer::new(token_input).tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("    {:?}", token);
            }
            println!("  Total: {} tokens", tokens.len());
        }
        Err(e) => println!("  Error: {}", e),
    }

    // 9. JsonParser struct API (step-by-step)
    println!("\n=== 9. JsonParser Struct API ===\n");
    let step_input = r#"{"method": "struct"}"#;
    println!("  Input: {}", step_input);
    match JsonParser::new(step_input) {
        Ok(mut parser) => match parser.parse() {
            Ok(val) => println!("  Result: {}", val),
            Err(e) => println!("  Parse error: {}", e),
        },
        Err(e) => println!("  Tokenize error: {}", e),
    }

    // Compare with convenience function
    let conv_result = parse_json(step_input).unwrap();
    println!("  parse_json() gives same result: {}", conv_result);

    // 10. Error handling
    println!("\n=== 10. Error Handling ===\n");
    let error_cases: Vec<(&str, &str)> = Vec::from([
        ("Empty input", ""),
        ("Invalid token", "@bad"),
        ("Unclosed string", r#""unterminated"#),
        ("Unclosed array", "[1, 2"),
        ("Trailing comma", "[1, 2,]"),
        ("Missing colon", r#"{"key" "value"}"#),
        ("Extra tokens", r#"true false"#),
    ]);
    for (label, json) in error_cases.iter() {
        match parse_json(json) {
            Ok(_) => println!("  {:<18} => unexpectedly succeeded", label),
            Err(e) => println!("  {:<18} => {}", label, e),
        }
    }

    // Show error variant matching
    println!("\n  Pattern matching on JsonError:");
    match parse_json("@") {
        Err(JsonError::UnexpectedToken {
            expected,
            found,
            position,
        }) => {
            println!(
                "    UnexpectedToken {{ expected: {:?}, found: {:?}, position: {} }}",
                expected, found, position
            );
        }
        other => println!("    Unexpected: {:?}", other),
    }

    // 11. Python bindings
    println!("\n=== 11. Python Bindings ===\n");
    println!("  This parser is also available as a Python package via PyO3.");
    println!("  Build:   make python-build");
    println!("  Test:    make python-test");
    println!("  Usage:");
    println!("    import rust_json_parser as rjp");
    println!("    data = rjp.parse_json('{{\"key\": \"value\"}}')");
    println!("    print(rjp.dumps(data, indent=2))");
    println!("    python -m rust_json_parser data.json");
}
