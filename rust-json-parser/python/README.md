# rust-json-parser - Python Bindings

Python bindings for the Rust JSON parser via PyO3. Parses JSON using Rust and returns native Python types.

## Installation

Requires a Rust toolchain and [uv](https://docs.astral.sh/uv/):

```bash
uv run maturin develop
# or
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
```

Output is pretty-printed with 2-space indentation.

## Development

```bash
make python-build      # build with maturin
make python-test       # run 11 pytest tests
make python-fmt        # format with ruff
make python-lint       # lint with ruff
make python-typecheck  # type check with ty
make python-run        # run CLI demo
make python-all        # all of the above
```
