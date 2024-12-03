use std::fs;

use regex::Regex;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let reg = Regex::new("mul\\(\\d*,\\d*\\)").unwrap();
    let inner_reg = Regex::new("\\d*,\\d*").unwrap();
    let data = reg.find_iter(&file);

    let mut result = 0;
    for d in data {
        let mut res = inner_reg.find(d.as_str()).unwrap().as_str().split(',');
        let lhs = res.next().unwrap().parse::<i64>().unwrap();
        let rhs = res.next().unwrap().parse::<i64>().unwrap();
        result += lhs * rhs;
    }

    println!("{}", result);

    let reg = Regex::new("(mul\\(\\d*,\\d*\\))|(don't\\(\\))|(do\\(\\))").unwrap();
    let inner_reg = Regex::new("\\d*,\\d*").unwrap();
    let mut lock = false;
    let data = reg.find_iter(&file);

    let mut result = 0;
    for d in data {
        let str = d.as_str();
        if str == "do()" {
            lock = false;
        } else if str == "don't()" {
            lock = true
        } else if !lock {
            let mut res = inner_reg.find(d.as_str()).unwrap().as_str().split(',');
            let lhs = res.next().unwrap().parse::<i64>().unwrap();
            let rhs = res.next().unwrap().parse::<i64>().unwrap();
            result += lhs * rhs;
        }
    }

    println!("{}", result);
}
