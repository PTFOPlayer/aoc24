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

    print_state(&data);

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
    print_state(&data);

    let result = data
        .iter()
        .map(|x| *x.iter().counts_by(|x| *x == b'X').get(&true).unwrap_or(&0))
        .sum::<usize>();
    println!("{}", result);
}

fn print_state(data: &Vec<Vec<u8>>) {
    for line in data {
        for elem in line {
            print!("{}", char::from(*elem))
        }
        println!();
    }

    println!();
}
