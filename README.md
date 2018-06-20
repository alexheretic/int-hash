int_hash
[![crates.io](https://img.shields.io/crates/v/int_hash.svg)](https://crates.io/crates/int_hash)
[![Documentation](https://docs.rs/int_hash/badge.svg)](https://docs.rs/int_hash)
================

Very fast, very simple hash algorithm designed for use in integer hash maps & sets.

This crate attempts to provide the **fastest option for integer key hashmaps** in the Rust language.
So the algorithm may change if a better method is found for this use case.

```rust
use int_hash::IntHashMap;
let mut map: IntHashMap<u32, &str> = IntHashMap::default();
map.insert(22, "abc");
```

## Limitations
`int_hash` is valid for use only with integer sized data, ie <= 16 bytes. This is enforced with debug assertions.

## Benchmark Performance
For more info see the [the full benchmark report](bench_report.md).

Hash Algorithm | Integer Sample Set | `int_hash` is
--- | --- | ---
Rust default _aka **SipHash**_ | ℕ: Natural numbers | **2.57-8.38x** faster
Rust default _aka **SipHash**_ | Random numbers | **1.17-3.61x** faster
Rust default _aka **SipHash**_ | 32× table | **1.52-3.09x** faster
`fnv` | ℕ: Natural numbers | **1.36-5.27x** faster
`fnv` | Random numbers | **0.99-1.83x** faster
`fnv` | 32× table | **0.60-1.12x** faster
`rustc-hash` _aka **fx**_ | ℕ: Natural numbers | **1.16-2.34x** faster
`rustc-hash` _aka **fx**_ | Random numbers | **0.94-1.10x** faster
`rustc-hash` _aka **fx**_ | 32× table | **0.98-1.17x** faster
`seahash` | ℕ: Natural numbers | **2.72-9.70x** faster
`seahash` | Random numbers | **1.07-2.83x** faster
`seahash` | 32× table | **1.31-2.10x** faster
`twox_hash` _aka **xx**_ | ℕ: Natural numbers | **2.81-9.19x** faster
`twox_hash` _aka **xx**_ | Random numbers | **1.17-3.91x** faster
`twox_hash` _aka **xx**_ | 32× table | **1.52-3.59x** faster
