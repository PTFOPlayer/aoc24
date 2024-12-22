use std::{collections::HashMap, fs};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let lines = file.lines().collect_vec();
    let part1 = lines
        .par_iter()
        .map(|line| {
            let mut num = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                num = calculate_secret(num);
            }
            num
        })
        .sum::<u64>();
    println!("{}", part1);

    let secrets = lines
        .par_iter()
        .map(|line| {
            let mut num = line.parse::<u64>().unwrap();
            let mut differences = vec![];
            let mut window = [0; 4];
            let mut prev = num;
            for _ in 0..2000 {
                num = calculate_secret(num);
                window.rotate_left(1);
                window[3] = (num % 10) as i8 - (prev % 10) as i8;
                if !differences
                    .iter()
                    .map(|(window, _)| window)
                    .contains(&window)
                {
                    differences.push((window, num % 10));
                }
                prev = num;
            }

            differences[3..].to_vec()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut result: HashMap<[i8; 4], u64> = HashMap::new();
    for (key, value) in secrets {
        *result.entry(key).or_insert(0) += value;
    }

    let part2 = result.iter().max_by(|lhs, rhs| lhs.1.cmp(rhs.1)).unwrap();
    println!("{:?}", part2);
}

fn calculate_secret(mut num: u64) -> u64 {
    num = ((num * 64) ^ num) % 16777216;
    num = ((num / 32) ^ num) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;
    num
}
