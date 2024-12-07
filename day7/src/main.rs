use std::{collections::VecDeque, fs};

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut data = Vec::new();

    for line in file.lines() {
        let mut splitted = line.split_whitespace();
        let fst = splitted.next().unwrap();
        let fst = fst[0..fst.len() - 1].parse::<i64>().unwrap();

        let mut v = VecDeque::new();
        for num in splitted {
            v.push_back(num.parse::<i64>().unwrap());
        }

        data.push((fst, v))
    }

    let part1 = solve(&data, 2);
    println!("{}", part1);
    let part2 = solve(&data, 3);
    println!("{}", part2);
}

fn solve(data: &[(i64, VecDeque<i64>)], base: i64) -> i64 {
    let mut result = 0;
    for (target, queue) in data {
        let queue_len = queue.len();
        let variants = i64::pow(base, (queue_len - 1) as u32);
        'variant_loop: for variant in 0..variants {
            let mut queue = queue.clone();
            let ops = &to_base(variant, base, queue_len - 1);
            let fst = queue.pop_front().unwrap();
            let rhs =
                queue
                    .into_iter()
                    .zip(ops.into_iter())
                    .fold(fst, |lhs, (val, op)| match *op {
                        b'0' => lhs + val,
                        b'1' => lhs * val,
                        b'2' => (lhs * count_places(val)) + val,
                        _ => panic!("invalid operation"),
                    });
            if *target == rhs {
                result += target;
                break 'variant_loop;
            }
        }
    }
    result
}

fn count_places(mut val: i64) -> i64 {
    let mut result = 1;
    while val > 0 {
        result *= 10;
        val = val / 10;
    }
    result
}

fn to_base(mut val: i64, base: i64, mut padding: usize) -> Vec<u8> {
    let mut data = vec![];

    while val > 0 {
        data.push((val % base) as u8 + b'0');
        val /= base;
    }
    padding = padding - data.len();

    for _ in 0..padding {
        data.push(b'0');
    }

    data.reverse();
    data
}
