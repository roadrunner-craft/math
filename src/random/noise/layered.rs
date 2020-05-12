use crate::random::Seed;
use noise::{NoiseFn, Perlin, Seedable};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayeredNoiseOptions {
    /// The number of noise layer to generate
    octaves: u32,
    /// The scale of the noise
    scale: f64,
    /// A number between 0.0 and 1.0 that determines how much each octave contributes to the final
    /// image (Changes amplitude)
    persistance: f64,
    /// A number >= 1 that determines how much detail is added or removed at each octave. (Changes
    /// frequency)
    lacunarity: f64,
    /// The seed used by the noise function
    seed: Seed,
}

impl LayeredNoiseOptions {
    pub fn new(octaves: u32, scale: f64, persistance: f64, lacunarity: f64, seed: Seed) -> Self {
        Self {
            octaves,
            scale,
            persistance,
            lacunarity,
            seed,
        }
    }
}

pub struct LayeredNoise {
    noise: Perlin,
    options: LayeredNoiseOptions,
}

impl LayeredNoise {
    pub fn new(octaves: u32, scale: f64, persistance: f64, lacunarity: f64, seed: Seed) -> Self {
        Self {
            noise: Perlin::new().set_seed(seed.0),
            options: LayeredNoiseOptions {
                octaves,
                scale,
                persistance,
                lacunarity,
                seed,
            },
        }
    }

    pub fn from_options(options: LayeredNoiseOptions) -> Self {
        Self {
            noise: Perlin::new().set_seed(options.seed.0),
            options: options,
        }
    }
}

macro_rules! layered_noise_impl {
    ($n:expr, $($i:expr),+) => (
        impl NoiseFn<[i64; $n]> for LayeredNoise {
            /// Returns a value between 0.0 and 1.0
            fn get(&self, point: [i64; $n]) -> f64 {
                let mut acc: f64 = 0.0;
                let mut acc_amplitude: f64 = 0.0;

                let mut frequency: f64 = 1.0;
                let mut amplitude: f64 = 1.0;

                for _ in 0..self.options.octaves {
                    let value = self.noise.get([
                        $(point[$i] as f64 * frequency / self.options.scale,)*
                    ]);

                    acc += value * amplitude;
                    acc_amplitude += amplitude;

                    frequency *= self.options.lacunarity;
                    amplitude *= self.options.persistance;
                }

                (acc + acc_amplitude) / (2.0 * acc_amplitude)
            }
        }
    );
}

layered_noise_impl!(2, 0, 1);
layered_noise_impl!(3, 0, 1, 2);
layered_noise_impl!(4, 0, 1, 2, 3);
