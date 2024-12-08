use std::fs;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();

    let mut frequencies: FxHashMap<char, FxHashSet<(i64, i64)>> = FxHashMap::default();

    for (y, line) in file.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                frequencies
                    .entry(ch)
                    .or_default()
                    .insert((x as i64, y as i64));
            }
        }
    }

    let size = file.lines().count() as i64;

    let mut antinodes: FxHashSet<(i64, i64)> = FxHashSet::default();

    for (_, indices) in frequencies.iter() {
        indices
            .iter()
            .tuple_combinations()
            .map(|(lhs, rhs)| calculate_antinodes(*lhs, *rhs))
            .flatten()
            .filter(|antinode| in_range(*antinode, size))
            .for_each(|antinode| {
                antinodes.insert(antinode);
            });
    }
    println!("{}", antinodes.len());
}

fn in_range(pair: (i64, i64), size: i64) -> bool {
    0 <= pair.0 && pair.0 < size && 0 <= pair.1 && pair.1 < size
}

fn calculate_antinodes(lhs: (i64, i64), rhs: (i64, i64)) -> [(i64, i64); 2] {
    let ((ax, ay), (bx, by)) = (lhs, rhs);
    let antinode1: (i64, i64) = (2 * ax - bx, 2 * ay - by);
    let antinode2: (i64, i64) = (2 * bx - ax, 2 * by - ay);
    [antinode1, antinode2]
}
