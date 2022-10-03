use rand::{prelude::*, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub type Seed = <ChaCha8Rng as SeedableRng>::Seed;

pub fn gen_seed() -> Seed {
    thread_rng().gen()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_seed() {
        let seed = gen_seed();
        assert_eq!(seed.len(), 32);
    }
}
