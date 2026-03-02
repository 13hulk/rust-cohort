"""Rust JSON parser with Python bindings via PyO3."""

from rust_json_parser._rust_json_parser import parse_json, parse_json_file, dumps

__all__ = ["parse_json", "parse_json_file", "dumps"]
