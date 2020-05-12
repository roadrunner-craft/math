use crate::random::Seed;

pub use noise::NoiseFn;
use noise::{Perlin, Seedable};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoiseOptions {
    scale: f64,
    seed: Seed,
}

impl NoiseOptions {
    pub fn new(scale: f64, seed: Seed) -> Self {
        Self { scale, seed }
    }
}

#[derive(Clone)]
pub struct Noise {
    noise: Perlin,
    options: NoiseOptions,
}

impl Noise {
    pub fn new(scale: f64, seed: Seed) -> Self {
        Self {
            noise: Perlin::new().set_seed(seed.0),
            options: NoiseOptions { scale, seed },
        }
    }

    pub fn from_options(options: NoiseOptions) -> Self {
        Self {
            noise: Perlin::new().set_seed(options.seed.0),
            options: options,
        }
    }
}

macro_rules! noise_impl {
    ($n:expr, $($i:expr),+) => (
        impl NoiseFn<[i64; $n]> for Noise {
            /// Returns a value between 0.0 and 1.0
            fn get(&self, point: [i64; $n]) -> f64 {
                (self.noise.get([
                    $((point[$i] as f64) / self.options.scale,)*
                ]) + 1.0) / 2.0
            }
        }
    );
}

noise_impl!(2, 0, 1);
noise_impl!(3, 0, 1, 2);
noise_impl!(4, 0, 1, 2, 3);
