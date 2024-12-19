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
    let cnt = lines
        .skip(1)
        .filter(|towel| search_possibilities(towel, &variants, &mut cache))
        .count();
    println!("{}", cnt);
}

fn search_possibilities<'a>(
    towel: &'a str,
    variants: &HashMap<char, Vec<&str>>,
    cache: &mut AHashMap<&'a str, bool>,
) -> bool {
    if let Some(cached) = cache.get(towel) {
        return *cached;
    }

    let Some(chr) = towel.chars().next() else {
        return true;
    };

    let Some(v) = variants.get(&chr) else {
        return false;
    };

    for v in v {
        if let Some(new_towel) = towel.strip_prefix(v) {
            if search_possibilities(new_towel, variants, cache) {
                cache.insert(towel, true);
                return true;
            }
        }
    }

    cache.insert(towel, false);

    return false;
}
