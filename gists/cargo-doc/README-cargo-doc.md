# Cargo Doc - Rust's Built-in Documentation Generator

## What is `cargo doc`?

`cargo doc` generates HTML documentation from your Rust code and doc comments. It's built into Cargo - no plugins or configuration needed.

## Basic Usage

```bash
# Generate documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

## Writing Doc Comments

Use `///` for documentation comments (these become part of the generated docs):

```rust
/// Parses a JSON string and returns a JsonValue.
///
/// # Arguments
/// * `input` - A string slice containing valid JSON
///
/// # Returns
/// * `Ok(JsonValue)` - Successfully parsed value
/// * `Err(JsonError)` - Parse error with position info
///
/// # Example
/// ```
/// let value = parse_json("42").unwrap();
/// ```
pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    // ...
}
```

## Module-Level Docs

Use `//!` at the top of a file for module documentation:

```rust
//! JSON parser library.
//!
//! This crate provides tokenization and parsing of JSON primitives.

pub mod error;
pub mod parser;
```

## What You Get

Running `cargo doc --open` generates:
- Browsable HTML documentation
- Automatic linking between types and functions
- Search functionality
- Source code links
- Organized by modules

## Output Location

Documentation is generated in:
```
target/doc/<crate_name>/index.html
```

## Why It's Great

1. **Zero config** - Works out of the box
2. **Consistent style** - All Rust docs look the same
3. **Code examples are tested** - Doc tests run with `cargo test`
4. **Part of the ecosystem** - Same format as docs.rs

## Related Commands

```bash
cargo doc --open          # Generate and view
cargo test --doc          # Run doc tests only
cargo doc --no-deps       # Skip dependencies (faster)
```

## Example Output

For our JSON parser, `cargo doc --open` generates docs showing:
- `JsonError` enum with all variants
- `JsonValue` enum with all variants
- `parse_json()` function with signature
- `tokenize()` function with signature
- Module hierarchy (error, parser, tokenizer, value)
