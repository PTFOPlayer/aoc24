use std::fs;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut cnt = 0;
    let mut cnt2 = 0;
    for line in file.lines() {
        let splitted = line.split_whitespace().collect::<Vec<_>>();
        cnt += process_line(&splitted);
        cnt2 += process_line_rm(&splitted);
    }

    println!("{}, {}", cnt, cnt2);
}

fn process_line(splitted: &[&str]) -> i32 {
    let mut splitted = splitted.iter();
    let mut safe = true;
    let mut direction = None;
    let mut prev: i32 = splitted.next().unwrap().parse().unwrap();
    for elem in splitted {
        let elem = elem.parse::<i32>().unwrap();
        if let None = direction {
            direction = Some((elem - prev).is_positive());
        }

        let mut distance = elem - prev;
        let is_pos = distance.is_positive();
        distance = distance.abs();

        prev = elem;
        if distance > 3 || distance < 1 || is_pos != direction.unwrap() {
            safe = false
        }
    }

    safe as i32
}

fn process_line_rm(splitted: &[&str]) -> i32 {
    if process_line(splitted) == 1 {
        return 1;
    }

    for i in 0..splitted.len() {
        let mut temp = splitted.to_vec();
        temp.remove(i);
        if process_line(&temp) == 1 {
            return 1;
        }
    }

    0
}
