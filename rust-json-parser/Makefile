.PHONY: help build test run fmt clippy check clean doc pre-commit-install all python-build python-test python-all

help:
	@echo "Available commands:"
	@echo "  make build              - Build the project (Rust only)"
	@echo "  make test               - Run Rust tests (no Python linkage)"
	@echo "  make run                - Run the binary"
	@echo "  make fmt                - Format code"
	@echo "  make fmt-check          - Check formatting"
	@echo "  make clippy             - Run linter (no Python linkage)"
	@echo "  make check              - Quick compile check"
	@echo "  make clean              - Remove build artifacts"
	@echo "  make doc                - Generate and open docs"
	@echo "  make pre-commit-install - Install pre-commit hooks"
	@echo "  make all                - Run fmt, clippy, test, build"
	@echo "  make python-build       - Build Python package with maturin"
	@echo "  make python-test        - Run Python integration tests"
	@echo "  make python-all         - Build and test Python package"

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
	. .venv/bin/activate && maturin develop

python-test:
	. .venv/bin/activate && pytest tests/test_python_integration.py -v

python-all: python-build python-test

all: fmt clippy test build
