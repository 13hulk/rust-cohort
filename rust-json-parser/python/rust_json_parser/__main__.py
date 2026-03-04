"""CLI entry point for rust_json_parser.

Usage:
    python -m rust_json_parser '{"key": "value"}'
    python -m rust_json_parser data.json
    python -m rust_json_parser --benchmark
    echo '{"test": 123}' | python -m rust_json_parser
"""

import os.path
import platform
import sys
from datetime import UTC, datetime
from pathlib import Path

from rust_json_parser import dumps, parse_json, parse_json_file

# Benchmark iteration counts applied to every input file.
ITERATION_COUNTS = [100, 1000]

# Descriptions shown next to each file name in benchmark output.
FILE_DESCRIPTIONS = {
    "small.json": "Single flat object with 6 fields",
    "medium.json": "Array of 75 objects with nested address",
    "large.json": "Array of 750 objects with nested address",
    "xlarge.json": "1230 objects with long strings and nested metadata",
    "nested.json": "Deeply nested objects and arrays (228 levels)",
}


def _find_project_root():
    """Find the rust-json-parser project root directory.

    Walks up from the package location looking for the Cargo.toml marker.
    """
    current = Path(__file__).resolve().parent
    for _ in range(10):
        if (current / "Cargo.toml").exists():
            return current
        current = current.parent
    return None


def _load_benchmark_inputs():
    """Load benchmark JSON inputs from the 5 sample files.

    Benchmark scenarios (5 files x 2 iteration counts = 10 combinations):

      small.json   Single flat object with 6 fields
      medium.json  Array of 75 objects with nested address
      large.json   Array of 750 objects with nested address
      xlarge.json  1230 objects with long strings and nested metadata
      nested.json  Deeply nested objects and arrays (228 levels)

    Each file is benchmarked at 100 and 1000 iterations.

    The sample files in benchmarks/ are deterministic (no randomness) and
    committed to the repository for reproducible results. Regenerate them
    with ``make benchmark-data`` if needed.
    """
    project_root = _find_project_root()
    benchmarks_dir = project_root / "benchmarks" if project_root else None

    filenames = [
        "small.json",
        "medium.json",
        "large.json",
        "xlarge.json",
        "nested.json",
    ]
    inputs = []

    for filename in filenames:
        filepath = benchmarks_dir / filename if benchmarks_dir else None
        if filepath and filepath.exists():
            json_str = filepath.read_text()
            inputs.append((filename, json_str))
        else:
            print(
                f"Error: {filename} not found. Run 'make benchmark-data' first.",
                file=sys.stderr,
            )
            sys.exit(1)

    return inputs


def _format_size(num_bytes):
    """Format byte count as human-readable string."""
    if num_bytes < 1024:
        return f"{num_bytes} bytes"
    return f"{num_bytes / 1024:.1f} KB"


def _format_comparison(label, time_secs, rust_time):
    """Format a benchmark comparison line with speedup ratio."""
    ratio = time_secs / rust_time if rust_time > 0 else 0.0
    descriptor = "faster" if ratio >= 1.0 else "slower"
    return f"    {label:20s} {time_secs:.6f}s  (Rust is {ratio:.2f}x {descriptor})"


def _format_ratio(time_secs, rust_time):
    """Format a speedup ratio for the results table."""
    if rust_time <= 0:
        return "N/A"
    ratio = time_secs / rust_time
    if ratio >= 1.0:
        return f"{ratio:.2f}x faster"
    return f"{1.0 / ratio:.2f}x slower"


def _format_iterations(n):
    """Format an iteration count with comma separators."""
    return f"{n:,}"


