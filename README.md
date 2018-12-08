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

When hashing data larger than 64-bits the hasher will fallback to a secondary algorithm suitable for arbitrary data (defaults to `FxHasher`).

## Benchmark Performance
For more info see the [the full benchmark report](bench_report.md).

Hash Algorithm | Integer Sample Set | `int_hash` is
--- | --- | ---
Rust default _aka **SipHash**_ | Random numbers | **1.23-9.61x** faster
Rust default _aka **SipHash**_ | Natural numbers | **3.82-21.6x** faster
Rust default _aka **SipHash**_ | 32× table | **1.59-4.94x** faster
`fnv` | Random numbers | **0.99-1.58x** faster
`fnv` | Natural numbers | **2.11-11.08x** faster
`fnv` | 32× table | **0.57-1.09x** faster
`rustc-hash` _aka **fx**_ | Random numbers | **0.95-1.29x** faster
`rustc-hash` _aka **fx**_ | Natural numbers | **1.26-2.15x** faster
`rustc-hash` _aka **fx**_ | 32× table | **0.94-1.28x** faster
`seahash` | Random numbers | **1.16-5.57x** faster
`seahash` | Natural numbers | **3.65-19.35x** faster
`seahash` | 32× table | **1.33-3.11x** faster
`twox_hash` _aka **xx**_ | Random numbers | **1.25-9.55x** faster
`twox_hash` _aka **xx**_ | Natural numbers | **4.05-23.95x** faster
`twox_hash` _aka **xx**_ | 32× table | **1.68-5.67xx** faster

_Note: For > 64-bit keys int_hash will perform inline with `rustc_hash`._

## Limitations
The algorithm is non-cryptographic.

## Why is it so fast
`int_hash` is dedicated at solving integer-sized hashing. Producing a unique `u64` from an integer is not a very difficult problem, though getting a good spread of values to minimise hashmap collisions is a little harder.

The current implementation uses simple `u64` XOR mixing to spread values. The sheer simplicity of this approach makes the hashing operation very fast and the primitive spreading is good enough to produce best-in-class hashmap performance.
