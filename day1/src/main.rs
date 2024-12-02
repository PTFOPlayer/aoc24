use std::fs;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut left = vec![];
    let mut right = vec![];

    file.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i64>().unwrap());
        right.push(split.next().unwrap().parse::<i64>().unwrap());
    });

    left.sort();
    right.sort();

    let part1 = left
        .iter()
        .zip(right.iter())
        .map(|(lhs, rhs)| i64::abs(lhs - rhs))
        .sum::<i64>();

    println!("{}", part1);

    let r_hash = right.into_iter().counts();
    let part2 = left
        .iter()
        .unique()
        .map(|lhs| *r_hash.get(lhs).unwrap_or(&0) as i64 * lhs)
        .sum::<i64>();

    println!("{}", part2);
}
