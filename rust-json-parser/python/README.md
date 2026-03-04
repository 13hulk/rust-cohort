# rust-json-parser - Python Bindings

Python bindings for the Rust JSON parser via PyO3. Parses JSON using Rust and returns native Python types. Includes a benchmarking function for comparing parse performance against Python's `json` and `simplejson`.

## Installation

Requires a Rust toolchain and [uv](https://docs.astral.sh/uv/):

```bash
make python-build
```

## API

### `parse_json(input: str) -> dict | list | str | float | bool | None`

Parse a JSON string into native Python types.

```python
from rust_json_parser import parse_json

data = parse_json('{"name": "Alice", "scores": [95, 87]}')
data["name"]       # "Alice"
data["scores"][0]  # 95.0
```

Raises `ValueError` for invalid JSON (error message includes position).

### `parse_json_file(path: str) -> dict | list | str | float | bool | None`

Parse a JSON file by path.

```python
from rust_json_parser import parse_json_file

data = parse_json_file("data.json")
```

Raises `IOError` for file not found, `ValueError` for invalid JSON.

### `dumps(obj, indent=None) -> str`

Serialize a Python object to a JSON string.

```python
from rust_json_parser import dumps

dumps({"key": "value"})              # '{"key":"value"}'
dumps({"key": "value"}, indent=2)    # pretty-printed with 2-space indent
```

Object keys are sorted alphabetically in indented output.

### `benchmark_performance(json_str: str, iterations: int = 1000) -> tuple[float, float, float]`

Benchmark JSON parsing speed across three parsers: Rust, Python `json`, and `simplejson`. Each parser runs a 100-iteration warmup before the timed run.

Returns a tuple of `(rust_time, python_json_time, simplejson_time)` in seconds.

```python
from rust_json_parser import benchmark_performance

rust_t, json_t, sjson_t = benchmark_performance('{"key": "value"}', iterations=5000)
print(f"Rust: {rust_t:.4f}s, json: {json_t:.4f}s, simplejson: {sjson_t:.4f}s")
```

Requires `simplejson` to be installed (`uv pip install simplejson`).

## Type Mapping

| JSON    | Python | Notes                         |
|---------|--------|-------------------------------|
| object  | dict   |                               |
| array   | list   |                               |
| string  | str    |                               |
| number  | float  | All numbers become float      |
| boolean | bool   |                               |
| null    | None   |                               |

## CLI

```bash
# Parse a JSON string
python -m rust_json_parser '{"key": "value"}'

# Parse a file
python -m rust_json_parser data.json

# Read from stdin
echo '{"test": 123}' | python -m rust_json_parser

# Run benchmarks (results saved to benchmarks/results.md)
python -m rust_json_parser --benchmark
```

Output is pretty-printed with indentation.

## Benchmarking

The `--benchmark` flag runs a 3-way comparison across small, medium, and large JSON inputs:

```bash
python -m rust_json_parser --benchmark
```

### Scenarios

Benchmark data files are deterministic (no randomness) and committed to the repository for reproducible results. Regenerate them with `make benchmark-data`.

| File | Size | Description | Iterations |
|------|------|-------------|------------|
| small.json | ~110 bytes | Single flat object with 6 fields | 100, 1000 |
| medium.json | ~10 KB | Array of 75 objects with nested address | 100, 1000 |
| large.json | ~104 KB | Array of 750 objects with nested address | 100, 1000 |
| xlarge.json | ~500 KB | 1230 objects with long strings and nested metadata | 100, 1000 |
| nested.json | ~10 KB | Deeply nested objects and arrays (228 levels) | 100, 1000 |

This parses each input file multiple times and prints timing results with speedup ratios. Results are automatically saved to `benchmarks/results.md`.

You can also run benchmarks from the project root:

```bash
make benchmark        # builds release, runs benchmark, saves results
make benchmark-data   # regenerate benchmark sample data files
```

## Development

```bash
make python-build      # build with maturin
make python-test       # run 15 pytest tests
make python-fmt        # format with ruff
make python-lint       # lint with ruff
make python-typecheck  # type check with ty
make python-run        # run CLI demo
make python-all        # all of the above
```
