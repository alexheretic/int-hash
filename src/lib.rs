// Copyright 2018 Alex Butler.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Very fast, very simple hash algorithm designed for use in
//! integer hash maps & sets.
//!
//! This crate attempts to provide the **fastest option for integer key hashmaps**.
//! So the algorithm may change if a better method is found for this use case.
//!
//! # Example
//!
//! ```rust
//! use int_hash::IntHashMap;
//! let mut map: IntHashMap<u32, &str> = IntHashMap::default();
//! map.insert(22, "abc");
//! ```
//!
//! # Limitations
//! Valid for use only with integer sized data, ie <= 16 bytes.
//! This is enforced with debug assertions.

extern crate byteorder;

use byteorder::{ByteOrder, NativeEndian};
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::BitXor;

/// Type alias for a hashmap using [IntHasher] hashing.
///
/// # Example
///
/// ```rust
/// use int_hash::IntHashMap;
/// let mut map: IntHashMap<usize, &str> = IntHashMap::default();
/// map.insert(22, "abc");
/// ```
pub type IntHashMap<K, V> = HashMap<K, V, IntBuildHasher>;

/// Type alias for a hashmap using [IntHasher] hashing.
///
/// # Example
///
/// ```rust
/// use int_hash::IntHashSet;
/// let mut set: IntHashSet<usize> = IntHashSet::default();
/// set.insert(22);
/// ```
pub type IntHashSet<V> = HashSet<V, IntBuildHasher>;

/// Very fast, very simple hash algorithm designed for use in
/// integer hash maps & sets.
///
/// Valid for use only with integer sized data, ie <= 16 bytes.
/// This is enforced with debug assertions.
pub struct IntHasher {
    #[cfg(debug_assertions)]
    bytes_hashed: u8,
    hash: usize,
}

impl Default for IntHasher {
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn default() -> Self {
        Self::new(0xdead_beef)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn default() -> Self {
        // Magic number found by random search & benchmark
        Self::new(0xe26a_f83e_0dff_34cc)
    }
}

#[cfg(debug_assertions)]
const ASSERT_MESSAGE: &str = "int_hash algorithm only valid for values of size <= 16 bytes";

impl IntHasher {
    /// Returns a new [IntHasher] with a custom inital value.
    ///
    /// `IntHasher::default()` is generally fine.
    #[inline]
    pub fn new(seed: usize) -> Self {
        Self {
            #[cfg(debug_assertions)]
            bytes_hashed: 0,
            hash: seed,
        }
    }

    #[inline]
    fn hash(&mut self, v: usize) {
        #[cfg(debug_assertions)] {
            if cfg!(target_pointer_width = "32") {
                self.bytes_hashed += 4;
            } else {
                self.bytes_hashed += 8;
            }
            assert!(self.bytes_hashed <= 16, ASSERT_MESSAGE);
        }

        self.hash = self.hash.bitxor(v);
    }
}

impl Hasher for IntHasher {
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 4 {
            self.write_u32(NativeEndian::read_u32(bytes));
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.write_u16(NativeEndian::read_u16(bytes));
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.write_u8(bytes[0]);
        }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.write_u64(NativeEndian::read_u64(bytes));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.write_u32(NativeEndian::read_u32(bytes));
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.write_u16(NativeEndian::read_u16(bytes));
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.write_u8(bytes[0]);
        }
    }

    #[inline]
    fn write_u8(&mut self, v: u8) {
        self.hash(usize::from(v));
    }

    #[inline]
    fn write_u16(&mut self, v: u16) {
        self.hash(usize::from(v));
    }

    #[inline]
    fn write_u32(&mut self, v: u32) {
        self.hash(v as usize);
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn write_u64(&mut self, v: u64) {
        self.hash(v as usize);
        self.hash((v >> 32) as usize);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write_u64(&mut self, v: u64) {
        self.hash(v as usize);
    }

    #[inline]
    fn write_u128(&mut self, v: u128) {
        self.write_u64(v as u64);
        self.write_u64((v >> 64) as u64);
    }

    #[inline]
    fn write_usize(&mut self, v: usize) {
        self.hash(v);
    }

    #[inline]
    fn write_isize(&mut self, v: isize) {
        self.write_usize(v as usize);
    }

    #[inline]
    fn write_i8(&mut self, v: i8) {
        self.write_u8(v as u8);
    }

    #[inline]
    fn write_i16(&mut self, v: i16) {
        self.write_u16(v as u16);
    }

    #[inline]
    fn write_i32(&mut self, v: i32) {
        self.write_u32(v as u32);
    }

    #[inline]
    fn write_i64(&mut self, v: i64) {
        self.write_u64(v as u64);
    }

    #[inline]
    fn write_i128(&mut self, v: i128) {
        self.write_u128(v as u128);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}

/// [IntHasher] builder.
pub type IntBuildHasher = BuildHasherDefault<IntHasher>;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn mix_hash_map() {
        let mut map: HashMap<u64, _, _> = HashMap::with_hasher(IntBuildHasher::default());

        map.insert(123, "a");
        map.insert(124, "b");

        assert_eq!(map[&123], "a");
        assert_eq!(map[&124], "b");
    }

    #[test]
    fn wrapped_signed_type() {
        #[derive(PartialEq, Eq, Hash)]
        struct MyInt(i32);

        let mut map: HashMap<MyInt, &str, IntBuildHasher> = IntHashMap::default();

        map.insert(MyInt(123), "a");
        map.insert(MyInt(124), "b");
        map.insert(MyInt(-124), "c");

        assert_eq!(map[&MyInt(123)], "a");
        assert_eq!(map[&MyInt(124)], "b");
        assert_eq!(map[&MyInt(-124)], "c");
    }

    #[test]
    fn arbitrary_type() {
        let mut map: IntHashMap<[u8; 4], _> = HashMap::default();

        map.insert([1, 2, 3, 4], "a");
        map.insert([2, 3, 4, 5], "b");
        map.insert([0, 0, 0, 1], "c");

        assert_eq!(map[&[1, 2, 3, 4]], "a");
        assert_eq!(map[&[2, 3, 4, 5]], "b");
        assert_eq!(map[&[0, 0, 0, 1]], "c");
    }

    /// Should assert small key usage
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn max_data_debug_assert() {
        let mut large_key_map: IntHashMap<[u32; 4], _> = HashMap::default();
        large_key_map.insert([1, 2, 3, 4], "a");
    }

    /// Should not panic in release mode
    #[test]
    #[cfg(not(debug_assertions))]
    fn max_data_debug_assert() {
        let mut large_key_map: IntHashMap<[u32; 4], _> = HashMap::default();
        large_key_map.insert([1, 2, 3, 4], "a");

        assert_eq!(large_key_map[&[1, 2, 3, 4]], "a");
    }
}
