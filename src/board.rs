use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{seed::Seed, Generate};

#[derive(Debug, Copy, Clone)]
pub struct Number {
    value: u8,
}

impl Distribution<Number> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Number {
        Number {
            value: rng.gen_range(0..=9),
        }
    }
}

impl Generate for Number {}

impl Default for Number {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<Number> for u8 {
    fn from(number: Number) -> Self {
        number.value
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cube {
    rows: [[Number; 3]; 3],
}

impl Distribution<Cube> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cube {
        let mut rows = [[Number::default(); 3]; 3];

        for row in rows.iter_mut() {
            for number in row.iter_mut() {
                *number = Number::generate_seeded(rng);
            }
        }

        Cube { rows }
    }
}

impl Generate for Cube {}

impl Default for Cube {
    fn default() -> Self {
        Self::generate()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    cubes: [[Cube; 3]; 3],
}

impl Distribution<Board> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Board {
        let mut cubes = [[Cube::default(); 3]; 3];

        for cubes in cubes.iter_mut() {
            for cube in cubes.iter_mut() {
                *cube = Cube::generate_seeded(rng);
            }
        }

        Board { cubes }
    }
}

impl Generate for Board {}

impl Default for Board {
    fn default() -> Self {
        Self::generate()
    }
}
