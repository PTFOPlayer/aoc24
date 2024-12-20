use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

const MODIFIERS: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut map = vec![];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in file.lines().enumerate() {
        let mut row = vec![];
        for (j, byte) in line.as_bytes().iter().enumerate() {
            if *byte == b'S' {
                start = (i, j)
            }
            if *byte == b'E' {
                end = (i, j)
            }
            row.push(*byte);
        }
        map.push(row);
    }

    let mut path = VecDeque::from([(start.0, start.1, 0usize)]);
    let mut distances = HashMap::new();
    while let Some((x, y, n)) = path.pop_front() {
        if (x, y) == end {
            continue;
        }

        if distances.get(&(x, y)).is_some() {
            continue;
        }

        distances.insert((x, y), n);
        for (x_mod, y_mod) in MODIFIERS {
            let (next_x, next_y) = (x + x_mod as usize, y + y_mod as usize);
            if map[next_x][next_y] != b'#' {
                path.push_back((next_x, next_y, n + 1));
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for ((p1, &d1), (p2, &d2)) in distances.iter().tuple_combinations() {
        let d = distance(*p1, *p2);
        let relative = d2.abs_diff(d1);
        if relative >= d + 100 {
            if d <= 2 {
                part1 += 1;
            }
            if d <= 20 {
                part2 += 1;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2)
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}
