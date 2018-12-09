int_hash
[![crates.io](https://img.shields.io/crates/v/int_hash.svg)](https://crates.io/crates/int_hash)
[![Documentation](https://docs.rs/int_hash/badge.svg)](https://docs.rs/int_hash)
================

This crate attempts to provide the **fastest option for integer key hashmaps** in the Rust language.

While the function's performance does seem superior for 64-bit data in benchmarks, it does _not_ clearly surpass fx-hash and seems to be more volatile in practice.
Because of this I would recommend the [rustc-hash](https://github.com/rust-lang/rustc-hash) crate for integer hash maps & sets.
