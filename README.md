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

## Benchmark Performance
For more info see the [the full benchmark report](bench_report.md).

Hash Algorithm | Integer Sample Set | `int_hash` is
--- | --- | ---
Rust default _aka **SipHash**_ | ℕ: Natural numbers | **2.53-9.06x** faster
Rust default _aka **SipHash**_ | Random numbers | **1.18-3.90x** faster
Rust default _aka **SipHash**_ | 32× table | **1.49-3.13x** faster
`fnv` | ℕ: Natural numbers | **1.31-5.84x** faster
`fnv` | Random numbers | **1.00-1.84x** faster
`fnv` | 32× table | **0.59-1.14x** faster
`rustc-hash` _aka **fx**_ | ℕ: Natural numbers | **1.14-2.48x** faster
`rustc-hash` _aka **fx**_ | Random numbers | **0.95-1.07x** faster
`rustc-hash` _aka **fx**_ | 32× table | **0.97-1.13x** faster
`seahash` | ℕ: Natural numbers | **2.71-10.67x** faster
`seahash` | Random numbers | **1.11-2.61x** faster
`seahash` | 32× table | **1.29-2.14x** faster
`twox_hash` _aka **xx**_ | ℕ: Natural numbers | **2.93-9.85x** faster
`twox_hash` _aka **xx**_ | Random numbers | **1.20-4.17x** faster
`twox_hash` _aka **xx**_ | 32× table | **1.55-3.64x** faster

## Limitations
`int_hash` is valid for use only with integer sized data, ie <= 16 bytes. This is enforced with debug assertions. This should guarantee that whenever `int_hash` works it's among the fastest options.

However, for general non-integer small keys ***fx-hash*** seems the best option. Don't forget about ***vec_map*** either which may fit a natural number use case better than any hashmap.

The algorithm is non-cryptographic.

## Why is it so fast
`int_hash` is dedicated at solving integer-sized hashing and _only_ integer-sized hashing. Producing a unique `u64` from an integer is not a very difficult problem, though getting a good spread of values to minimise hashmap collisions is a little harder.

The current implementation uses simple `usize` XOR mixing to spread values. The sheer simplicity of this approach makes the hashing operation very fast and the primitive spreading is good enough to produce best-in-class hashmap performance.
