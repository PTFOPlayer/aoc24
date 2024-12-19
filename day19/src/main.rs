use ahash::AHashMap;
use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut lines = file.lines();

    let variants = lines
        .next()
        .unwrap()
        .split(", ")
        .into_group_map_by(|str| str.chars().next().unwrap());

    let mut cache = AHashMap::new();
    let res = lines
        .skip(1)
        .map(|towel| search_possibilities(towel, &variants, &mut cache))
        .filter(|t| *t != 0)
        .fold((0, 0), |lhs, rhs| (lhs.0 + 1, lhs.1 + rhs));

    println!("part1: {}", res.0);
    println!("part2: {}", res.1);
}

fn search_possibilities<'a>(
    towel: &'a str,
    variants: &HashMap<char, Vec<&str>>,
    cache: &mut AHashMap<&'a str, i64>,
) -> i64 {
    if let Some(cached) = cache.get(towel) {
        return *cached;
    }

    let Some(chr) = towel.chars().next() else {
        return 1;
    };

    let Some(v) = variants.get(&chr) else {
        return 0;
    };

    let mut cnt = 0;
    for v in v {
        if let Some(new_towel) = towel.strip_prefix(v) {
            cnt += search_possibilities(new_towel, variants, cache)
        }
    }

    cache.insert(towel, cnt);

    return cnt;
}
