use rand_chacha::ChaCha8Rng;

use seed::Seed;

pub mod seed;

pub fn gen_seed() -> Seed {
    Seed::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_seed() {
        let seed = gen_seed();

        println!("{:?}", seed);
    }
}
