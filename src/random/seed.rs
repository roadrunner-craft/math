use serde::{Deserialize, Serialize};
use std::convert::From;
use std::iter;
use std::time::SystemTime;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(pub u32);

impl Seed {
    pub fn new() -> Self {
        let value = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Seed((value & 0xffffffff) as u32)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SeedBuffer(pub [u8; 16]);

impl From<Seed> for SeedBuffer {
    fn from(seed: Seed) -> Self {
        let mut buffer: [u8; 16] = [0; 16];

        for (i, value) in iter::repeat(seed.0).take(4).enumerate() {
            buffer[i * 4] = (value >> 24 & 0xff) as u8;
            buffer[i * 4 + 1] = (value >> 16 & 0xff) as u8;
            buffer[i * 4 + 2] = (value >> 8 & 0xff) as u8;
            buffer[i * 4 + 3] = (value & 0xff) as u8;
        }

        Self(buffer)
    }
}
