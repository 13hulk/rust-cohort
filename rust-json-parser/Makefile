.PHONY: help build test run fmt fmt-check clippy check clean doc pre-commit-install all
.PHONY: python-build python-test python-fmt python-fmt-check python-lint python-lint-fix python-typecheck python-run python-all

help:
	@echo "Rust commands:"
	@echo "  make build              - Build the project (Rust only)"
	@echo "  make test               - Run Rust tests (no Python linkage)"
	@echo "  make run                - Run the Rust demo binary"
	@echo "  make fmt                - Format Rust code"
	@echo "  make fmt-check          - Check Rust formatting"
	@echo "  make clippy             - Run Rust linter (warnings as errors)"
	@echo "  make check              - Quick compile check"
	@echo "  make clean              - Remove build artifacts"
	@echo "  make doc                - Generate and open docs"
	@echo "  make all                - Run fmt, clippy, test, build"
	@echo ""
	@echo "Python commands:"
	@echo "  make python-build       - Build Python package with maturin"
	@echo "  make python-test        - Run Python integration tests"
	@echo "  make python-fmt         - Format Python code with ruff"
	@echo "  make python-fmt-check   - Check Python formatting"
	@echo "  make python-lint        - Lint Python code with ruff"
	@echo "  make python-lint-fix    - Lint and auto-fix Python code"
	@echo "  make python-typecheck   - Type check Python code with ty"
	@echo "  make python-run         - Run the Python CLI demo"
	@echo "  make python-all         - Build, format, lint, typecheck, test"
	@echo ""
	@echo "Other:"
	@echo "  make pre-commit-install - Install pre-commit hooks"

build:
	cargo build --no-default-features

test:
	cargo test --no-default-features

run:
	cargo run --no-default-features

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy --no-default-features -- -D warnings

check:
	cargo check --no-default-features

clean:
	cargo clean

doc:
	cargo doc --open

pre-commit-install:
	pre-commit install

python-build:
	uv run maturin develop

python-test:
	uv run pytest tests/test_python_integration.py -v

python-fmt:
	uv run ruff format python/ tests/

python-fmt-check:
	uv run ruff format --check python/ tests/

python-lint:
	uv run ruff check python/ tests/

python-lint-fix:
	uv run ruff check --fix python/ tests/

python-typecheck:
	uv run ty check python/ tests/

python-run:
	uv run python -m rust_json_parser '{"name": "Alice", "age": 30, "tags": ["rust", "python"], "active": true}'

python-all: python-build python-fmt python-lint python-typecheck python-test

all: fmt clippy test build
