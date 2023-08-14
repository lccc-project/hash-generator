#![cfg_attr(not(test), no_std)]
//! A crate for generating near-perfect hash functions
//!
//! The generated hash functions are based on the [`gen::SipHasher`]
//!

extern crate alloc;

pub mod gen;
pub mod hash;

#[cfg(test)]
mod test {
    use core::hash::{Hash, Hasher};

    const TEST_CRATE_ID: u64 = 8936564510611380703;

    const TEST_DENSITY: f64 = 1.0f64 / 1024.0f64;

    const TEST_STEPS: usize = 8;

    const SAMPLE: &[&str] = &[
        "hello", "world", "foo", "baz", "bar", "lorem", "ipsum", "dolar", "sit", "amit",
    ];

    #[test]

    fn build_table_finishes() {
        let _ = crate::gen::gen_hash_fn::<_, 2, 4>(SAMPLE, TEST_CRATE_ID, TEST_DENSITY, TEST_STEPS);
    }

    #[test]
    fn build_table_consistent() {
        let (key, tsize) =
            crate::gen::gen_hash_fn::<_, 2, 4>(SAMPLE, TEST_CRATE_ID, TEST_DENSITY, TEST_STEPS);
        let (key2, tsize2) =
            crate::gen::gen_hash_fn::<_, 2, 4>(SAMPLE, TEST_CRATE_ID, TEST_DENSITY, TEST_STEPS);

        assert_eq!(key, key2);
        assert_eq!(tsize, tsize2);
    }

    #[test]
    fn build_table_perfect() {
        let (key, tsize) =
            crate::gen::gen_hash_fn::<_, 2, 4>(SAMPLE, TEST_CRATE_ID, 0.0, TEST_STEPS);

        let mut keys = vec![None; tsize];

        for &str in SAMPLE {
            let mut hasher = crate::hash::SipHasher::<2, 4>::new_with_keys(TEST_CRATE_ID, key);
            str.hash(&mut hasher);
            let idx = hasher.finish() as usize;

            if let Some(existing) = core::mem::replace(&mut keys[idx & (tsize - 1)], Some(str)) {
                panic!(
                    "Duplicate key placed in array: {} and {} both hashed to {}",
                    existing, key, idx
                );
            }
        }
    }
}
