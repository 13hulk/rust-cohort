# 🦀 Rust API Reference

**The guts of the parser.** Module-by-module guide with code examples you can copy-paste and actually run.

| | |
|---|---|
| 🦀 Modules | ![modules](https://img.shields.io/badge/modules-5-blue) |
| 📖 Doc tests | ![doc tests](https://img.shields.io/badge/doc_tests-18_passing-brightgreen) |
| ⚡ Optimizations | ![opt sites](https://img.shields.io/badge/with__capacity-12_sites-blue) |

## 📑 Index

- [🔍 Tokenizer](#-tokenizer--tokenizerrs) — byte scanner, token variants, escape handling
- [🌳 Parser](#-parser--parserrs) — recursive descent, convenience API, struct API
- [💎 Value](#-value--valuers) — `JsonValue` enum, accessors, serialization
- [❌ Error](#-error--errorrs) — error variants, positional diagnostics
- [🚀 Optimizations](#-optimizations) — capacity hints, buffer reuse, byte-scan tricks
- **[⬆ Project Internals](../README.md)** · **[🐍 Python API](../python/README.md)**

---

## 🔍 Tokenizer · `tokenizer.rs`

Scans JSON byte-by-byte into a flat token stream. Pre-allocates with `Vec::with_capacity(input.len() / 3)` — tokens average ~3 bytes each.

```rust
use rust_json_parser::tokenizer::Tokenizer;

let tokens = Tokenizer::new(r#"{"key": 42}"#).tokenize().unwrap();
// → [LeftBrace, String("key"), Colon, Number(42.0), RightBrace]
```

**Token variants:** `LeftBrace` `RightBrace` `LeftBracket` `RightBracket` `Comma` `Colon` `String(String)` `Number(f64)` `Boolean(bool)` `Null`

**Escape sequences:** `\"` `\\` `\/` `\b` `\f` `\n` `\r` `\t` `\uXXXX`

**Buffer reuse:** `retokenize()` clears and re-scans (reuses the `Vec<Token>` allocation). `tokenize_into()` appends to an existing vector.

---

## 🌳 Parser · `parser.rs`

Recursive descent over the token stream. Two ways to use it:

```rust
use rust_json_parser::parser::{parse_json, JsonParser};

// Quick and easy — one-liner
let value = parse_json(r#"{"name": "Alice"}"#).unwrap();

// Reusable — amortizes allocations across multiple parses
let mut parser = JsonParser::new();
let v1 = parser.parse(r#"{"a": 1}"#).unwrap();
let v2 = parser.parse(r#"{"b": 2}"#).unwrap();  // reuses internal buffers
```

Pre-allocates arrays/objects with `with_capacity()` based on remaining token count estimates.

---

## 💎 Value · `value.rs`

Six variants. Type-safe accessors. Round-trip serialization via `Display`.

```rust
use rust_json_parser::parser::parse_json;

let v = parse_json(r#"{"name": "Alice", "scores": [95, 87]}"#).unwrap();

v.get("name").unwrap().as_str();        // Some("Alice")
v.get("scores").unwrap().get_index(0);  // Some(&Number(95.0))
v.get("missing");                       // None — no panics, ever

// Round-trip: Display serializes back to valid JSON
let json_str = v.to_string();
let reparsed = parse_json(&json_str).unwrap();
assert_eq!(v, reparsed);  // ✅
```

| Variant | 🦀 Type | Accessor |
|---------|---------|----------|
| `Null` | — | `is_null()` |
| `Boolean(bool)` | `bool` | `as_bool()` |
| `Number(f64)` | `f64` | `as_f64()` |
| `String(String)` | `String` | `as_str()` |
| `Array(Vec<JsonValue>)` | `Vec` | `as_array()`, `get_index(i)` |
| `Object(HashMap<String, JsonValue>)` | `HashMap` | `as_object()`, `get(key)` |

The `Display` impl uses a private `JsonFormat` trait — per-type formatting methods, no giant match blocks. String escaping uses byte-scanning with bulk `push_str()` for unescaped runs.

---

## ❌ Error · `error.rs`

Every error knows *where* it happened. Position info is baked into each variant.

```rust
use rust_json_parser::parser::parse_json;
use rust_json_parser::error::JsonError;

match parse_json("@invalid") {
    Err(JsonError::UnexpectedToken { expected, found, position }) => {
        // position: 0, found: "@" — straight to the crime scene 🔍
    }
    _ => {}
}
```

| Variant | When |
|---------|------|
| `UnexpectedToken` | Wrong token type at position |
| `UnexpectedEndOfInput` | JSON cut short |
| `InvalidNumber` | Malformed number literal |
| `InvalidEscape` | Bad escape sequence in string |
| `InvalidUnicode` | Bad `\uXXXX` codepoint |

---

## 🚀 Optimizations

Memory pre-allocation throughout. No allocation left behind. 🪖

| Technique | Where | Heuristic |
|-----------|-------|-----------|
| `Vec::with_capacity()` | Token vector | `input.len() / 3` |
| `String::with_capacity()` | String token buffer | 32 chars |
| `Vec::with_capacity()` | Array values | `remaining_tokens / 2` (capped) |
| `HashMap::with_capacity()` | Object entries | `remaining_tokens / 4` (capped) |
| `retokenize()` | Benchmark loops | Reuses token vector across iterations |
| `reparse()` | Benchmark loops | Reuses parser buffers across iterations |
| Byte-scan + bulk copy | `to_json_string()` | Scan for escapes, `push_str()` unescaped segments |