def _save_results(all_results, results_path):
    """Save benchmark results with per-file table and summary sections.

    Writes a Markdown file with metadata, a results table for all file/
    iteration combinations, per-iteration totals, and overall totals.
    """
    now = datetime.now(UTC).strftime("%Y-%m-%d %H:%M:%S")
    platform_info = platform.platform()
    python_version = platform.python_version()

    lines = [
        "# Benchmark Results",
        "",
        f"**Date:** {now}  ",
        f"**Platform:** {platform_info}  ",
        f"**Python:** {python_version}  ",
        "**Build:** release  ",
        "",
        "## Benchmark Plan",
        "",
        "Three-way comparison of JSON parsing performance:",
        "",
        "- **Rust** -- our `rust_json_parser` (PyO3 extension, release build)",
        "- **Python json** -- CPython's built-in `json.loads()` (C implementation)",
        "- **simplejson** -- pure-Python `simplejson.loads()`",
        "",
        "### Input Files",
        "",
        "| File | Description | Size |",
        "|------|-------------|------|",
    ]

    seen_files = []
    for label, size_str, _iters, _results in all_results:
        if label not in seen_files:
            seen_files.append(label)
            desc = FILE_DESCRIPTIONS.get(label, "")
            lines.append(f"| {label} | {desc} | {size_str} |")

    lines += [
        "",
        "### Method",
        "",
        "Each file is parsed repeatedly at two iteration counts "
        f"({', '.join(str(n) for n in ITERATION_COUNTS)}) "
        "to measure both startup overhead and sustained throughput. "
        "All timing uses `std::time::Instant` in Rust, "
        "including Python parser calls via PyO3 FFI. "
        'The "Rust vs" columns show how many times faster or slower '
        "our parser is compared to each baseline.",
        "",
        "## Results",
        "",
        "| File | Iterations | Rust (s) | Python json (s) | simplejson (s) "
        "| Rust vs json | Rust vs simplejson |",
        "|------|------------|----------|-----------------|----------------"
        "|--------------|---------------------|",
    ]

    for label, _size_str, iterations, (rust_t, json_t, sjson_t) in all_results:
        rust_vs_json = _format_ratio(json_t, rust_t)
        rust_vs_sjson = _format_ratio(sjson_t, rust_t)
        lines.append(
            f"| {label} | {_format_iterations(iterations)} "
            f"| {rust_t:.6f} | {json_t:.6f} "
            f"| {sjson_t:.6f} | {rust_vs_json} | {rust_vs_sjson} |"
        )

    # --- Per-Iteration Totals ---
    iter_totals = {}
    for _label, _size, iterations, (rust_t, json_t, sjson_t) in all_results:
        if iterations not in iter_totals:
            iter_totals[iterations] = [0.0, 0.0, 0.0]
        iter_totals[iterations][0] += rust_t
        iter_totals[iterations][1] += json_t
        iter_totals[iterations][2] += sjson_t

    lines += [
        "",
        "## Summary",
        "",
        "### Per-Iteration Totals",
        "",
        "| Iterations | Rust Total (s) | Python json Total (s) "
        "| simplejson Total (s) |",
        "|------------|----------------|----------------------|----------------------|",
    ]

    for iters in ITERATION_COUNTS:
        if iters in iter_totals:
            r, j, s = iter_totals[iters]
            lines.append(
                f"| {_format_iterations(iters)} | {r:.6f} | {j:.6f} | {s:.6f} |"
            )

    # --- Overall Totals ---
    total_rust = sum(t[0] for _, _, _, t in all_results)
    total_json = sum(t[1] for _, _, _, t in all_results)
    total_sjson = sum(t[2] for _, _, _, t in all_results)
    total_all = total_rust + total_json + total_sjson

    lines += [
        "",
        "### Overall Totals",
        "",
        "| Parser | Total Time (s) |",
        "|--------|----------------|",
        f"| Rust | {total_rust:.6f} |",
        f"| Python json | {total_json:.6f} |",
        f"| simplejson | {total_sjson:.6f} |",
        f"| **Total benchmark time** | **{total_all:.6f}** |",
        "",
    ]

    results_path.write_text("\n".join(lines))
    print(f"\nResults saved to {results_path}")


def run_benchmark():
    """Run performance benchmarks comparing Rust, Python json, and simplejson.

    Loads 5 JSON input files (small, medium, large, xlarge, nested), runs each
    at 2 iteration counts (100, 1000), and prints results with speedup ratios.
    Results are saved to benchmarks/results.md with summary sections.
    """
    from rust_json_parser import benchmark_performance

    test_cases = _load_benchmark_inputs()

    all_results = []

    print("=== JSON Parser Benchmark ===", flush=True)
    print("Build: release", flush=True)
    print(flush=True)

    for label, json_str in test_cases:
        size_bytes = len(json_str.encode("utf-8"))
        size_str = _format_size(size_bytes)
        desc = FILE_DESCRIPTIONS.get(label, "")

        print(f"--- {label} -- {desc} -- {size_str} ---", flush=True)
        print(flush=True)

        for iterations in ITERATION_COUNTS:
            rust_time, python_json_time, simplejson_time = benchmark_performance(
                json_str, iterations=iterations
            )

            print(f"  {_format_iterations(iterations)} iterations:", flush=True)
            print(f"    {'Rust:':20s} {rust_time:.6f}s", flush=True)
            print(
                _format_comparison("Python json (C):", python_json_time, rust_time),
                flush=True,
            )
            print(
                _format_comparison("simplejson:", simplejson_time, rust_time),
                flush=True,
            )
            print(flush=True)

            all_results.append(
                (
                    label,
                    size_str,
                    iterations,
                    (rust_time, python_json_time, simplejson_time),
                )
            )

    # Always save results
    project_root = _find_project_root()
    if project_root:
        results_path = project_root / "benchmarks" / "results.md"
        results_path.parent.mkdir(parents=True, exist_ok=True)
        _save_results(all_results, results_path)
    else:
        print(
            "Warning: could not find project root, skipping results save",
            file=sys.stderr,
        )


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
