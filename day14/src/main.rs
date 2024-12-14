use std::{fs, io::Write};

use itertools::Itertools;
use regex::Match;

const LIMITS: (i16, i16) = (101, 103);

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();
    let reg = regex::Regex::new("[-]?\\d+").unwrap();
    let org_robots = file
        .lines()
        .map(|line| Robot::from(reg.find_iter(line).collect_vec().as_slice()))
        .collect_vec();

    let mut robots = org_robots.clone();

    let (limit_x, limit_y) = LIMITS;
    for _tick in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.tick_robot(limit_x, limit_y));
    }

    let mut arr = [[0usize; LIMITS.0 as usize]; LIMITS.1 as usize];

    for robot in robots {
        let x = usize::try_from(robot.x_pos).unwrap();
        let y = usize::try_from(robot.y_pos).unwrap();
        arr[y][x] += 1;
    }

    let mut quads = [0, 0, 0, 0];
    let mut idx = 0;
    for j in 0..limit_y {
        if j == limit_y / 2 {
            idx += 2;
        }
        let mut tmp_idx = idx;

        for i in 0..limit_x {
            if i == limit_x / 2 {
                tmp_idx += 1;
                // print!("  ")
            } else if j != limit_y / 2 {
                quads[tmp_idx] += arr[j as usize][i as usize];
                // print!("{} ", arr[j as usize][i as usize]);
            }
        }
        // println!()
    }

    println!("quads: {:?}", quads);
    println!("part1: {}", quads.iter().fold(1, |lhs, rhs| lhs * rhs));
    process_tree(org_robots);
}

#[derive(Clone, Copy)]
struct Robot {
    x_pos: i16,
    y_pos: i16,
    x_vel: i16,
    y_vel: i16,
}

impl From<&[Match<'_>]> for Robot {
    fn from(value: &[Match<'_>]) -> Self {
        Robot {
            x_pos: value[0].as_str().parse().unwrap(),
            y_pos: value[1].as_str().parse().unwrap(),
            x_vel: value[2].as_str().parse().unwrap(),
            y_vel: value[3].as_str().parse().unwrap(),
        }
    }
}

impl Robot {
    pub fn tick_robot(&mut self, limit_x: i16, limit_y: i16) {
        self.x_pos = sum_wrapped_to(self.x_pos, self.x_vel, limit_x);
        self.y_pos = sum_wrapped_to(self.y_pos, self.y_vel, limit_y);
    }

    pub fn distance(&self, other: &Robot) -> f64 {
        f64::sqrt(((self.x_pos - other.x_pos).pow(2) + (self.y_pos - other.y_pos).pow(2)) as f64)
    }

    pub fn get_idx(&self) -> (usize, usize) {
        (self.x_pos as usize, self.y_pos as usize)
    }
}

fn sum_wrapped_to(lhs: i16, rhs: i16, wrapped_to: i16) -> i16 {
    let res = (lhs + rhs) % wrapped_to;
    if res < 0 {
        res + wrapped_to
    } else {
        res
    }
}

fn process_tree(mut robots: Vec<Robot>) {
    let (limit_x, limit_y) = LIMITS;
    // assumption that entropy of tree will be lower than 80% od starting entropy
    let mut init_entropy = get_entropy(&robots) * 0.80;
    let mut target_tick = 0;
    let mut mods = 0;
    let mut arr = [[b' '; LIMITS.0 as usize]; LIMITS.1 as usize];
    // assumption it will be under 10000 seconds
    for tick in 0..10000 {
        robots
            .iter_mut()
            .for_each(|robot| robot.tick_robot(limit_x, limit_y));
        let entropy = get_entropy(&robots);

        if entropy < init_entropy {
            init_entropy = entropy;
            target_tick = tick + 1;
            mods += 1;

            arr.iter_mut()
                .for_each(|seg| seg.iter_mut().for_each(|x| *x = b' '));

            for robot in &robots {
                let (x, y) = robot.get_idx();
                arr[y][x] = b'#';
            }
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("./tree.txt")
                .unwrap();
            let joinked = arr.clone().join(&b'\n');
            file.write_all(&joinked).unwrap();
        }
    }
    println!("part2: {}, after entropy changes: {}", target_tick, mods);
}

fn get_entropy(robots: &[Robot]) -> f64 {
    robots
        .iter()
        .cartesian_product(robots.iter())
        .map(|(lhs, rhs)| lhs.distance(rhs))
        .sum::<f64>()
        / robots.len() as f64
}
