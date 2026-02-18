# Wins of the Week

Once a week, come back to this file and document your "wins of the week".

Anything you consider a win counts but even better if it's a result of your progress in the Program.

## Week 01: 2026-01-19 - 2026-01-25

### Rust Cohort
- Joined the PDC Program
    - Attended the kick-off session, met my cohort members and the coach
    - Familiarized myself with the course structure and resources provided

### Week 1 Goals and Rust Setup
- Identified my learning goals for the program
- Set up my development environment for Rust programming
    - Discovered useful Rust documentation and community forums
    - Started exploring Rust syntax and basic concepts
    - Found out Cargo == uv in Python

### Week 1 Curriculum
- Completed my first Rust "Hello, World!" program
- Completed week 1 curriculum assignments

## Week 02: 2026-01-26 - 2026-02-01

### Key Learning: Result vs Option
- `Result<T, E>` - for operations that can fail with an error (e.g., parsing invalid JSON)
- `Option<T>` - for values that may or may not exist, where absence isn't an error (e.g., `.peek()` on iterator)
- The try operator (`?`) replaces verbose match statements - makes happy path clean and readable
- Both use pattern matching: `Ok/Err` for Result, `Some/None` for Option

### Error Handling Foundation
- Implemented custom `JsonError` enum with struct variants (named fields)
- Each error carries position information for helpful error messages
- Learned the try operator (`?`) for elegant error propagation

### Type-Safe Parsing
- Created `JsonValue` enum for 4 primitive types (Null, Boolean, Number, String)
- Converted tokenizer from `Vec<Token>` to `Result<Vec<Token>, JsonError>`
- Built `parse_json()` function that parses primitive JSON values

### Testing Practices
- Learned table-driven testing pattern for testing multiple inputs concisely
- Understood how to test Option methods: `.is_some()`, `.is_none()`, `.unwrap_or()`

### Challenges
- Curriculum interpretation - Initially confused "Test Option methods" with "methods returning Option" - clarified it means testing Option's built-in methods like `.is_some()`, `.unwrap_or()`

## Week 03: 2026-02-02 - 2026-02-08

### Compiler Attributes: `#[allow(dead_code)]` and Code Quality
- Initially added `#[allow(dead_code)]` on `is_at_end()` in both `tokenizer.rs` and `parser.rs` because the methods were written in anticipation of Week 4's recursive descent parsing but not yet called
- Without the attribute, `cargo clippy -D warnings` (enforced by pre-commit hooks) would fail the build since it treats warnings as errors
- During PR review, got feedback that `advance()` and `peek()` could use `is_at_end()` instead of inline `self.position < self.input.len()` checks
- After making that change, `is_at_end()` was no longer dead code, so `#[allow(dead_code)]` was removed
- Learning: `#[allow(dead_code)]` suppresses the "function is never used" compiler warning -- useful as a temporary measure, but it is better to find a way to actually use the function now rather than leaving dead code annotations around

### From Functions to Structs
- Refactored both tokenizer and parser from free functions to struct-based designs
    - `Tokenizer` struct with `new(&str)`, `tokenize(&mut self)`, `peek(&self)`, `advance(&mut self)`
    - `JsonParser` struct with `new(&str) -> Result<Self>`, `parse(&mut self)`, `advance(&mut self)`
- Removed the `parse_json()` wrapper function, migrated all callers to `JsonParser` API directly

### Ownership and Borrowing in Practice
- Applied "borrow at boundary, own internally" pattern: `Tokenizer::new()` takes `&str` but stores `Vec<char>`
- Understood why `JsonValue::String` must contain `String` (owned) not `&str` (borrowed) -- the parsed value must outlive the parser
- Used `.clone()` in `advance()` to extract tokens from a `Vec` without moving -- first practical encounter with the borrow checker's constraints on indexed access

### Method Receiver Choices
- Made deliberate `&self` vs `&mut self` decisions for every method
    - `peek()` and `is_at_end()` use `&self` (read-only access)
    - `advance()`, `tokenize()`, and `parse()` use `&mut self` (modify position state)
- Understood the connection to borrowing rules: `&self` = shared reference, `&mut self` = exclusive reference

### Escape Sequence and Unicode Handling
- Implemented all 8 JSON escape sequences: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`
- Implemented `\uXXXX` unicode escapes using `u32::from_str_radix` and `char::from_u32`
- Added `InvalidEscape` and `InvalidUnicode` error variants with `Display` implementations
- Learned to use Rust raw strings (`r#"..."#`) for writing escape sequence tests

### Challenges
- The `.clone()` in parser `advance()` -- cannot move out of an indexed Vec, had to clone
- Choosing `Vec<char>` over `&str` with lifetimes -- don't fully understand lifetimes yet, so went with the simpler owned approach

## Week 04: 2026-02-09 - 2026-02-15

### Full Recursive Descent JSON Parser
- Implemented `parse_array()` and `parse_object()` with recursive descent -- the parser can now handle arbitrarily nested JSON structures
- Refactored `parse()` into `parse_value()` for recursive dispatch: `parse_value()` checks `peek()` and delegates to `parse_array()`, `parse_object()`, or handles primitives directly
- Added `Array(Vec<JsonValue>)` and `Object(HashMap<String, JsonValue>)` variants to `JsonValue` enum -- the data model is now complete for all 6 JSON types

### Display Trait for Round-Trip Serialization
- Implemented `Display` trait for `JsonValue` so any parsed value can be serialized back to a JSON string via `.to_string()`
- Built `escape_string()` helper to handle special characters (`"`, `\`, `\n`, `\r`, `\t`) in string output
- Whole numbers display cleanly: `42` not `42.0` (using `n.fract() == 0.0` check)
- Round-trip works: `parse_json(input).to_string()` produces valid JSON that can be re-parsed to the same value

### Accessor Methods and Convenience API
- Added `as_array()`, `as_object()`, `get()`, and `get_index()` accessor methods for ergonomic value access
- Re-introduced `parse_json()` as a convenience free function wrapping the struct constructor and parse call

### Collections and Iterators in Practice
- First real use of `Vec<T>` beyond token storage -- `Vec<JsonValue>` builds up array elements during parsing
- First real use of `HashMap<K,V>` -- `HashMap<String, JsonValue>` stores object key-value pairs
- Used `.iter().enumerate()` in Display for comma-separated array output, `for (key, value) in map` for objects

### Recursive Types Without `Box<T>`
- Discovered that `Vec` and `HashMap` provide heap indirection naturally, so `Box<T>` is not needed for recursive enum variants
- The enum has a fixed stack size because `Vec` and `HashMap` are just pointer + length + capacity on the stack

### Curriculum Compliance
- Avoided `std::mem::discriminant()` (not confirmed in curriculum) -- used `matches!` macro instead
- No closures used anywhere -- all iteration via `for` loops
- No custom traits, no generics, no explicit lifetimes, no external crates
- 159 tests passing (55 new), `make all` clean

### Challenges and Learnings
- HashMap ordering is nondeterministic -- learned to use `.contains()` for Display tests on objects instead of exact `assert_eq!`
- Trailing comma detection requires checking for closing bracket/brace after consuming a comma, before attempting to parse the next value
- Error messages use `format!("{:?}", token)` which shows Debug output -- not ideal for end users but sufficient for the curriculum scope

## Week 05: 2026-02-16 - 2026-02-22

-
-
-

## Week 06: 2026-02-23 - 2026-03-01

-
-
-
