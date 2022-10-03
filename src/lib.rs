use rand::{distributions::Standard, prelude::Distribution, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use seed::Seed;

pub mod board;
pub mod seed;

pub trait Generate: std::marker::Sized {
    /// Generate a new instance of the type using a random seed.
    fn generate() -> Self
    where
        Standard: Distribution<Self>,
    {
        Self::generate_seeded(&mut ChaCha8Rng::from_seed(Seed::default().into()))
    }

    /// Generate a new instance of the type using the given seed.
    fn generate_seeded<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        Standard: Distribution<Self>,
    {
        rng.gen()
    }
}

pub fn gen_seed() -> Seed {
    Seed::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_board() {
        let board = board::Board::generate();
        println!("{:?}", board);
    }
}
