use crate::hash::SipHasher;
use core::hash::{Hash, Hasher};
/// Generates a near-perfect hash function for the elements of `vals`, based on a `global_key`.
///
/// The return value is a pair of the set-specific key, and the minimum table size that allows the function to be unique when results are taken modulo this size.
/// The table size is always a power of 2.
///
/// The `gen_hash_fn` is based on the [`SipHasher<C,D>`] algorithm, where the first key is `global_key`, and the second key is the local key returned by the function.
///
/// Two configuration parameters are provided: `density_threshold` and `keys_per_step`. Increasing either will decrease the size of the table wrt. `vals.len()`, and decreasing either will increase the size of the table.
///
/// `density_threshold` is the maximum acceptable "density of collisons", which is the number of elements with a duplicate index divided by the table size.
///  Setting this to `0` generates a perfect hash function for the input. A higher number will allow for more collisions, thus allowing a smaller table size.
///
/// `keys_per_step` is the number of keys the function tries per each table size step. Keys are psuedo-randomly generated based on a seed that is variable only in `global_key`.
///
/// ## Notes
///
/// This function assumes (but does not verify) that `vals` contains no duplicates. Unpredictable results may occur if duplicates appear in `vals`.
///
pub fn gen_hash_fn<T: Hash, const C: usize, const D: usize>(
    vals: &[T],
    global_key: u64,
    density_threshold: f64,
    keys_per_step: usize,
) -> (u64, usize) {
    const SEED: u64 = 1138006940306161589;
    const MULTIPLIER: u64 = 4470274377298057907;
    const INC: u64 = 65537;

    let mut tsize = vals
        .len()
        .checked_next_power_of_two()
        .expect("Table size overflowed");

    let mut key = SEED.wrapping_add(global_key);

    loop {
        for _ in 0..keys_per_step {
            let mut counters = alloc::vec![0u32; tsize];
            for val in vals {
                let mut hasher = SipHasher::<C, D>::new_with_keys(global_key, key);

                val.hash(&mut hasher);

                let hash = hasher.finish() as usize;

                let idx = hash & (tsize - 1);

                counters[idx] += 1;
            }

            let collision_count = counters
                .into_iter()
                .map(|v| v.saturating_sub(1))
                .sum::<u32>();

            let density = (collision_count as f64) / (tsize as f64);

            if density <= density_threshold {
                return (key, tsize);
            }

            key = key.wrapping_mul(MULTIPLIER).wrapping_add(INC);
        }

        tsize = tsize.checked_mul(2).expect("Table size overflowed");
    }
}
