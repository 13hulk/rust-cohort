# rust-json-parser

A JSON parser in Rust with Python bindings via PyO3. Pure Rust core with zero external dependencies. Two-phase pipeline: tokenization of JSON text into semantic tokens, then recursive descent parsing into a value tree.

## Features

- Full JSON spec: objects, arrays, strings (with escape sequences including `\uXXXX`), numbers, booleans, null
- Two-phase pipeline: tokenizer produces a token stream, parser builds a value tree
- Accessor methods for type-safe value extraction (`get`, `get_index`, `as_str`, `as_f64`, etc.)
- `Display` trait for JSON serialization (round-trip capable)
- Python bindings via PyO3 with native type conversion (dict, list, str, float, bool, None)
- CLI tool: `python -m rust_json_parser`

## Quick Start

### Rust

```bash
make all        # format + lint + test + build
make run        # run the demo binary (showcases all features)
cargo test      # run 159 tests
```

### Python

```bash
make python-all    # build + format + lint + typecheck + test
make python-run    # run the Python CLI demo
```

Requires [uv](https://docs.astral.sh/uv/) for Python dependency management.

## Project Structure

```
src/
  tokenizer.rs       # Phase 1: JSON text -> token stream
  parser.rs          # Phase 2: token stream -> value tree
  value.rs           # JsonValue enum with accessors and Display
  error.rs           # JsonError enum with position info
  python_bindings.rs # PyO3 bindings (feature-gated)
  lib.rs             # Module declarations
  main.rs            # Demo binary
python/
  rust_json_parser/  # Python package wrapper
tests/
  test_python_integration.py  # 11 pytest tests
```

## Documentation

- [Rust Parser Architecture & API](src/README.md)
- [Python Bindings & CLI](python/README.md)

## Development

```bash
# Rust
make fmt           # format
make clippy        # lint (warnings as errors)
make test          # run tests
make all           # all of the above + build

# Python
make python-fmt    # format with ruff
make python-lint   # lint with ruff
make python-typecheck  # type check with ty
make python-test   # run pytest
make python-all    # all of the above

# Pre-commit hooks (Rust fmt/clippy + Python ruff)
make pre-commit-install
```
