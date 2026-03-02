"""CLI entry point for rust_json_parser.

Usage:
    python -m rust_json_parser '{"key": "value"}'
    python -m rust_json_parser data.json
    echo '{"test": 123}' | python -m rust_json_parser
"""

import sys
import os.path

from rust_json_parser import parse_json, parse_json_file, dumps


def main():
    if len(sys.argv) < 2:
        # No argument provided, try reading from stdin
        data = sys.stdin.read()
        if not data.strip():
            print("Usage: python -m rust_json_parser <json_string_or_file>", file=sys.stderr)
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
        if os.path.exists(arg):
            result = parse_json_file(arg)
        else:
            result = parse_json(arg)
        print(dumps(result, indent=2))
    except ValueError as e:
        print(f"Parse error: {e}", file=sys.stderr)
        sys.exit(1)
    except IOError as e:
        print(f"File error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
