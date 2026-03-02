# Rust JSON Parser - Architecture & API

## Architecture

Two-phase pipeline: raw JSON text -> token stream -> value tree.

```
"{"key": [1, true]}"
        |
        v
  Tokenizer::tokenize()
        |
        v
[LeftBrace, String("key"), Colon, LeftBracket, Number(1.0), Comma, Boolean(true), RightBracket, RightBrace]
        |
        v
  JsonParser::parse()
        |
        v
Object({"key": Array([Number(1.0), Boolean(true)])})
```

## Modules

### Tokenizer (`tokenizer.rs`)

Converts a JSON string into a flat token stream.

```rust
use rust_json_parser::tokenizer::Tokenizer;

let tokens = Tokenizer::new(r#"{"key": 42}"#).tokenize().unwrap();
// [LeftBrace, String("key"), Colon, Number(42.0), RightBrace]
```

**Token variants:** `LeftBrace`, `RightBrace`, `LeftBracket`, `RightBracket`, `Comma`, `Colon`, `String(String)`, `Number(f64)`, `Boolean(bool)`, `Null`

**Escape sequences:** `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX` (unicode)

### Parser (`parser.rs`)

Recursive descent parser that walks the token stream and builds a value tree.

```rust
use rust_json_parser::parser::{parse_json, JsonParser};

// Convenience function
let value = parse_json(r#"{"name": "Alice"}"#).unwrap();

// Or step-by-step with the struct API
let mut parser = JsonParser::new(r#"{"name": "Alice"}"#).unwrap();
let value = parser.parse().unwrap();
```

### Value (`value.rs`)

`JsonValue` enum with 6 variants and accessor methods.

```rust
use rust_json_parser::parser::parse_json;

let value = parse_json(r#"{"name": "Alice", "scores": [95, 87]}"#).unwrap();

// Object access
value.get("name");              // Some(&JsonValue::String("Alice"))
value.get("missing");           // None

// Array access
let scores = value.get("scores").unwrap();
scores.get_index(0);            // Some(&JsonValue::Number(95.0))
scores.as_array().unwrap().len(); // 2

// Primitive extraction
value.get("name").unwrap().as_str();   // Some("Alice")
value.get("name").unwrap().as_f64();   // None (wrong type)
value.get("name").unwrap().is_null();  // false

// Display (serializes back to valid JSON)
let json_string = value.to_string();
let reparsed = parse_json(&json_string).unwrap();
assert_eq!(value, reparsed);
```

**Variants:** `Null`, `Boolean(bool)`, `Number(f64)`, `String(String)`, `Array(Vec<JsonValue>)`, `Object(HashMap<String, JsonValue>)`

**Accessors:** `is_null()`, `as_str()`, `as_f64()`, `as_bool()`, `as_array()`, `as_object()`, `get(key)`, `get_index(i)`

### Error (`error.rs`)

All errors carry position information for diagnostics.

```rust
use rust_json_parser::parser::parse_json;
use rust_json_parser::error::JsonError;

match parse_json("@invalid") {
    Err(JsonError::UnexpectedToken { expected, found, position }) => {
        // expected: "valid JSON token", found: "@", position: 0
    }
    _ => {}
}
```

**Variants:** `UnexpectedToken`, `UnexpectedEndOfInput`, `InvalidNumber`, `InvalidEscape`, `InvalidUnicode`

## Building

```bash
cargo build --no-default-features    # Rust-only build (no Python linkage)
cargo test --no-default-features     # Run 159 Rust tests
cargo check --features python        # Check with PyO3 enabled
```

The `--no-default-features` flag excludes the PyO3 dependency, which requires a Python environment to link against.
