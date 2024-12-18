use std::fs;

use itertools::Itertools;
use pathfinding::directed::astar::astar;

const MODIFIERS: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

const DIMM: usize = 71;
const TARGET: (usize, usize) = (70, 70);
const CELLS: usize = 1024;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let bytes = file
        .lines()
        .map(|line| {
            line.split(',')
                .map(|item| item.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize)>()
        })
        .filter_map(|x| x)
        .collect_vec();

    let mut memory = vec![vec![b'.'; DIMM]; DIMM];

    for (i, j) in &bytes[0..CELLS] {
        memory[*j][*i] = b'#';
    }

    let path = check_path(&memory);

    let Some(path) = path else { panic!("invalid") };
    println!("part1: {}", path.1);

    let mut cell = 0;
    let mut last_path = path.0;
    for iter in CELLS..bytes.len() {
        let (i, j) = bytes[iter];
        memory[j][i] = b'#';
        if last_path.contains(&(j, i)) {
            if let Some(path) = check_path(&memory) {
                last_path = path.0;
            } else {
                cell = iter;
                break;
            }
        }
    }

    println!("part2: {:?}", bytes[cell]);
}

fn check_path(memory: &Vec<Vec<u8>>) -> Option<(Vec<(usize, usize)>, usize)> {
    astar(
        &(0usize, 0usize),
        |p| {
            let p = *p;
            MODIFIERS.iter().filter_map(move |by| {
                let x = (p.0 as i32).wrapping_add(by.0) as usize;
                let y = (p.1 as i32).wrapping_add(by.1) as usize;

                if x < DIMM && y < DIMM && memory[x][y] != b'#' {
                    Some(((x, y), 1))
                } else {
                    None
                }
            })
        },
        |p| dinstance(*p),
        |p| *p == TARGET,
    )
}

fn dinstance(p: (usize, usize)) -> usize {
    (p.0.abs_diff(6) + p.1.abs_diff(6)) / 3
}
