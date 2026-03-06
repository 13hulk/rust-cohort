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
    if let Some(document) = show_parsing() {
        show_object_access(&document);
        show_array_access(&document);
        show_primitives(&document);
    }
    show_round_trip();
    show_edge_cases();
    show_escapes();
    show_tokenization();
    show_struct_api();
    show_error_handling();
    show_python_bindings();
}

fn show_parsing() -> Option<JsonValue> {
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
    match parse_json(input) {
        Ok(value) => {
            println!("Input:\n{}\n", input);
            println!("Display output: {}\n", value);
            Some(value)
        }
        Err(e) => {
            println!("  Parse error: {}", e);
            None
        }
    }
}

fn show_object_access(document: &JsonValue) {
    println!("=== Object Field Access ===\n");
    println!("  .get(\"name\")     => {:?}", document.get("name"));
    println!("  .get(\"age\")      => {:?}", document.get("age"));
    println!("  .get(\"active\")   => {:?}", document.get("active"));
    println!("  .get(\"nickname\") => {:?}", document.get("nickname"));
    println!("  .get(\"missing\")  => {:?}", document.get("missing"));

    if let Some(address) = document.get("address") {
        println!(
            "  .get(\"address\").get(\"city\") => {:?}",
            address.get("city")
        );
    }
}

fn show_array_access(document: &JsonValue) {
    println!("\n=== Array Access ===\n");
    if let Some(tags) = document.get("tags") {
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
}

fn show_primitives(document: &JsonValue) {
    println!("\n=== Primitive Accessors ===\n");
    let name_val = document.get("name");
    let age_val = document.get("age");
    let active_val = document.get("active");
    let null_val = document.get("nickname");

    if let Some(name) = name_val {
        println!("  String \"Alice\":");
        println!("    .as_str()  => {:?}", name.as_str());
        println!("    .as_f64()  => {:?}", name.as_f64());
        println!("    .as_bool() => {:?}", name.as_bool());
        println!("    .is_null() => {}", name.is_null());
    }

    if let Some(age) = age_val {
        println!("  Number 28:");
        println!("    .as_f64()  => {:?}", age.as_f64());
        println!("    .as_str()  => {:?}", age.as_str());
    }

    if let Some(active) = active_val {
        println!("  Boolean true:");
        println!("    .as_bool() => {:?}", active.as_bool());
        println!("    .as_f64()  => {:?}", active.as_f64());
    }

    if let Some(null) = null_val {
        println!("  Null:");
        println!("    .is_null() => {}", null.is_null());
        println!("    .as_str()  => {:?}", null.as_str());
    }

    if let Some(name) = name_val {
        println!("  String .get(\"key\")     => {:?}", name.get("key"));
        println!("  String .as_array()     => {:?}", name.as_array());
        println!("  String .as_object()    => {:?}", name.as_object());
    }
    if let Some(age) = age_val {
        println!("  Number .get_index(0)   => {:?}", age.get_index(0));
    }
}

fn show_round_trip() {
    println!("\n=== Display Round-Trip ===\n");
    let input = r#"[1,"two",true,null]"#;
    match parse_json(input) {
        Ok(parsed) => {
            let serialized = parsed.to_string();
            match parse_json(&serialized) {
                Ok(reparsed) => {
                    println!("  Original:   {}", input);
                    println!("  Serialized: {}", serialized);
                    println!("  Re-parsed:  {}", reparsed);
                    println!("  Match: {}", parsed == reparsed);
                }
                Err(e) => println!("  Re-parse error: {}", e),
            }
        }
        Err(e) => println!("  Parse error: {}", e),
    }
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
        match parse_json(json) {
            Ok(result) => println!("  {:<16} {} => {}", label, json, result),
            Err(e) => println!("  {:<16} {} => error: {}", label, json, e),
        }
    }
}

fn show_escapes() {
    println!("\n=== Escape Sequences ===\n");
    let input = r#"{"tab": "a\tb", "newline": "a\nb", "quote": "a\"b", "backslash": "a\\b", "unicode": "\u0041\u0042\u0043"}"#;
    let escaped = match parse_json(input) {
        Ok(v) => v,
        Err(e) => {
            println!("  Parse error: {}", e);
            return;
        }
    };
    println!("  Input: {}", input);
    let fields = ["tab", "newline", "quote", "backslash", "unicode"];
    for field in &fields {
        if let Some(val) = escaped.get(field)
            && let Some(s) = val.as_str()
        {
            println!("  {:<10} {:?} (Display: {})", format!("{}:", field), s, val);
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

    match parse_json(input) {
        Ok(conv_result) => println!("  parse_json() gives same result: {}", conv_result),
        Err(e) => println!("  parse_json() error: {}", e),
    }
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
                if let Err(e) = parser.reparse(&input) {
                    println!("  Parse error: {}", e);
                    break;
                }
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
