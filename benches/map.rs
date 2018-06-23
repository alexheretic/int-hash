#![feature(test)]
#![allow(unused)]

extern crate fnv;
extern crate int_hash as this_crate;
extern crate rustc_hash;
extern crate seahash;
extern crate twox_hash;
extern crate test;

use fnv::FnvBuildHasher;
use this_crate::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::io::{BufRead, BufReader};
use test::Bencher;

/// Each file contains 100000 unique integers
const KEYS_LINE_COUNT: usize = 100_000;
/// Sample size of the integers, ie should be <= `KEYS_LINE_COUNT`
/// - 300 is fast & seems ok
/// - 100_000 gives full report
const BENCH_MAP_SIZE: usize = 300; // 100_000;

macro_rules! bench_keys {
    ($build_hasher:expr, $($t:ty = $keys_name:ident = $keys_file:expr),+ $(,)*) => {
        $(
            mod $keys_name {
                use super::*;

                type Int = $t;

                fn test_values() -> Vec<Int> {
                    let mut test_vals = Vec::with_capacity(BENCH_MAP_SIZE);
                    let file = File::open($keys_file).unwrap();
                    for (idx, line) in BufReader::new(&file).lines().enumerate() {
                        if idx % (KEYS_LINE_COUNT / BENCH_MAP_SIZE) == 0 {
                            if let Ok(int) = Int::from_str_radix(&line.unwrap(), 10) {
                                test_vals.push(int);
                            }
                        }
                    }
                    if test_vals.len() < BENCH_MAP_SIZE {
                        for (idx, line) in BufReader::new(file).lines().enumerate() {
                            if idx % (KEYS_LINE_COUNT / BENCH_MAP_SIZE) != 0 {
                                if let Ok(int) = Int::from_str_radix(&line.unwrap(), 10) {
                                    test_vals.push(int);
                                    if test_vals.len() >= BENCH_MAP_SIZE {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    test_vals
                }

                #[bench]
                fn insert_all(b: &mut Bencher) {
                    let test_vals = test_values();

                    b.iter(|| {
                        let mut map = HashMap::with_hasher($build_hasher);
                        for val in &test_vals {
                            map.insert(*val, ());
                        }
                    });
                }

                #[bench]
                fn find_all(b: &mut Bencher) {
                    let test_vals = test_values();

                    let map = {
                        let mut map = HashMap::with_capacity_and_hasher(
                            KEYS_LINE_COUNT,
                            $build_hasher
                        );
                        for val in &test_vals {
                            map.insert(*val, *val);
                        }
                        map
                    };

                    b.iter(|| {
                        for val in &test_vals {
                            map.get(val).unwrap();
                        }
                    });
                }
            }
        )*
    }
}

macro_rules! bench_hasher {
    ($($mod_name:ident, $build_hasher:expr);+ $(;)*) => {
        $(
            mod $mod_name {
                use super::*;

                bench_keys!(
                    $build_hasher,
                    u128 = u128_natural = "benches/natural-u64.txt",
                    u64 = u64_natural = "benches/natural-u64.txt",
                    usize = usize_natural = "benches/natural-u64.txt",
                    u32 = u32_natural = "benches/natural-u64.txt",
                    u16 = u16_natural = "benches/natural-u64.txt",

                    u128 = u128_rand = "benches/rand-u64.txt",
                    u64 = u64_rand = "benches/rand-u64.txt",
                    usize = usize_rand = "benches/rand-u64-small.txt",
                    u32 = u32_rand = "benches/rand-u64-small.txt",
                    u16 = u16_rand = "benches/rand-u64-small.txt",
                    u8 = u8_rand = "benches/rand-u8.txt",

                    u64 = u64_times_table_32 = "benches/32xt-u64.txt",
                    u32 = u32_times_table_32 = "benches/32xt-u64.txt",
                    u16 = u16_times_table_32 = "benches/32xt-u64.txt",
                );
            }
        )*
    }
}

type FxBuildHasher = BuildHasherDefault<rustc_hash::FxHasher>;
type SeaBuildHasher = BuildHasherDefault<seahash::SeaHasher>;
type XxBuildHasher = BuildHasherDefault<twox_hash::XxHash>;

bench_hasher!(
    int_hash, IntBuildHasher::default();
    wang_mix, wang::WangMixBuildHasher::default();
    fnv_crate, FnvBuildHasher::default();
    rustc_crate, FxBuildHasher::default();

    // Following are not good for small values like integers (just here to show that)
    // default_hash, RandomState::new();
    // seahash_crate, SeaBuildHasher::default();
    // twox_crate, XxBuildHasher::default();
);

/// Thomas Wang 32-bit & 64-bit integer mix functions https://gist.github.com/badboy/6267743
mod wang {
    use std::hash::{BuildHasherDefault, Hasher};

    pub struct WangMixHasher {
        hash: u64,
    }

    impl Default for WangMixHasher {
        #[inline]
        fn default() -> Self {
            Self { hash: 0 }
        }
    }

    impl WangMixHasher {
        #[cfg(target_pointer_width = "64")]
        #[inline]
        fn hash(&mut self, mut k: usize) {
            k = (!k).wrapping_add(k << 21); // k = (k << 21) - k - 1;
            k = k ^ (k >> 24);
            k = (k.wrapping_add(k << 3)).wrapping_add(k << 8); // k * 265
            k = k ^ (k >> 14);
            k = (k.wrapping_add(k << 2)).wrapping_add(k << 4); // k * 21
            k = k ^ (k >> 28);
            k = k + (k << 31);
            self.hash = k as u64;
        }

        #[cfg(target_pointer_width = "32")]
        #[inline]
        fn hash(&mut self, mut k: usize) {
            k = (!k).wrapping_add(k << 15); // k = (k << 15) - k - 1;
            k = k ^ (k >> 12);
            k = k.wrapping_add(k << 2);
            k = k ^ (k >> 4);
            k = k.wrapping_mul(2057); // k = (k + (k << 3)) + (k << 11);
            k = k ^ (k >> 16);
            self.hash = k as u64;
        }
    }

    impl Hasher for WangMixHasher {
        fn write(&mut self, _: &[u8]) {
            panic!("Only valid for single integer hashing")
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
            let mix32: usize = v as usize ^ (v >> 32) as usize;
            self.hash(mix32);
        }

        #[cfg(target_pointer_width = "64")]
        #[inline]
        fn write_u64(&mut self, v: u64) {
            self.hash(v as usize);
        }

        #[inline]
        fn write_u128(&mut self, v: u128) {
            let mix64 = (v as u64) ^ (v >> 64) as u64;
            self.write_u64(mix64)
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
            self.hash
        }
    }

    pub type WangMixBuildHasher = BuildHasherDefault<WangMixHasher>;
}
