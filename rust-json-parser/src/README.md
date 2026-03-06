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

Scans JSON text byte by byte and produces a flat token stream. Pre-allocates the token vector with `Vec::with_capacity(input.len() / 3)` based on the heuristic that tokens average roughly 3 bytes each. Supports buffer reuse via `retokenize()` and `tokenize_into()` for benchmark loops.

```rust
use rust_json_parser::tokenizer::Tokenizer;

let tokens = Tokenizer::new(r#"{"key": 42}"#).tokenize().unwrap();
// [LeftBrace, String("key"), Colon, Number(42.0), RightBrace]
```

**Token variants:** `LeftBrace`, `RightBrace`, `LeftBracket`, `RightBracket`, `Comma`, `Colon`, `String(String)`, `Number(f64)`, `Boolean(bool)`, `Null`

**Escape sequences:** `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX` (unicode)

**Capacity hints:** String buffers (`String::with_capacity(32)`). Keywords and numbers use slicing (no allocation needed).

### Parser (`parser.rs`)

Recursive descent parser that walks the token stream and builds a value tree. Pre-allocates arrays with `Vec::with_capacity` and objects with `HashMap::with_capacity` based on remaining token count estimates. Reuses internal buffers across multiple `parse()` calls.

```rust
use rust_json_parser::parser::{parse_json, JsonParser};

// Convenience function
let value = parse_json(r#"{"name": "Alice"}"#).unwrap();

// Or with the struct API (reuses buffers across calls)
let mut parser = JsonParser::new();
let value = parser.parse(r#"{"name": "Alice"}"#).unwrap();
```

### Value (`value.rs`)

`JsonValue` enum with 6 variants and accessor methods. The `Display` implementation uses a private `JsonFormat` trait with per-type formatting methods. String escaping uses byte scanning with bulk `push_str()` copies for unescaped segments.

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

## Documentation

The crate enforces `#![warn(missing_docs)]` at the top of `lib.rs`, ensuring all public items (modules, structs, enums, variants, functions, methods) have doc comments. Many doc comments include runnable examples that serve as both documentation and tests.

```bash
make doc        # generate docs (open with: cargo doc --no-default-features --open)
make doc-test   # run 18 doc tests
```

## Optimizations

Memory pre-allocation is used throughout to reduce allocations:

- **Tokenizer:** Token vector sized to `input.len() / 3`, string buffers to 32 chars. Buffer reuse via `retokenize()`
- **Parser:** Arrays and objects pre-allocated based on remaining token count (capped). Buffer reuse via `reparse()`
- **Value:** String escaping via byte scan + bulk `push_str()` copies
- **Python bindings:** Conversion buffers for lists, dicts, and serialization output

## Building

```bash
make build      # Rust-only build (no Python linkage)
make test       # Run 180 Rust tests (162 unit + 18 doc)
make doc        # Generate API documentation
make all        # fmt + clippy + test + build
```

All Makefile targets use `--no-default-features` to exclude PyO3 (which requires a Python environment to link against).
