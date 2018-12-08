// Copyright 2018 Alex Butler.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Very fast, very simple hash algorithm designed for use in integer hash maps & sets.
//!
//! This crate attempts to provide the **fastest option for integer key hashmaps**.
//! So the algorithm may change if a better method is found for this use case.
//!
//! When hashing data larger than 64-bits the hasher will fallback to a secondary algorithm suitable
//! for arbitrary data (defaults to `FxHasher`).
//!
//! # Example
//!
//! ```rust
//! use int_hash::IntHashMap;
//! let mut map: IntHashMap<u32, &str> = IntHashMap::default();
//! map.insert(22, "abc");
//! ```
use byteorder::{ByteOrder, NativeEndian};
use rustc_hash::FxHasher;
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
/// When hashing data larger than 64-bits the hasher will fallback
/// to a secondary algorithm suitable for arbitrary data (defaults to `FxHasher`).
pub enum IntHasher<F: Hasher + Default = FxHasher> {
    Small { data: [u8; 8], length: u8 },
    Large(F),
}

impl<F: Hasher + Default> Default for IntHasher<F> {
    #[inline]
    fn default() -> Self {
        IntHasher::Small {
            data: [0; 8],
            length: 0,
        }
    }
}

impl<F: Hasher + Default> IntHasher<F> {
    #[inline]
    fn enlarge(&mut self) -> &mut F {
        match self {
            IntHasher::Small { data, .. } => {
                let mut large_hasher = F::default();
                large_hasher.write(data);
                *self = IntHasher::Large(large_hasher);
                if let IntHasher::Large(hasher) = self {
                    hasher
                } else {
                    unreachable!()
                }
            }
            IntHasher::Large(ref mut hasher) => hasher,
        }
    }
}

impl<F: Hasher + Default> Hasher for IntHasher<F> {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        match self {
            IntHasher::Small {
                ref mut data,
                ref mut length,
            } => {
                let data_used = *length as usize;
                let new_bytes_len = bytes.len();
                if new_bytes_len > 8 || new_bytes_len + data_used > 8 {
                    self.enlarge().write(bytes);
                } else {
                    data[data_used..data_used + new_bytes_len].clone_from_slice(&bytes);
                    *length += new_bytes_len as u8;
                }
            }
            IntHasher::Large(hasher) => hasher.write(bytes),
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        match self {
            IntHasher::Small { data, .. } => {
                NativeEndian::read_u64(data).bitxor(0xe26a_f83e_0dff_34cc)
            }
            IntHasher::Large(hasher) => hasher.finish(),
        }
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

    /// Should not panic in release mode
    #[test]
    fn max_data_debug_assert() {
        let mut large_key_map: IntHashMap<[u32; 4], _> = HashMap::default();
        large_key_map.insert([1, 2, 3, 4], "a");

        assert_eq!(large_key_map[&[1, 2, 3, 4]], "a");
    }
}
