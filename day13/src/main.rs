use std::{fs, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    println!("part1: {}", solve(&file, 0.0));
    println!("part2: {}", solve(&file, 10000000000000.0));
}

fn solve(file: &str, mod_p: f64) -> f64 {
    file.lines()
        .filter(|x| *x != "")
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let (ax, ay, bx, by, px, py) = match_reg(&chunk.collect_vec().concat());
            let (px, py) = (px + mod_p, py + mod_p);

            //      pxby-pybx
            //ma = -----------
            //      byax-bxay
            //
            //      pxay-pyax
            //mb = -----------
            //      bxay-byax
            let ma = ((px * by) - (py * bx)) / ((by * ax) - (bx * ay));
            let mb = ((px * ay) - (py * ax)) / ((bx * ay) - (by * ax));
            // println!(
            // "ax{} ay{}\tbx{} by{}\tpx{} py{}\t ma{:.2} mb{:.2}\t tok{:?}",
            // ax, ay, bx, by, px, py, ma, mb, tok
            // );
            
            if ma == f64::floor(ma) && mb == f64::floor(mb) {
                Some(ma * 3.0 + mb)
            } else {
                None
            }
        })
        .filter_map(|t| t)
        .sum::<f64>()
}

static REG: OnceLock<Regex> = OnceLock::new();

#[inline(always)]
fn match_reg(str: &str) -> (f64, f64, f64, f64, f64, f64) {
    let reg = REG.get_or_init(||Regex::new("[XY][+=]\\d*").unwrap());
    reg.find_iter(str)
        .map(|x| x.as_str()[2..].parse().unwrap())
        .collect_tuple()
        .unwrap()
}
