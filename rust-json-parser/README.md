# rust-json-parser

A JSON parser in Rust with Python bindings via PyO3. Pure Rust core with zero external dependencies. Two-phase pipeline: tokenization of JSON text into semantic tokens, then recursive descent parsing into a value tree.

## Features

- Full JSON spec: objects, arrays, strings (with escape sequences including `\uXXXX`), numbers, booleans, null
- Two-phase pipeline: tokenizer produces a token stream, parser builds a value tree
- Accessor methods for type-safe value extraction (`get`, `get_index`, `as_str`, `as_f64`, etc.)
- `Display` trait for JSON serialization (round-trip capable)
- Comprehensive documentation with tested examples (`cargo doc`, 18 doc tests)
- Performance benchmarking against Python `json` and `simplejson`
- Memory-optimized with pre-allocated buffers (`with_capacity` throughout tokenizer, parser, and value modules)
- Python bindings via PyO3 with native type conversion (dict, list, str, float, bool, None)
- CLI tool: `python -m rust_json_parser`

## Quick Start

### Rust

```bash
make all        # format + lint + test + build
make run        # run the demo binary (showcases all features)
make doc        # generate API documentation
make doc-test   # run 18 doc tests
make test       # run 180 tests (162 unit + 18 doc)
```

### Python

```bash
make python-all    # build + format + lint + typecheck + test
make python-run    # run the Python CLI demo
make benchmark     # run 3-way benchmark (Rust vs json vs simplejson)
```

Requires [uv](https://docs.astral.sh/uv/) for Python dependency management.

## Benchmark Results

Three-way comparison of Rust parser, Python's built-in `json` module (C extension), and `simplejson` across different input sizes. Benchmark data files are deterministic (no randomness) and committed to the repository for reproducible results. Regenerate them with `make benchmark-data`.

### Scenarios

| File | Size | Description | Iterations |
|------|------|-------------|------------|
| small.json | ~110 bytes | Single flat object with 6 fields | 100, 1000 |
| medium.json | ~10 KB | Array of 75 objects with nested address | 100, 1000 |
| large.json | ~104 KB | Array of 750 objects with nested address | 100, 1000 |
| xlarge.json | ~500 KB | 1230 objects with long strings and nested metadata | 100, 1000 |
| nested.json | ~10 KB | Deeply nested objects and arrays (228 levels) | 100, 1000 |

Run `make benchmark` to reproduce; results are saved to [`benchmarks/results.md`](benchmarks/results.md).

## Documentation

All public items have doc comments with tested examples. The crate enforces `#![warn(missing_docs)]`.

```bash
make doc        # generate API docs (open with: cargo doc --no-default-features --open)
make doc-test   # run 18 doc tests
```

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
  test_python_integration.py  # 15 pytest tests
benchmarks/
  small.json         # ~110-byte test input
  medium.json        # ~10 KB test input
  large.json         # ~104 KB test input
  xlarge.json        # ~500 KB test input
  nested.json        # ~10 KB deeply nested input (228 levels)
  generate.py        # Script to regenerate sample JSON files
  results.md         # Latest benchmark results
```

## Documentation Links

- [Rust Parser Architecture & API](src/README.md)
- [Python Bindings & CLI](python/README.md)

## Development

```bash
# Rust
make fmt           # format
make clippy        # lint (warnings as errors)
make test          # run tests
make doc           # generate documentation
make doc-test      # run doc tests
make all           # fmt + clippy + test + build

# Python
make python-fmt    # format with ruff
make python-lint   # lint with ruff
make python-typecheck  # type check with ty
make python-test   # run pytest
make python-all    # all of the above

# Benchmarks
make benchmark           # run benchmarks (release build) and save results
make benchmark-data      # regenerate benchmark sample data files

# Pre-commit hooks (Rust fmt/clippy + Python ruff)
make pre-commit-install
```
