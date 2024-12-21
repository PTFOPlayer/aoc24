use itertools::Itertools;
use pathfinding::prelude::{astar_bag, AstarSolution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs;

//  +---+---+---+
//  | 7 | 8 | 9 |
//  +---+---+---+
//  | 4 | 5 | 6 |
//  +---+---+---+
//  | 1 | 2 | 3 |
//  +---+---+---+
//      | 0 | A |
//      +---+---+

//      +---+---+
//      | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let combinations = file.lines().collect_vec();

    let res_final: usize = combinations
        .into_par_iter()
        .map(|combination| {
            let mut len = usize::MAX;
            for sequence in unwind_numberpad(combination) {
                for sequence in unwind_arrowpad(sequence) {
                    for sequence in unwind_arrowpad(sequence) {
                        if sequence.len() < len {
                            len = sequence.len();
                        }
                    }
                }
            }
            let num = combination[0..combination.len() - 1]
                .parse::<usize>()
                .unwrap();
            let res = num * len;
            res
        })
        .sum();

    println!("{}", res_final)
}
fn unwind_numberpad(combination: &str) -> Vec<Vec<u8>> {
    let mut sequences: Vec<Vec<u8>> = vec![];
    let mut current = 10;
    for byte in combination.as_bytes() {
        let map = match byte {
            b'0'..=b'9' => byte - b'0',
            b'A' => 10,
            _ => panic!("on construct"),
        };
        let res = astar_bag(
            &(current, b'_'),
            |point| {
                let vaiable: &[(u8, u8)] = match point.0 {
                    10 => &[(0, b'<'), (3, b'^')],
                    0 => &[(2, b'^'), (10, b'>')],
                    1 => &[(2, b'>'), (4, b'^')],
                    2 => &[(0, b'v'), (3, b'>'), (1, b'<'), (5, b'^')],
                    3 => &[(6, b'^'), (2, b'<'), (10, b'v')],
                    4 => &[(1, b'v'), (5, b'>'), (7, b'^')],
                    5 => &[(2, b'v'), (6, b'>'), (4, b'<'), (8, b'^')],
                    6 => &[(9, b'^'), (5, b'<'), (3, b'v')],
                    7 => &[(4, b'v'), (8, b'>')],
                    8 => &[(7, b'<'), (5, b'v'), (9, b'>')],
                    9 => &[(8, b'<'), (6, b'v')],
                    _ => panic!("on match"),
                };
                vaiable.iter().map(|v| (*v, 1)).collect_vec()
            },
            |_| 0,
            |p| p.0 == map,
        )
        .unwrap()
        .0;
        current = map;
        if sequences.len() == 0 {
            sequences.append(
                &mut res
                    .map(|v| v[1..].iter().map(|x| x.1).chain(vec![b'A']).collect_vec())
                    .collect_vec(),
            );
        } else {
            sequences = sequences
                .iter()
                .cartesian_product(res)
                .map(|(seq, new)| {
                    let mut v = seq.clone();
                    let mut new = new[1..].iter().map(|x| x.1).chain(vec![b'A']).collect_vec();

                    v.append(&mut new);
                    v
                })
                .collect_vec();
        }
    }

    let shortest = sequences.iter().fold(
        usize::MAX,
        |lhs, rhs| if lhs > rhs.len() { rhs.len() } else { lhs },
    );

    sequences
        .into_iter()
        .filter(|l| l.len() == shortest)
        .collect_vec()
}

fn unwind_arrowpad(combination: Vec<u8>) -> Vec<Vec<u8>> {
    let mut sequences: Vec<Vec<u8>> = vec![];
    let mut current = b'A';
    for byte in combination {
        let res = arrowpad_path(current, byte);
        current = byte;
        if sequences.len() == 0 {
            sequences = res
                .map(|v| {
                    v.iter()
                        .skip(1)
                        .map(|x| x.1)
                        .chain(vec![b'A'])
                        .collect_vec()
                })
                .collect_vec()
        } else {
            sequences = sequences
                .into_iter()
                .cartesian_product(res)
                .map(|(mut seq, new)| {
                    let mut new = new.iter().skip(1).map(|x| x.1).collect_vec();
                    seq.append(&mut new);
                    seq.push(b'A');
                    seq
                })
                .collect_vec();
        }
    }
    let shortest = sequences.iter().fold(
        usize::MAX,
        |lhs, rhs| if lhs > rhs.len() { rhs.len() } else { lhs },
    );

    sequences
        .into_iter()
        .filter(|l| l.len() == shortest)
        .collect_vec()
}

#[memoize::memoize]
fn arrowpad_path(current: u8, target: u8) -> AstarSolution<(u8, u8)> {
    astar_bag(
        &(current, b'_'),
        |point| {
            let vaiable: &[(u8, u8)] = match point.0 {
                b'A' => &[(b'^', b'<'), (b'>', b'v')],
                b'>' => &[(b'v', b'<'), (b'A', b'^')],
                b'v' => &[(b'<', b'<'), (b'^', b'^'), (b'>', b'>')],
                b'^' => &[(b'A', b'>'), (b'v', b'v')],
                b'<' => &[(b'v', b'>')],
                _ => panic!("on match"),
            };
            vaiable.iter().map(|v| (*v, 1))
        },
        |_| 0,
        |p| p.0 == target,
    )
    .unwrap()
    .0
}
