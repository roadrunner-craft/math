use crate::random::noise::{LayeredNoise, LayeredNoiseOptions};
use noise::NoiseFn;

pub struct CombinedNoise {
    noise1: LayeredNoise,
    noise2: LayeredNoise,
    weight: f64,
}

impl CombinedNoise {
    pub fn new(options1: LayeredNoiseOptions, options2: LayeredNoiseOptions, weight: f64) -> Self {
        Self {
            noise1: LayeredNoise::from_options(options1),
            noise2: LayeredNoise::from_options(options2),
            weight,
        }
    }
}

impl NoiseFn<[i64; 2]> for CombinedNoise {
    fn get(&self, point: [i64; 2]) -> f64 {
        let value1 = (self.noise1.get(point) * 2.0 - 1.0) * self.weight;
        let point2 = [(value1 + point[0] as f64) as i64, point[1]];

        self.noise2.get(point2)
    }
}
