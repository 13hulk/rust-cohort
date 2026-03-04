"""
Benchmark data generator for rust-json-parser.

Generates 5 deterministic JSON files for reproducible benchmarking.
No randomness -- same output every run for consistent baselines.

| File        | Description                                           | ~Size   |
|-------------|-------------------------------------------------------|---------|
| small.json  | Single flat object with 6 fields                      | ~100 B  |
| medium.json | Array of 75 objects with nested address               | ~10 KB  |
| large.json  | Array of 750 objects with nested address              | ~100 KB |
| xlarge.json | 1230 objects with long strings and nested metadata    | ~500 KB |
| nested.json | 228 levels of deeply nested objects                   | ~10 KB  |

Usage:
    python benchmarks/generate.py
    make benchmark-data
"""

import json
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))


def generate_small():
    """Generate a single flat JSON object (~100 bytes).

    A small object with 6 fields of different types: string, number,
    email, boolean, float, and city. Tests baseline parsing overhead
    for small realistic JSON input.
    """
    return {
        "name": "Alice",
        "age": 30,
        "email": "alice@example.com",
        "active": True,
        "score": 95.5,
        "city": "Portland",
    }


def _make_user(i):
    """Build a user object for medium/large benchmarks."""
    return {
        "id": i,
        "name": f"user_{i}",
        "email": f"user_{i}@example.com",
        "age": 25 + (i % 50),
        "active": i % 2 == 0,
        "address": {
            "city": f"City_{i}",
            "zip": str(10000 + i),
        },
    }


def generate_medium():
    """Generate an array of 75 user objects with nested address fields (~10 KB).

    Each object has 6 top-level fields (id, name, email, age, active) plus
    a nested address object with city and zip. This size is representative
    of typical API responses.
    """
    return [_make_user(i) for i in range(75)]


def generate_large():
    """Generate an array of 750 user objects with nested address fields (~100 KB).

    Same structure as medium.json but scaled up. Tests parser performance
    on large input where parsing dominates overhead.
    """
    return [_make_user(i) for i in range(750)]


def generate_xlarge():
    """Generate 1230 objects with long strings and nested data (~500 KB).

    Each object has: deterministic UUID string, long description (~100 chars),
    large precise number, tags array, nested metadata object with
    dates/versions/flags. Tests parsing performance on field-heavy,
    value-heavy JSON with diverse types.
    """
    items = []
    for i in range(1230):
        items.append(
            {
                "id": i,
                "uuid": f"550e8400-e29b-41d4-a716-{i:012d}",
                "name": f"entity_{i}",
                "description": (
                    f"This is the description for entity number {i} "
                    "which contains enough text to test string parsing performance"
                ),
                "score": 10000 + i + i * 0.123456,
                "rating": 1.0 + (i % 5),
                "tags": [f"tag_{i * 3}", f"tag_{i * 3 + 1}", f"tag_{i * 3 + 2}"],
                "metadata": {
                    "created": "2024-01-01",
                    "updated": "2024-06-15",
                    "version": 1,
                    "flags": [True, False, True],
                },
                "active": i % 2 == 0,
            }
        )
    return items


def generate_nested_string():
    """Generate a deeply nested JSON string with 228 levels (~10 KB).

    Builds a JSON string where each level contains a 'level' number, a 'data'
    string, and a 'child' object that nests one level deeper. The innermost
    level has "child": null. Tests the parser's recursive descent performance
    and stack behavior on deeply nested input.

    Depth is set to 228 -- deep enough to stress-test recursion but within
    simplejson's pure-Python recursion limit (~498 levels).

    Returns a JSON string directly (not a Python dict) to avoid Python's
    recursion limit when serializing deeply nested structures.
    """
    depth = 228
    parts = []
    for level in range(depth):
        parts.append(f'{{"level": {level}, "data": "value_{level}", "child": ')
    # Innermost level has child: null
    parts.append(f'{{"level": {depth}, "data": "value_{depth}", "child": null}}')
    # Close all the outer braces
    parts.append("}" * depth)
    return "".join(parts)


def write_json(data, filename):
    """Write data to a JSON file and print its size."""
    path = os.path.join(SCRIPT_DIR, filename)
    with open(path, "w") as f:
        json.dump(data, f)
        f.write("\n")
    size = os.path.getsize(path)
    size_str = f"{size} bytes" if size < 1024 else f"{size / 1024:.1f} KB"
    print(f"Generated {filename:15s} ({size_str})")
    return path


def write_json_string(json_str, filename):
    """Write a pre-built JSON string to a file and print its size."""
    path = os.path.join(SCRIPT_DIR, filename)
    with open(path, "w") as f:
        f.write(json_str)
        f.write("\n")
    size = os.path.getsize(path)
    size_str = f"{size} bytes" if size < 1024 else f"{size / 1024:.1f} KB"
    print(f"Generated {filename:15s} ({size_str})")
    return path


def main():
    print("Generating benchmark data files...")
    print()

    write_json(generate_small(), "small.json")
    write_json(generate_medium(), "medium.json")
    write_json(generate_large(), "large.json")
    write_json(generate_xlarge(), "xlarge.json")
    # nested.json is generated as a string to avoid Python's recursion limit
    # when serializing 228 levels of nesting via json.dump.
    write_json_string(generate_nested_string(), "nested.json")

    print()
    print("Done. Regenerate with: make benchmark-data")


if __name__ == "__main__":
    main()
