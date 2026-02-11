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

-
-
-

## Week 05: 2026-02-16 - 2026-02-22

-
-
-

## Week 06: 2026-02-23 - 2026-03-01

-
-
-
