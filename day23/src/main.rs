use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut map = HashMap::new();
    for line in file.lines() {
        let (lhs, rhs) = line.split('-').collect_tuple::<(&str, &str)>().unwrap();
        map.entry(lhs).or_insert(vec![]).push(rhs);
        map.entry(rhs).or_insert(vec![]).push(lhs);
    }

    let mut sets = HashSet::new();
    let mut seen = HashSet::new();
    for (start, reachable) in &map {
        if seen.contains(start) {
            continue;
        }
        for (i, middle) in reachable.iter().enumerate() {
            if seen.contains(middle) {
                continue;
            }
            for end in reachable[i + 1..].iter() {
                if map.get(end).unwrap().contains(middle) && !seen.contains(end) {
                    seen.insert(start);
                    sets.insert([start, middle, end]);
                }
            }
        }
    }

    let mut res = 0;
    for set in sets {
        if set.iter().any(|s| starts_t(s)) {
            res += 1;
            break;
        }
    }

    println!("{}", res)
}

fn starts_t(s: &str) -> bool {
    s.as_bytes()[0] == b't'
}
