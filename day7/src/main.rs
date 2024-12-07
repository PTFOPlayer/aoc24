use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut data = Vec::new();

    for line in file.lines() {
        let mut splitted = line.split_whitespace();
        let fst = splitted.next().unwrap();
        let fst = fst[0..fst.len() - 1].parse::<i64>().unwrap();

        let mut v = Vec::new();
        for num in splitted {
            v.push(num.parse::<i64>().unwrap());
        }

        data.push((fst, v))
    }

    let part1 = solve(&data, 2);
    println!("{}", part1);
    let part2 = solve(&data, 3);
    println!("{}", part2);
}

fn solve(data: &[(i64, Vec<i64>)], base: i64) -> i64 {
    data.par_iter()
        .map(|(target, queue)| {
            let queue_len = queue.len();
            let mut ops = vec![0; queue_len - 1];
            for variant in 0..i64::pow(base, (queue_len - 1) as u32) {
                to_base(variant, base, queue_len - 1, &mut ops);
                let fst = queue[0];
                let rhs = queue[1..queue_len]
                    .iter()
                    .zip(ops.iter())
                    .fold(fst, |lhs, (val, op)| match op {
                        b'0' => lhs + val,
                        b'1' => lhs * val,
                        b'2' => (lhs * count_places(*val)) + val,
                        _ => panic!("invalid operation"),
                    });
                if *target == rhs {
                    return *target;
                }
            }
            0
        })
        .sum()
}

fn count_places(mut val: i64) -> i64 {
    let mut result = 1;
    while val > 0 {
        result *= 10;
        val = val / 10;
    }
    result
}

fn to_base(mut val: i64, base: i64, padding: usize, target: &mut [u8]) {
    for i in 0..padding {
        if val > 0 {
            target[i] = (val % base) as u8 + b'0';
            val /= base;
        } else {
            target[i] = b'0';
        }
    }

    target.reverse();
}
