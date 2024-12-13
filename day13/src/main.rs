use std::fs;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    println!("part1: {}", solve(&file, 0.0));
    println!("part2: {}", solve(&file, 10000000000000.0));
}


fn solve(file: &str, mod_p: f64) -> i64 {
    let x_regex = regex::Regex::new("X[+=]\\d*").unwrap();
    let y_regex = regex::Regex::new("Y[+=]\\d*").unwrap();
    file
        .lines()
        .filter(|x| *x != "")
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chunk = chunk.collect_vec();
            let ax = match_reg(&x_regex, chunk[0]);
            let ay = match_reg(&y_regex, chunk[0]);
            let bx = match_reg(&x_regex, chunk[1]);
            let by = match_reg(&y_regex, chunk[1]);
            let px = match_reg(&x_regex, chunk[2])+mod_p;
            let py = match_reg(&y_regex, chunk[2])+mod_p;

            //      pxby-pybx
            //ma = -----------
            //      byax-bxay
            //
            //      pxay-pyax
            //mb = -----------
            //      bxay-byax
            let ma = ((px * by) - (py * bx)) / ((by * ax) - (bx * ay));
            let mb = ((px * ay) - (py * ax)) / ((bx * ay) - (by * ax));
            let tok = if ma == f64::floor(ma) && mb == f64::floor(mb) {
                Some((ma * 3.0 + mb) as i64)
            } else {
                None
            };
            // println!(
                // "ax{} ay{}\tbx{} by{}\tpx{} py{}\t ma{:.2} mb{:.2}\t tok{:?}",
                // ax, ay, bx, by, px, py, ma, mb, tok
            // );
            tok
        })
        .filter_map(|t| t)
        .sum::<i64>()
}
fn match_reg(reg: &Regex, str: &str) -> f64 {
    reg.find(str).unwrap().as_str()[2..].parse().unwrap()
}
