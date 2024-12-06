use std::fs;

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();

    let mut data = file
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect_vec();

    let size = (data[0].len(), data.len());
    let mut guard = (0, 0);
    data.iter_mut().enumerate().for_each(|(i, line)| {
        line.iter_mut().enumerate().for_each(|(j, x)| match x {
            b'^' | b'>' | b'<' | b'v' => {
                guard = (i, j);
            }
            _ => {}
        })
    });

    let part1 = possible_positions(data.clone(), size, guard).unwrap();

    let result = part1
        .iter()
        .map(|x| *x.iter().counts_by(|x| *x == b'X').get(&true).unwrap_or(&0))
        .sum::<usize>();
    println!("{}", result);

    let mut part2 = 0usize;
    for i in 0..size.1 {
        for j in 0..size.0 {
            let mut cloned = data.clone();
            if part1[i][j] != b'X' || (i == guard.0 && j == guard.1) {
                continue;
            }

            cloned[i][j] = b'#';
            if let None = possible_positions(cloned, size, guard) {
                part2 += 1;
            }
        }
    }

    println!("{}", part2-1);
}

fn possible_positions(
    mut data: Vec<Vec<u8>>,
    size: (usize, usize),
    mut guard: (usize, usize),
) -> Option<Vec<Vec<u8>>> {
    let mut history = vec![];
    history.push((guard, b'^'));
    'l1: loop {
        let value = match data[guard.0][guard.1] {
            b'^' => (-1, 0, b'>'),
            b'>' => (0, 1, b'v'),
            b'v' => (1, 0, b'<'),
            b'<' => (0, -1, b'^'),
            _ => panic!("thats not guard"),
        };

        let next = (
            (guard.0 as i32 + value.0) as usize,
            (guard.1 as i32 + value.1) as usize,
        );

        if next.0 >= size.0 || next.1 as usize >= size.1 {
            break 'l1;
        }

        if data[next.0][next.1] == b'#' {
            if history.iter().contains(&(guard, data[guard.0][guard.1])) {
                return None;
            }
            history.push((guard, data[guard.0][guard.1]));
            data[guard.0][guard.1] = value.2;
        } else if data[next.0][next.1] == b'.' || data[next.0][next.1] == b'X' {
            let symbol = data[guard.0][guard.1];
            data[guard.0][guard.1] = b'X';
            guard.0 = next.0;
            guard.1 = next.1;
            data[guard.0][guard.1] = symbol;
        }
    }

    data[guard.0][guard.1] = b'X';

    Some(data)
}
