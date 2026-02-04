# Makefile for Rust Projects

A simple Makefile to wrap common Cargo commands with easy-to-remember shortcuts.

## Why Use a Makefile?

- Shorter commands: `make test` vs `cargo test`
- Consistent flags: `make clippy` always runs with `-D warnings`
- Discoverability: `make help` shows all available commands
- Combines steps: `make all` runs fmt, clippy, test, build in one command

## The Makefile

```makefile
.PHONY: help build test run fmt clippy check clean pre-commit-install all

help:
	@echo "Available commands:"
	@echo "  make build              - Build the project"
	@echo "  make test               - Run tests"
	@echo "  make run                - Run the binary"
	@echo "  make fmt                - Format code"
	@echo "  make fmt-check          - Check formatting"
	@echo "  make clippy             - Run linter"
	@echo "  make check              - Quick compile check"
	@echo "  make clean              - Remove build artifacts"
	@echo "  make pre-commit-install - Install pre-commit hooks"
	@echo "  make all                - Run fmt, clippy, test, build"

build:
	cargo build

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy -- -D warnings

check:
	cargo check

clean:
	cargo clean

pre-commit-install:
	pre-commit install

all: fmt clippy test build
```

## Usage

```bash
# See all available commands
make help

# Common workflow
make fmt        # Format code
make clippy     # Check for issues
make test       # Run tests
make build      # Build project

# Or do it all at once
make all        # Runs: fmt → clippy → test → build

# Other useful commands
make run        # Run the binary
make check      # Quick compile check (faster than build)
make clean      # Remove target/ directory
```

## Setup

1. Create a file named `Makefile` in your project root (no extension)
2. Copy the content above
3. Run `make help` to verify it works

## Notes

- `.PHONY` tells Make these are commands, not files
- `@` prefix hides the command itself, only shows output
- `make all` runs targets in order: fmt, clippy, test, build
- Clippy runs with `-D warnings` to treat warnings as errors
