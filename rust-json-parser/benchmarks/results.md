# Benchmark Results

**Date:** 2026-03-04 19:18:23  
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
| small.json | 100 | 0.000106 | 0.000213 | 0.001428 | 2.00x faster | 13.43x faster |
| small.json | 1,000 | 0.000704 | 0.001641 | 0.011088 | 2.33x faster | 15.75x faster |
| medium.json | 100 | 0.006587 | 0.005269 | 0.068515 | 1.25x slower | 10.40x faster |
| medium.json | 1,000 | 0.051115 | 0.046113 | 0.678459 | 1.11x slower | 13.27x faster |
| large.json | 100 | 0.051143 | 0.047276 | 0.682032 | 1.08x slower | 13.34x faster |
| large.json | 1,000 | 0.513726 | 0.468484 | 6.941111 | 1.10x slower | 13.51x faster |
| xlarge.json | 100 | 0.228639 | 0.163425 | 2.359384 | 1.40x slower | 10.32x faster |
| xlarge.json | 1,000 | 2.179846 | 1.645709 | 24.203882 | 1.32x slower | 11.10x faster |
| nested.json | 100 | 0.006333 | 0.005368 | 0.083907 | 1.18x slower | 13.25x faster |
| nested.json | 1,000 | 0.064188 | 0.053686 | 0.836116 | 1.20x slower | 13.03x faster |

## Summary

### Per-Iteration Totals

| Iterations | Rust Total (s) | Python json Total (s) | simplejson Total (s) |
|------------|----------------|----------------------|----------------------|
| 100 | 0.292809 | 0.221551 | 3.195264 |
| 1,000 | 2.809578 | 2.215631 | 32.670656 |

### Overall Totals

| Parser | Total Time (s) |
|--------|----------------|
| Rust | 3.102387 |
| Python json | 2.437183 |
| simplejson | 35.865920 |
| **Total benchmark time** | **41.405490** |
