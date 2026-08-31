# math

A linear algebra and procedural noise library for the roadrunner voxel game. It gathers the math primitives the engine and world generation share — vectors, 4×4 matrices, quaternions, transforms, geometric containers, and seeded noise — behind one consistent API.

## Install

Add the crate as a git dependency:

```toml
[dependencies]
math = { git = "https://github.com/roadrunner-craft/math" }
```

## Usage

Vectors and quaternions:

```rust
use math::quaternion::Quaternion;
use math::vector::Vector3;

let direction = (Vector3::new(1.0, 2.0, 3.0) - Vector3::up()).normalized();
let facing = Vector3::cross(direction, Vector3::right());

let rotation = Quaternion::slerp(Quaternion::identity(), Quaternion::new(0.0, 0.0, 1.0, 0.0), 0.5);
```

Seeded Perlin noise, sampled on integer coordinates and normalized to `[0, 1]`:

```rust
use math::random::noise::{LayeredNoise, Noise, NoiseFn};
use math::random::Seed;

let seed = Seed::new();

let noise = Noise::new(64.0, seed);
let value = noise.get([x, z]);

let terrain = LayeredNoise::new(4, 128.0, 0.5, 2.0, seed);
```

A seeded pseudo-random number generator is available as `math::random::Prng`, and `math::transform::Transform` composes position, rotation, and scale into a `Matrix4`.

## Development

```sh
mise install
mise run build
mise run test
```
