pub mod part1;
pub mod part2;

use std::fs;

pub struct Vec2(pub isize, pub isize);

impl Vec2 {
    pub fn zero() -> Self {
        Self(0, 0)
    }
}

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    println!("{}", part1::part1(&file));
    println!("{}", part2::part2(&file));
}
