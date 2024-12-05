use ahash::AHashMap;
use itertools::Itertools;
use std::fs;
fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut rules: AHashMap<i32, Vec<i32>> = AHashMap::new();
    let mut lines = file.lines();

    let mut line = lines.next().unwrap();
    while line != "" {
        let (lhs, rhs): (i32, i32) = line
            .split('|')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if let Some(v) = rules.get_mut(&rhs) {
            v.push(lhs);
        } else {
            rules.insert(rhs, vec![lhs]);
        }

        line = lines.next().unwrap();
    }

    let mut data = vec![];
    for line in lines {
        data.push(line);
    }

    println!("{:?}", rules);
    println!("{:?}", data);

    let mut sum = 0;
    'outer: for page in data {
        let page = page
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect_vec();

        'inner: for i in 0..page.len() {
            let Some(segment) = rules.get(&page[i]) else {
                continue 'inner;
            };

            for j in i+1..page.len() {
                if segment.iter().contains(&page[j])  {
                    continue 'outer;
                }
            }
        }
        sum += page[page.len() / 2];
    }

    println!("{}", sum)
}
