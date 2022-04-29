use crate::prelude::*;

pub fn random_direction() -> Point {
    let mut rng = RandomNumberGenerator::new();
    match rng.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1)
    }
}