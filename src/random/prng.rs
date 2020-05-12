use rand_core::{RngCore, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use std::ops::Range;
use std::u32::MAX as U32_MAX;
use std::u64::MAX as U64_MAX;

use crate::random::{Seed, SeedBuffer};

type Rng = Lcg64Xsh32;

pub struct Prng {
    rng: Rng,
}

impl Prng {
    pub fn new(seed: Seed) -> Self {
        Self {
            rng: Rng::from_seed(SeedBuffer::from(seed).0),
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    pub fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    pub fn next_f32(&mut self) -> f32 {
        self.rng.next_u32() as f32 / U32_MAX as f32
    }

    pub fn next_f64(&mut self) -> f64 {
        self.rng.next_u64() as f64 / U64_MAX as f64
    }

    pub fn next_in_range(&mut self, range: Range<usize>) -> usize {
        let height = range.end - range.start;
        (self.next_f32() * height as f32 + range.start as f32) as usize
    }
}
