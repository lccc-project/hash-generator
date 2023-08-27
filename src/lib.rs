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
}
