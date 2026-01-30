.PHONY: help build test run fmt clippy check clean doc pre-commit-install all

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
	@echo "  make doc                - Generate and open docs"
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

doc:
	cargo doc --open

pre-commit-install:
	pre-commit install

all: fmt clippy test build
