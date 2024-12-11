use std::fs;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();
    let orginal_data = file
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    let ticks = 25;
    let mut data = orginal_data.clone();
    for _t in 0..ticks {
        data = tick(data);
    }
    println!("part1: {}", data.len())
}

fn tick(data: Vec<i64>) -> Vec<i64> {
    data.par_iter()
        .flat_map(|item| {
            let item = *item;
            let places = count_palces(item);
            if item == 0 {
                vec![1]
            } else if places % 2 == 0 {
                let splitter = i64::pow(10, places / 2);
                vec![item / splitter, item % splitter]
            } else {
                vec![item * 2024]
            }
        })
        .collect::<Vec<_>>()
}

fn count_palces(a: i64) -> u32 {
    f64::log10(a as f64) as u32 + 1
}
