"""CLI entry point for rust_json_parser.

Usage:
    python -m rust_json_parser '{"key": "value"}'
    python -m rust_json_parser data.json
    python -m rust_json_parser --benchmark
    echo '{"test": 123}' | python -m rust_json_parser
"""

import json
import os.path
import sys

from rust_json_parser import dumps, parse_json, parse_json_file


def _format_comparison(label, time_secs, rust_time):
    """Format a benchmark comparison line with speedup ratio."""
    ratio = time_secs / rust_time if rust_time > 0 else 0.0
    descriptor = "faster" if ratio >= 1.0 else "slower"
    return f"  {label:20s} {time_secs:.6f}s  (Rust is {ratio:.2f}x {descriptor})"


def run_benchmark():
    """Run performance benchmarks comparing Rust, Python json, and simplejson."""
    from rust_json_parser import benchmark_performance

    # Small JSON (~30 bytes)
    small_json = '{"key": "value"}'

    # Medium JSON (~500-1000 bytes): object with many key-value pairs
    medium_data = {f"key_{i}": f"value_{i}" for i in range(50)}
    medium_json = json.dumps(medium_data)

    # Large JSON (~10000+ bytes): large array of objects
    large_data = [
        {"id": i, "name": f"item_{i}", "active": i % 2 == 0} for i in range(200)
    ]
    large_json = json.dumps(large_data)

    test_cases = [
        ("Small JSON", small_json, 1000),
        ("Medium JSON", medium_json, 1000),
        ("Large JSON", large_json, 100),
    ]

    print("=== JSON Parser Benchmark ===")
    print()

    for label, json_str, iterations in test_cases:
        rust_time, python_json_time, simplejson_time = benchmark_performance(
            json_str, iterations=iterations
        )

        print(f"{label} ({len(json_str)} bytes):")
        print(f"  {'Rust:':20s} {rust_time:.6f}s")
        print(_format_comparison("Python json (C):", python_json_time, rust_time))
        print(_format_comparison("simplejson:", simplejson_time, rust_time))
        print()


def main():
    if "--benchmark" in sys.argv:
        run_benchmark()
        return

    if len(sys.argv) < 2:
        # No argument provided, try reading from stdin
        data = sys.stdin.read()
        if not data.strip():
            print(
                "Usage: python -m rust_json_parser <json_string_or_file>",
                file=sys.stderr,
            )
            sys.exit(1)
        try:
            result = parse_json(data)
            print(dumps(result, indent=2))
        except ValueError as e:
            print(f"Parse error: {e}", file=sys.stderr)
            sys.exit(1)
        return

    arg = sys.argv[1]

    try:
        result = parse_json_file(arg) if os.path.exists(arg) else parse_json(arg)
        print(dumps(result, indent=4))
    except ValueError as e:
        print(f"Parse error: {e}", file=sys.stderr)
        sys.exit(1)
    except OSError as e:
        print(f"File error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
