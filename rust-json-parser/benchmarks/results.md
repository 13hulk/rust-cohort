# Benchmark Results

**Date:** 2026-03-04 19:44:06  
**Platform:** macOS-15.7.2-arm64-arm-64bit-Mach-O  
**Python:** 3.14.3  
**Build:** release  

## Benchmark Plan

Three-way comparison of JSON parsing performance:

- **Rust** -- our `rust_json_parser` (PyO3 extension, release build)
- **Python json** -- CPython's built-in `json.loads()` (C implementation)
- **simplejson** -- pure-Python `simplejson.loads()`

### Input Files

| File | Description | Size |
|------|-------------|------|
| small.json | Single flat object with 6 fields | 110 bytes |
| medium.json | Array of 75 objects with nested address | 10.1 KB |
| large.json | Array of 750 objects with nested address | 103.9 KB |
| xlarge.json | 1230 objects with long strings and nested metadata | 500.5 KB |
| nested.json | Deeply nested objects and arrays (228 levels) | 10.1 KB |

### Method

Each file is parsed repeatedly at two iteration counts (100, 1000) to measure both startup overhead and sustained throughput. All timing uses `std::time::Instant` in Rust, including Python parser calls via PyO3 FFI. The "Rust vs" columns show how many times faster or slower our parser is compared to each baseline.

## Results

| File | Iterations | Rust (s) | Python json (s) | simplejson (s) | Rust vs json | Rust vs simplejson |
|------|------------|----------|-----------------|----------------|--------------|---------------------|
| small.json | 100 | 0.000099 | 0.000276 | 0.001617 | 2.78x faster | 16.27x faster |
| small.json | 1,000 | 0.000897 | 0.001825 | 0.012850 | 2.04x faster | 14.33x faster |
| medium.json | 100 | 0.006945 | 0.005267 | 0.073469 | 1.32x slower | 10.58x faster |
| medium.json | 1,000 | 0.054699 | 0.045556 | 0.727500 | 1.20x slower | 13.30x faster |
| large.json | 100 | 0.054167 | 0.047456 | 0.729585 | 1.14x slower | 13.47x faster |
| large.json | 1,000 | 0.531420 | 0.467366 | 7.163010 | 1.14x slower | 13.48x faster |
| xlarge.json | 100 | 0.243130 | 0.163195 | 2.491205 | 1.49x slower | 10.25x faster |
| xlarge.json | 1,000 | 2.023176 | 1.639014 | 24.868609 | 1.23x slower | 12.29x faster |
| nested.json | 100 | 0.006636 | 0.005176 | 0.084177 | 1.28x slower | 12.68x faster |
| nested.json | 1,000 | 0.065130 | 0.050914 | 0.837780 | 1.28x slower | 12.86x faster |

## Summary

### Per-Iteration Totals

| Iterations | Rust Total (s) | Python json Total (s) | simplejson Total (s) |
|------------|----------------|----------------------|----------------------|
| 100 | 0.310978 | 0.221369 | 3.380054 |
| 1,000 | 2.675322 | 2.204674 | 33.609748 |

### Overall Totals

| Parser | Total Time (s) |
|--------|----------------|
| Rust | 2.986300 |
| Python json | 2.426043 |
| simplejson | 36.989802 |
| **Total benchmark time** | **42.402145** |
