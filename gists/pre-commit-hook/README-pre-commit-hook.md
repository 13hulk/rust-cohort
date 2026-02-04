# Pre-commit Hooks for Rust Projects

Automatically run `cargo fmt` and `cargo clippy` before every commit. Catches issues before they reach the repo.

## Why Use Pre-commit Hooks?

- **Automatic formatting** - Never commit unformatted code
- **Catch issues early** - Clippy warnings caught before commit
- **Team consistency** - Everyone's code meets the same standards
- **No CI surprises** - If it commits, it passes checks

## Prerequisites

Install pre-commit (Python tool):

```bash
# macOS
brew install pre-commit

# pip
pip install pre-commit
```

## The Config File

Create `.pre-commit-config.yaml` in your project root:

```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --manifest-path rust-json-parser/Cargo.toml --
        language: system
        types: [rust]
        pass_filenames: false

      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --manifest-path rust-json-parser/Cargo.toml -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false
```

## Setup

```bash
# Install the hooks (run once per repo clone)
pre-commit install

# Or use make if you have the Makefile
make pre-commit-install
```

## How It Works

After setup, every `git commit` will:

1. Run `cargo fmt` - formats any changed Rust files
2. Run `cargo clippy` - checks for warnings/errors
3. If either fails, commit is blocked

```bash
$ git commit -m "Add feature"
cargo fmt................................................................Passed
cargo clippy.............................................................Passed
[main abc1234] Add feature
```

## If a Hook Fails

```bash
$ git commit -m "Add feature"
cargo fmt................................................................Failed
- hook id: cargo-fmt
- files were modified by this hook

# fmt modified files, need to re-stage and commit again
$ git add .
$ git commit -m "Add feature"
```

## Config Explained

| Field | Purpose |
|-------|---------|
| `repo: local` | Use local commands (not remote hook repos) |
| `entry` | The command to run |
| `--manifest-path` | Points to Cargo.toml location |
| `language: system` | Use system-installed cargo |
| `types: [rust]` | Only run on .rs files |
| `pass_filenames: false` | Run on whole project, not individual files |

## Useful Commands

```bash
# Run hooks manually (without committing)
pre-commit run --all-files

# Skip hooks for one commit (use sparingly!)
git commit --no-verify -m "WIP"

# Update hooks
pre-commit autoupdate
```

## Adapting for Your Project

If your Cargo.toml is in the root (not a subdirectory), simplify to:

```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --
        language: system
        types: [rust]
        pass_filenames: false

      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false
```
