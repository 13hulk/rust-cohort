# 🐍 Python API Reference

**🦀 speed, 🐍 vibes.** Parse JSON with 🦀 under the hood, get back native Python types. No serialization gymnastics.

| | |
|---|---|
| 🐍 Python | ![Python](https://img.shields.io/badge/Python-≥3.12-3776AB?logo=python&logoColor=white) |
| 🔗 FFI | ![PyO3](https://img.shields.io/badge/PyO3-0.25-blue) |
| ✅ Tests | ![tests](https://img.shields.io/badge/pytest-15_passing-brightgreen) |

## 📑 Index

- [📦 Installation](#-installation) — one command
- [🔌 API](#-api) — `parse_json`, `parse_json_file`, `dumps`, `benchmark_performance`
- [🔀 Type Mapping](#-type-mapping) — JSON ↔ 🐍 conversion table
- [💻 CLI](#-cli) — parse strings, files, stdin, run benchmarks
- **[⬆ Project Internals](../README.md)** · **[🦀 Rust API](../src/README.md)**

---

## 📦 Installation

```bash
make python-build   # builds the 🦀 extension with maturin
```

> Requires [🦀 toolchain](https://rustup.rs/) and [uv](https://docs.astral.sh/uv/).

---

## 🔌 API

### `parse_json(input: str) → dict | list | str | float | bool | None`

The main event. Parses a JSON string, returns native 🐍 types.

```python
from rust_json_parser import parse_json

data = parse_json('{"name": "Alice", "scores": [95, 87]}')
data["name"]       # "Alice"
data["scores"][0]  # 95.0
```

Raises `ValueError` with position info on invalid JSON.

### `parse_json_file(path: str) → dict | list | str | float | bool | None`

Same thing, but reads from a file path.

```python
from rust_json_parser import parse_json_file

data = parse_json_file("data.json")
```

Raises `IOError` for missing files, `ValueError` for bad JSON.

### `dumps(obj, indent=None) → str`

Serialize 🐍 objects to JSON strings. The reverse trip. 🔄

```python
from rust_json_parser import dumps

dumps({"key": "value"})              # '{"key":"value"}'
dumps({"key": "value"}, indent=2)    # pretty-printed, keys sorted
```

### `benchmark_performance(json_str, iterations=1000) → tuple[float, float, float]`

Three-parser shootout. Returns `(rust_time, json_time, simplejson_time)` in seconds.

```python
from rust_json_parser import benchmark_performance

rust_t, json_t, sjson_t = benchmark_performance('{"key": "value"}', iterations=5000)
```

Each parser gets a 100-iteration warmup. Requires `simplejson`.

---

## 🔀 Type Mapping

| JSON | 🐍 Python | Notes |
|------|-----------|-------|
| `{}` | `dict` | |
| `[]` | `list` | |
| `"..."` | `str` | |
| `123` | `float` | All numbers → float (JSON has no int) |
| `true`/`false` | `bool` | |
| `null` | `None` | |

---

## 💻 CLI

```bash
# Parse a string
python -m rust_json_parser '{"key": "value"}'

# Parse a file
python -m rust_json_parser data.json

# Pipe from stdin
echo '{"test": 123}' | python -m rust_json_parser

# Run the 3-way benchmark
python -m rust_json_parser --benchmark
```

Output is pretty-printed. Benchmark results are saved to `benchmarks/results.md`. 📊
