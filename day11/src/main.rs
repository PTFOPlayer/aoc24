use ahash::AHashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();
    let mut data = AHashMap::new();
    file.split_whitespace()
        .for_each(|s| *data.entry(s.parse::<i64>().unwrap()).or_default() += 1);

    for _t in 0..25 {
        data = tick(data);
    }
    println!("part1: {}", data.iter().map(|x| *x.1).sum::<i64>());

    for _t in 0..50 {
        data = tick(data);
    }
    println!("part2: {}", data.iter().map(|x| *x.1).sum::<i64>())
}

fn tick(data: AHashMap<i64, i64>) -> AHashMap<i64, i64> {
    let mut new_data = AHashMap::with_capacity(4000);
    data.into_iter().for_each(|(item, count)| match item {
        0 => *new_data.entry(1).or_default() += count,
        item => {
            let places = item.checked_ilog10().unwrap_or_default() + 1;
            if places % 2 == 0 {
                let splitter = 10i64.pow(places >> 1);
                *new_data.entry(item / splitter).or_default() += count;
                *new_data.entry(item % splitter).or_default() += count;
            } else {
                *new_data.entry(item * 2024).or_default() += count;
            }
        }
    });

    new_data
}
