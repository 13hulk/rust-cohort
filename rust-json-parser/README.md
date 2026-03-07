# 🏗️ Project Internals

**Where the sausage gets made.** Architecture, directory layout, design patterns, and every `make` target you'll ever need.

| | |
|---|---|
| 🦀 Build | ![build](https://img.shields.io/badge/make_all-passing-brightgreen) |
| 🦀 Tests | ![rust tests](https://img.shields.io/badge/Rust_tests-177-brightgreen) |
| 🐍 Tests | ![python tests](https://img.shields.io/badge/Python_tests-15-brightgreen) |
| 🧹 Clippy | ![clippy](https://img.shields.io/badge/clippy-0_warnings-brightgreen) |
| 📖 Docs | ![doc coverage](https://img.shields.io/badge/doc_coverage-100%25-brightgreen) |

## 📑 Index

- [🏛️ Architecture](#-architecture) — pipeline overview, data flow
- [🎨 Design Patterns](#-design-patterns) — patterns used and why
- [📂 Project Structure](#-project-structure) — every file, annotated
- [🛠️ Make Commands](#-make-commands) — 🦀, 🐍, and benchmark targets
- [📖 Documentation](#-documentation) — doc generation, doc tests, browsing
- **Deeper dives:**
  - [🦀 Rust API Reference](src/README.md) — modules, types, code examples, optimizations
  - [🐍 Python API Reference](python/README.md) — functions, CLI, type mapping
  - [📊 Benchmark Results](benchmarks/results.md) — full methodology and raw numbers
- **[⬆ Back to project root](../README.md)**

## 🏛️ Architecture

Two-phase pipeline. No magic, no macros, no lifetimes. Just structs and enums doing honest work.

```
  Phase 1: Tokenization              Phase 2: Parsing
  ─────────────────────              ──────────────────
  &str → Tokenizer::tokenize()       Vec<Token> → JsonParser::parse()
       → Vec<Token>                            → JsonValue
```

| Phase | Input | Output | Strategy |
|-------|-------|--------|----------|
| **Tokenize** | `&str` (raw JSON) | `Vec<Token>` (10 variants) | Byte-by-byte scan, `O(n)` |
| **Parse** | `Vec<Token>` | `JsonValue` (6-variant enum) | Recursive descent, single pass |

**Why two phases?** Each is independently testable. Tokenizer bugs don't hide behind parser logic. Parser tests don't need raw JSON strings — just token vectors.

## 🎨 Design Patterns

| Pattern | Where | Why |
|---------|-------|-----|
| **Two-phase pipeline** | `Tokenizer` → `Parser` | Separation of concerns, independent testing |
| **Recursive descent** | `parser.rs` | Natural fit for JSON's recursive grammar |
| **Enum-based AST** | `JsonValue` (6 variants) | Type-safe, exhaustive pattern matching |
| **Enum-based errors** | `JsonError` (5 variants) | Positional diagnostics, no stringly-typed errors |
| **Trait-based formatting** | `JsonFormat` (private) | Per-type `Display` without giant match blocks |
| **Feature-gated FFI** | `#[cfg(feature = "python")]` | Clean 🦀-only builds, optional 🐍 linkage |
| **Buffer reuse** | `retokenize()`, `reparse()` | Amortized allocation in hot loops |
| **Capacity hinting** | `with_capacity()` everywhere | 12 allocation sites pre-sized with heuristics |
| **Byte-scan serialization** | `to_json_string()` | Bulk `push_str()` for unescaped segments |
| **Dual crate type** | `cdylib` + `rlib` | Single crate → 🐍 extension + 🦀 library |

## 📂 Project Structure

```
rust-json-parser/
│
├── src/                              # 🦀 Rust source
│   ├── lib.rs                        #    Crate root — module decls, #![warn(missing_docs)]
│   ├── main.rs                       #    Demo binary — showcases all features
│   ├── tokenizer.rs                  #    Phase 1 — JSON text → Vec<Token>
│   ├── parser.rs                     #    Phase 2 — Vec<Token> → JsonValue
│   ├── value.rs                      #    JsonValue enum, accessors, Display
│   ├── error.rs                      #    JsonError enum, positional diagnostics
│   ├── python_bindings.rs            #    PyO3 FFI — feature-gated behind "python"
│   └── README.md                     #    🦀 API reference
│
├── python/                           # 🐍 Python package
│   └── rust_json_parser/
│       ├── __init__.py               #    Exports: parse_json, dumps, benchmark_performance
│       ├── __main__.py               #    CLI entry point (--benchmark flag)
│       ├── _rust_json_parser.pyi     #    Type stubs for IDE autocomplete
│       └── README.md                 #    🐍 API reference
│
├── tests/
│   └── test_python_integration.py    #    15 pytest tests — parsing, errors, CLI, benchmarks
│
├── benchmarks/
│   ├── generate.py                   #    Deterministic data generator (no randomness)
│   ├── results.md                    #    Latest 3-way benchmark results
│   ├── small.json                    #    110 B — flat object, 6 fields
│   ├── medium.json                   #    10.1 KB — 75 nested objects
│   ├── large.json                    #    103.9 KB — 750 nested objects
│   ├── xlarge.json                   #    500.5 KB — 1230 objects, long strings
│   └── nested.json                   #    10.1 KB — 228 levels deep
│
├── Cargo.toml                        #    Crate config — PyO3 optional, edition 2024
├── pyproject.toml                    #    🐍 build — maturin backend, ruff config
└── Makefile                          #    All build/test/lint/bench commands
```

## 🛠️ Make Commands

All commands run from this directory (`rust-json-parser/`).

### 🦀 Rust

| Command | What it does |
|---------|-------------|
| `make all` | 🏆 The full gauntlet: fmt → clippy → test → build |
| `make test` | Run 177 tests (159 unit + 18 doc) |
| `make clippy` | Lint with `-D warnings` (zero tolerance) |
| `make fmt` | Format with `rustfmt` |
| `make build` | Build 🦀-only (no 🐍 linkage) |
| `make doc` | Generate API docs |
| `make doc-test` | Run 18 doc tests only |
| `make clean` | Remove build artifacts |

### 🐍 Python

| Command | What it does |
|---------|-------------|
| `make python-all` | 🏆 Build → fmt → lint → typecheck → test |
| `make python-build` | Build extension with `maturin develop` |
| `make python-test` | Run 15 pytest integration tests |
| `make python-fmt` | Format with ruff |
| `make python-lint` | Lint with ruff |
| `make python-typecheck` | Type check with ty |
| `make python-run` | Run CLI demo |

### ⏱️ Benchmarks

| Command | What it does |
|---------|-------------|
| `make benchmark` | Release build + 3-way benchmark, saves to `results.md` |
| `make rust-benchmark` | 🦀-only benchmark (no 🐍) |
| `make benchmark-data` | Regenerate deterministic sample data |

### ⚙️ Setup

| Command | What it does |
|---------|-------------|
| `make pre-commit-install` | Install pre-commit hooks (fmt + clippy + ruff) |

> 💡 All 🦀 targets use `--no-default-features` to skip PyO3 linkage. 🐍 targets use [uv](https://docs.astral.sh/uv/).

## 📖 Documentation

Every public item has a doc comment. No exceptions — `#![warn(missing_docs)]` enforces it.

```bash
# Generate and browse docs in your browser
make doc
cargo doc --no-default-features --open

# Run the 18 doc tests (they compile, run, and assert)
make doc-test
```

Doc tests double as integration tests — if the docs lie, the build catches it. 🎯
