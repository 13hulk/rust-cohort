//! JSON parser demo showcasing all features.

use std::time::Instant;

use rust_json_parser::error::JsonError;
use rust_json_parser::parser::{JsonParser, parse_json};
use rust_json_parser::tokenizer::Tokenizer;
use rust_json_parser::value::JsonValue;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--benchmark" {
        run_benchmark();
        return;
    }
    let document = show_parsing();
    show_object_access(&document);
    show_array_access(&document);
    show_primitives(&document);
    show_round_trip();
    show_edge_cases();
    show_escapes();
    show_tokenization();
    show_struct_api();
    show_error_handling();
    show_python_bindings();
}

fn show_parsing() -> JsonValue {
    println!("=== Parsing a Complete JSON Document ===\n");
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
    value
}

fn show_object_access(document: &JsonValue) {
    println!("=== Object Field Access ===\n");
    println!("  .get(\"name\")     => {:?}", document.get("name"));
    println!("  .get(\"age\")      => {:?}", document.get("age"));
    println!("  .get(\"active\")   => {:?}", document.get("active"));
    println!("  .get(\"nickname\") => {:?}", document.get("nickname"));
    println!("  .get(\"missing\")  => {:?}", document.get("missing"));

    let address = document.get("address").unwrap();
    println!(
        "  .get(\"address\").get(\"city\") => {:?}",
        address.get("city")
    );
}

fn show_array_access(document: &JsonValue) {
    println!("\n=== Array Access ===\n");
    let tags = document.get("tags").unwrap();
    println!("  tags: {}", tags);
    if let Some(arr) = tags.as_array() {
        println!("  .as_array().len()  => {}", arr.len());
        for (i, item) in arr.iter().enumerate() {
            println!("  [{}] => {}", i, item);
        }
    }
    println!("  .get_index(0) => {:?}", tags.get_index(0));
    println!("  .get_index(5) => {:?}", tags.get_index(5));
}

fn show_primitives(document: &JsonValue) {
    println!("\n=== Primitive Accessors ===\n");
    let name_val = document.get("name").unwrap();
    let age_val = document.get("age").unwrap();
    let active_val = document.get("active").unwrap();
    let null_val = document.get("nickname").unwrap();

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

    println!("  String .get(\"key\")     => {:?}", name_val.get("key"));
    println!("  Number .get_index(0)   => {:?}", age_val.get_index(0));
    println!("  String .as_array()     => {:?}", name_val.as_array());
    println!("  String .as_object()    => {:?}", name_val.as_object());
}

fn show_round_trip() {
    println!("\n=== Display Round-Trip ===\n");
    let input = r#"[1,"two",true,null]"#;
    let parsed = parse_json(input).unwrap();
    let serialized = parsed.to_string();
    let reparsed = parse_json(&serialized).unwrap();
    println!("  Original:   {}", input);
    println!("  Serialized: {}", serialized);
    println!("  Re-parsed:  {}", reparsed);
    println!("  Match: {}", parsed == reparsed);
}

fn show_edge_cases() {
    println!("\n=== Edge Cases ===\n");
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
}

fn show_escapes() {
    println!("\n=== Escape Sequences ===\n");
    let input = r#"{"tab": "a\tb", "newline": "a\nb", "quote": "a\"b", "backslash": "a\\b", "unicode": "\u0041\u0042\u0043"}"#;
    let escaped = parse_json(input).unwrap();
    println!("  Input: {}", input);
    let fields = ["tab", "newline", "quote", "backslash", "unicode"];
    for field in &fields {
        if let Some(val) = escaped.get(field) {
            println!(
                "  {:<10} {:?} (Display: {})",
                format!("{}:", field),
                val.as_str().unwrap(),
                val
            );
        }
    }
}

fn show_tokenization() {
    println!("\n=== Tokenization ===\n");
    let input = r#"{"items": [1, true], "ok": null}"#;
    println!("  Input: {}\n  Tokens:", input);
    match Tokenizer::new(input).tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("    {:?}", token);
            }
            println!("  Total: {} tokens", tokens.len());
        }
        Err(e) => println!("  Error: {}", e),
    }
}

fn show_struct_api() {
    println!("\n=== JsonParser Struct API ===\n");
    let input = r#"{"method": "struct"}"#;
    println!("  Input: {}", input);
    match JsonParser::new(input) {
        Ok(mut parser) => match parser.parse() {
            Ok(val) => println!("  Result: {}", val),
            Err(e) => println!("  Parse error: {}", e),
        },
        Err(e) => println!("  Tokenize error: {}", e),
    }

    let conv_result = parse_json(input).unwrap();
    println!("  parse_json() gives same result: {}", conv_result);
}

fn show_error_handling() {
    println!("\n=== Error Handling ===\n");
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
}

fn show_python_bindings() {
    println!("\n=== Python Bindings ===\n");
    println!("  This parser is also available as a Python package via PyO3.");
    println!("  Build:   make python-build");
    println!("  Test:    make python-test");
    println!("  Usage:");
    println!("    import rust_json_parser as rjp");
    println!("    data = rjp.parse_json('{{\"key\": \"value\"}}')");
    println!("    print(rjp.dumps(data, indent=2))");
    println!("    python -m rust_json_parser data.json");
}

fn run_benchmark() {
    println!("=== Pure Rust JSON Parser Benchmark (release build) ===\n");

    let files = [
        ("small.json", "Single flat object with 6 fields"),
        ("medium.json", "Array of 75 objects with nested address"),
        ("large.json", "Array of 750 objects with nested address"),
        (
            "xlarge.json",
            "1230 objects with long strings and nested metadata",
        ),
        (
            "nested.json",
            "Deeply nested objects and arrays (228 levels)",
        ),
    ];

    for (filename, description) in &files {
        let path = format!("benchmarks/{}", filename);
        let input = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                println!("  Skipping {} -- {}", filename, e);
                continue;
            }
        };

        println!(
            "--- {} -- {} -- {} bytes ---\n",
            filename,
            description,
            input.len()
        );

        let mut parser = JsonParser::new_reusable();
        for iterations in [100, 1_000, 10_000] {
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = parser.reparse(&input).unwrap();
            }
            let elapsed = start.elapsed();
            let per_iter_us = elapsed.as_secs_f64() * 1_000_000.0 / iterations as f64;
            println!(
                "  {:>6} iterations: {:.6}s  ({:.1} us/iter)",
                iterations,
                elapsed.as_secs_f64(),
                per_iter_us,
            );
        }
        println!();
    }
}
