use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Seed([u8; 32]);

impl Seed {
    pub fn new(seed: [u8; 32]) -> Self {
        Self(seed)
    }

    pub fn generate() -> Self {
        Self::new(thread_rng().gen())
    }
}

impl Default for Seed {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Seed> for [u8; 32] {
    fn from(seed: Seed) -> Self {
        seed.0
    }
}
