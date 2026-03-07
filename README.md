# 🦀 Rust JSON Parser

**A JSON parser that parses faster than you can say "simplejson".**

Pure 🦀. Zero dependencies. Two-phase pipeline. Python bindings via PyO3. Built from scratch — no `serde` in sight.

| | |
|---|---|
| 🦀 Language | ![Rust](https://img.shields.io/badge/Rust-1.93.0-orange?logo=rust&logoColor=white) |
| 🐍 Python | ![Python](https://img.shields.io/badge/Python-≥3.12-3776AB?logo=python&logoColor=white) |
| 🔗 FFI | ![PyO3](https://img.shields.io/badge/PyO3-0.25-blue) |
| 📦 Build | ![maturin](https://img.shields.io/badge/maturin-≥1.0-blue) |
| ✅ Tests | ![Tests](https://img.shields.io/badge/tests-192%20passing-brightgreen?logo=checkmarx&logoColor=white) |
| 🧹 Lint | ![Clippy](https://img.shields.io/badge/clippy-0%20warnings-brightgreen) |
| 📖 Docs | ![Docs](https://img.shields.io/badge/docs-100%25%20coverage-brightgreen) |
| 📐 Spec | ![RFC 8259](https://img.shields.io/badge/RFC-8259-green) |
| ⚖️ License | ![License](https://img.shields.io/badge/license-MIT-blue) |

## 📑 Index

- **[Project Internals](rust-json-parser/README.md)** — structure, architecture, design patterns, build commands
  - **[🦀 Rust API Reference](rust-json-parser/src/README.md)** — modules, types, code examples, optimization notes
  - **[🐍 Python API Reference](rust-json-parser/python/README.md)** — Python functions, CLI, type mapping
- **[📊 Benchmark Data](rust-json-parser/benchmarks/results.md)** — full methodology and raw numbers

## ⚡ The Numbers

Three-way showdown: **🦀 Rust** vs **Python (CPython C)** (15 years of battle-hardened C) vs **simplejson** (pure Python).

> 📊 macOS 15.7.2 arm64 · Python 3.14.3 · `--release` build · 100-iteration warmup · [full results](rust-json-parser/benchmarks/results.md)

| Input | Size | vs Python (CPython C) | vs simplejson |
|-------|------|:---------------------:|:-------------:|
| small.json | 110 B | 🟢 **2.0x faster** | 🟢 **14.3x faster** |
| medium.json | 10 KB | 🟡 0.83x | 🟢 **13.3x faster** |
| large.json | 104 KB | 🟡 0.88x | 🟢 **13.5x faster** |
| xlarge.json | 501 KB | 🟡 0.81x | 🟢 **12.3x faster** |
| nested.json | 10 KB | 🟡 0.78x | 🟢 **12.9x faster** |

**TL;DR:**
- 🔥 **12–14x faster** than simplejson. Every time. No exceptions.
- 🏎️ Within **80% of CPython's C extension** — a C codebase with 15+ years of hand-tuned optimization. Not bad for a from-scratch 🦀 parser with zero `unsafe`.
- 🚀 **2x faster than C** on small inputs where 🦀's zero-overhead abstractions shine before FFI costs kick in.

## 🔧 Tech Stack

| Layer | Tool | Notes |
|-------|------|-------|
| 🦀 Language | Rust 1.93, Edition 2024 | Zero external crates |
| 🔗 Python FFI | PyO3 0.25 + maturin | Feature-gated `cdylib` + `rlib` |
| 🐍 Python | ≥ 3.12 | Tested on 3.14.3 |
| 🧹 Linting | clippy (`-D warnings`) + ruff | Pre-commit enforced |
| 🎨 Formatting | `cargo fmt` + `ruff format` | Pre-commit enforced |
| 🔍 Type checking | `ty` | Python stubs included |
| ✅ Testing | `cargo test` + pytest | 177 🦀 + 15 🐍 |
| ⏱️ Benchmarking | `std::time::Instant` | Deterministic inputs, reproducible |
| 📐 Spec | [RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259) | Full compliance |

## 🚀 Quick Start

```bash
cd rust-json-parser

make all            # fmt → clippy → test → build (🦀)
make python-all     # build → fmt → lint → typecheck → test (🐍)
make benchmark      # release build + 3-way benchmark
```

> Requires [Rust toolchain](https://rustup.rs/) and [uv](https://docs.astral.sh/uv/).

## 🏗️ How It Works

```
  "{"key": [1, true]}"
          │
    ┌─────▼──────┐
    │  Tokenizer  │  byte-by-byte scan
    └─────┬──────┘
          │
  [LeftBrace, String("key"), Colon, LeftBracket, ...]
          │
    ┌─────▼──────┐
    │   Parser    │  recursive descent
    └─────┬──────┘
          │
  Object({"key": Array([Number(1.0), Boolean(true)])})
```

Two phases, no drama. Tokenize first, parse second. Each phase is independently testable and reusable.

See [Project Internals](rust-json-parser/README.md) for architecture details and design patterns.
