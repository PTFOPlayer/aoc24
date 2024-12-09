use std::{collections::VecDeque, fs};

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();
    let part1 = part1(&file);
    println!("{}", part1);
}

fn part1(file: &str) -> i64 {
    let mut deserialized: VecDeque<Option<i64>> = VecDeque::new();
    let mut tick_tock = true;
    let mut idx = 0;
    for char in file.as_bytes() {
        let len = *char as i64 - b'0' as i64;
        if tick_tock {
            for _ in 0..len {
                deserialized.push_back(Some(idx));
            }
            idx += 1;
        } else {
            for _ in 0..len {
                deserialized.push_back(None);
            }
        }
        tick_tock = !tick_tock;
    }

    let mut res = 0;
    let mut idx = 0;
    'l1: while deserialized.len() > 0 {
        let fst = deserialized.pop_front();
        if let Some(Some(fst)) = fst {
            res += idx * fst;
        } else {
            let mut last = None;
            while last == None {
                if let Some(lst) = deserialized.pop_back() {
                    last = lst
                } else {
                    break 'l1;
                }
            }
            res += idx * last.unwrap();
        }
        idx += 1;
    }

    res
}
