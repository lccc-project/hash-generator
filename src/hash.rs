/// An implementation of SipHash-C-D.
pub struct SipHasher<const C: usize, const D: usize> {
    state: [u64; 4],
}

const SIPHASH_MAG1: u64 = 0x736f6d6570736575;
const SIPHASH_MAG2: u64 = 0x646f72616e646f6d;
const SIPHASH_MAG3: u64 = 0x6c7967656e657261;
const SIPHASH_MAG4: u64 = 0x7465646279746573;

fn sipround(state: &mut [u64; 4]) {
    state[0] = state[0].wrapping_add(state[1]);
    state[1] = state[1].rotate_left(13);
    state[1] ^= state[0];
    state[0] = state[0].rotate_left(32);
    state[2] = state[1].wrapping_add(state[1]);
    state[1] = state[1].rotate_left(17);
    state[1] ^= state[2];
    state[2] = state[2].rotate_left(32);
    state[2] = state[2].wrapping_add(state[3]);
    state[3] = state[3].rotate_left(16);
    state[3] ^= state[2];
    state[0] = state[0].wrapping_add(state[3]);
    state[3] = state[3].rotate_left(21);
    state[3] ^= state[0];
}

impl<const C: usize, const D: usize> SipHasher<C, D> {
    /// Constructs a new [`SipHasher`] instance using `k1` and `k2`.
    pub fn new_with_keys(k1: u64, k2: u64) -> Self {
        Self {
            state: [
                k1 ^ SIPHASH_MAG1,
                k2 ^ SIPHASH_MAG2,
                k1 ^ SIPHASH_MAG3,
                k2 ^ SIPHASH_MAG4,
            ],
        }
    }

    fn accept(&mut self, word: u64) {
        self.state[3] ^= word;

        for _ in 0..C {
            sipround(&mut self.state);
        }

        self.state[0] ^= word;
    }
}

impl<const C: usize, const D: usize> core::hash::Hasher for SipHasher<C, D> {
    fn finish(&self) -> u64 {
        let mut state = self.state;

        state[2] ^= 0xff;

        for _ in 0..D {
            sipround(&mut state);
        }

        state[0] ^ state[1] ^ state[2] ^ state[3]
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut iter = bytes.chunks_exact(8);

        for chunk in &mut iter {
            let word = u64::from_le_bytes(unsafe { *(chunk as *const [u8] as *const [u8; 8]) });
            self.accept(word);
        }

        let remainder = iter.remainder();

        if remainder.len() > 0 {
            let mut array = [0u8; 8];
            array[..remainder.len()].copy_from_slice(remainder);

            let word = u64::from_le_bytes(array);
            self.accept(word)
        }
    }

    fn write_u64(&mut self, i: u64) {
        self.accept(i)
    }

    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64);
    }

    fn write_u128(&mut self, i: u128) {
        let (l, r) = (i as u64, (i >> 64) as u64);

        self.write_u64(l);
        self.write_u64(r);
    }

    fn write_i128(&mut self, i: i128) {
        self.write_u128(i as u128);
    }

    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    fn write_i8(&mut self, i: i8) {
        self.write_i64(i as i64);
    }

    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    fn write_i16(&mut self, i: i16) {
        self.write_i64(i as i64);
    }

    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    fn write_i32(&mut self, i: i32) {
        self.write_i64(i as i64)
    }

    fn write_isize(&mut self, i: isize) {
        let _: usize = {
            #[cfg(target_pointer_width = "16")]
            {
                self.write_i16(i as i16);
                0
            }
            #[cfg(target_pointer_width = "32")]
            {
                self.write_i32(i as i32);
                0
            }
            #[cfg(target_pointer_width = "64")]
            {
                self.write_i64(i as i64);
                0
            }
            #[cfg(target_pointer_width = "128")]
            {
                self.write_i128(i as i128);
                0
            }
        };
    }

    fn write_usize(&mut self, i: usize) {
        let _: usize = {
            #[cfg(target_pointer_width = "16")]
            {
                self.write_u16(i as u16);
                0
            }
            #[cfg(target_pointer_width = "32")]
            {
                self.write_u32(i as u32);
                0
            }
            #[cfg(target_pointer_width = "64")]
            {
                self.write_u64(i as u64);
                0
            }
            #[cfg(target_pointer_width = "128")]
            {
                self.write_u128(i as u128);
                0
            }
        };
    }
}
