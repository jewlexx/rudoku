use rand_chacha::ChaCha8Rng;

use seed::Seed;

pub mod seed;

pub fn gen_seed() -> Seed {
    Seed::default()
}
