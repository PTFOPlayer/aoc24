use std::fs;

use regex::Regex;

const LEFT_IGNORE: usize = "mul(".len();

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let reg = Regex::new("mul\\(\\d*,\\d*\\)").unwrap();
    let data = reg.find_iter(&file);

    let mut result = 0;
    for d in data {
        let str = d.as_str();
        result += str[LEFT_IGNORE..str.len() - 1]
            .split(',')
            .fold(1, |lhs, rhs| lhs * rhs.parse::<i64>().unwrap());
    }

    println!("{}", result);

    let reg = Regex::new("(mul\\(\\d*,\\d*\\))|(don't\\(\\))|(do\\(\\))").unwrap();
    let data = reg.find_iter(&file);

    let mut lock = 1;
    let mut result = 0;
    for d in data {
        let str = d.as_str();
        match str {
            "do()" => lock = 1,
            "don't()" => lock = 0,
            _ => {
                result += lock
                    * str[LEFT_IGNORE..str.len() - 1]
                        .split(',')
                        .fold(1, |lhs, rhs| lhs * rhs.parse::<i64>().unwrap());
            }
        }
    }

    println!("{}", result);
}
