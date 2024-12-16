use itertools::Itertools;
use pathfinding::prelude::astar_bag;
use std::fs;

const MOVES: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Point {
    pos: (usize, usize),
    direction: Direction,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
#[repr(usize)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut map = vec![];
    let mut start = None;
    let mut end = None;
    for (i, line) in file.lines().enumerate() {
        let mut row = vec![];
        for (j, byte) in line.as_bytes().iter().enumerate() {
            if *byte == b'S' {
                start = Some((i, j))
            }
            if *byte == b'E' {
                end = Some((i, j))
            }
            row.push(*byte);
        }
        map.push(row);
    }

    let start = Point {
        pos: start.unwrap(),
        direction: Direction::Right,
    };
    let end = end.unwrap();

    let res = astar_bag(
        &start,
        |point| next_position(point, &map),
        |point| ((point.pos.0.abs_diff(end.0)) + (point.pos.1.abs_diff(end.1))) / 3,
        |success| success.pos == end,
    );

    if let Some(res) = res {
        println!("part1: {}", res.1);
        println!("part2: {}", res.0.into_iter().flatten().map(|point| point.pos).unique().count());
        
    }
}

fn next_position(point: &Point, map: &Vec<Vec<u8>>) -> Vec<(Point, usize)> {
    let mut v = vec![];
    match point.direction {
        Direction::Left | Direction::Right => {
            v.push((
                Point {
                    pos: point.pos,
                    direction: Direction::Down,
                },
                1000,
            ));
            v.push((
                Point {
                    pos: point.pos,
                    direction: Direction::Up,
                },
                1000,
            ));
        }
        Direction::Up | Direction::Down => {
            v.push((
                Point {
                    pos: point.pos,
                    direction: Direction::Left,
                },
                1000,
            ));
            v.push((
                Point {
                    pos: point.pos,
                    direction: Direction::Right,
                },
                1000,
            ));
        }
    }
    let n = next(point.pos, MOVES[point.direction as usize]);
    if map[n.0][n.1] != b'#' {
        v.push((
            Point {
                pos: next(point.pos, MOVES[point.direction as usize]),
                direction: point.direction,
            },
            1,
        ));
    }
    v
}

#[inline(always)]
fn next(point: (usize, usize), by: (i32, i32)) -> (usize, usize) {
    (
        (point.0 as i32 + by.0) as usize,
        (point.1 as i32 + by.1) as usize,
    )
}
