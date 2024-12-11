use std::fs;

use fxhash::FxHashMap;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();
    let orginal_data = file
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    let mut data = FxHashMap::default();
    for item in orginal_data {
        *data.entry(item).or_insert(0i64) += 1;
    }
    for _t in 0..25 {
        data = part2_tick(data);
    }
    println!("part1: {}", data.iter().map(|x| *x.1).sum::<i64>());

    for _t in 0..50 {
        data = part2_tick(data);
    }
    println!("part2: {}", data.iter().map(|x| *x.1).sum::<i64>())
}

fn part2_tick(data: FxHashMap<i64, i64>) -> FxHashMap<i64, i64> {
    let mut new_data = FxHashMap::default();
    data.iter().for_each(|(item, count)| {
        let item = *item;
        let places = count_palces(item);
        if item == 0 {
            *new_data.entry(1).or_insert(0) += count;
        } else if places % 2 == 0 {
            let splitter = i64::pow(10, places / 2);
            *new_data.entry(item / splitter).or_insert(0) += count;
            *new_data.entry(item % splitter).or_insert(0) += count;
        } else {
            *new_data.entry(item * 2024).or_insert(0) += count;
        }
    });

    new_data
}

fn count_palces(a: i64) -> u32 {
    (f64::log10(a as f64) as u32 + 1) as u32
}
