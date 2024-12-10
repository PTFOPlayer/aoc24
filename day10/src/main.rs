use pathfinding::{directed::astar, prelude::AstarSolution};
use std::fs;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut data = vec![];

    let mut zeros = vec![];
    let mut nines = vec![];

    for (i, line) in file.lines().enumerate() {
        let mut row = vec![];
        for (j, chr) in line.bytes().enumerate() {
            if chr == b'0' {
                zeros.push((i, j));
            }
            if chr == b'9' {
                nines.push((i, j));
            }

            row.push((chr - b'0') as i32);
        }
        data.push(row);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for start in zeros {
        for end in &nines {
            if let Some(paths) = path_find(start, *end, &data) {
                part1 += 1;
                part2 += paths.0.into_iter().count();
            }
        }
    }
    println!("{}", part1);
    println!("{}", part2)
}

fn path_find(
    start: (usize, usize),
    end: (usize, usize),
    data: &Vec<Vec<i32>>,
) -> Option<(AstarSolution<(usize, usize)>, usize)> {
    astar::astar_bag(
        &start,
        |current| next_points(*current, data),
        |start| (start.0.abs_diff(end.0) + start.1.abs_diff(end.1)) / 3,
        |p| *p == end,
    )
}

fn next_points(current: (usize, usize), data: &Vec<Vec<i32>>) -> Vec<((usize, usize), usize)> {
    let (i, j) = current;
    let point = data[i][j];

    let mut succesors = vec![];
    NEXT.iter().for_each(|(i_1, j_1)| {
        let new_i = (i as i32 + i_1) as usize;
        let new_j = (j as i32 + j_1) as usize;
        if let Some(row) = data.get(new_i) {
            if let Some(next) = row.get(new_j) {
                if *next == point + 1 {
                    succesors.push(((new_i, new_j), 1));
                }
            }
        }
    });

    succesors
}
