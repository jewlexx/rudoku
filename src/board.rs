use rand_chacha::ChaCha8Rng;

use crate::seed::Seed;

pub struct Cube {
    seed: Seed,
    rng: ChaCha8Rng,
    rows: [[u8; 3]; 3],
}

pub struct Board {
    seed: Seed,
    rng: ChaCha8Rng,
    cubes: [[Cube; 3]; 3],
}
