use std::{collections::HashMap, fs, sync::OnceLock};

static MODIFIERES: OnceLock<HashMap<u8, (i32, i32)>> = OnceLock::new();

fn main() {
    let _ = MODIFIERES.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(b'^', (-1, 0));
        map.insert(b'>', (0, 1));
        map.insert(b'<', (0, -1));
        map.insert(b'v', (1, 0));
        map
    });

    let file = fs::read_to_string("./data.txt").unwrap();

    let lines = file.lines();

    let mut map = vec![];
    let mut state = true;
    let mut moves = vec![];
    let mut robot = (0, 0);
    for (idx, line) in lines.enumerate() {
        if line == "" {
            state = false;
            continue;
        }
        if state {
            if let Some(row) = line.find('@') {
                robot = (idx, row);
            }
            map.push(line.as_bytes().to_vec());
        } else {
            moves.append(&mut line.as_bytes().to_vec());
        }
    }

    for direction in moves {
        move_robot(direction, &mut robot, &mut map);
    }

    let mut result = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == b'O' {
                result += (row_idx * 100) + col_idx
            }
        }
    }
    println!("{}", result)
}

fn move_robot(direction: u8, robot: &mut (usize, usize), map: &mut Vec<Vec<u8>>) {
    let direction = MODIFIERES.get().unwrap().get(&direction).unwrap();
    let next_pos = next_position(*robot, *direction);
    let next = map[next_pos.0][next_pos.1];
    if next != b'#' && (next == b'.' || check_recursive(next_pos, *direction, map)) {
        map[robot.0][robot.1] = b'.';
        *robot = next_pos;
        map[robot.0][robot.1] = b'@';
    }
}

fn check_recursive(current: (usize, usize), direction: (i32, i32), map: &mut Vec<Vec<u8>>) -> bool {
    let next_pos = next_position(current, direction);
    let cur = map[current.0][current.1];
    let next = map[next_pos.0][next_pos.1];
    if next == b'#' {
        return false;
    } else if next == b'.' || check_recursive(next_pos, direction, map) {
        map[next_pos.0][next_pos.1] = cur;
        map[current.0][current.1] = b'.';
        return true;
    }
    return false;
}

#[inline(always)]
fn next_position(current: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
    (
        (current.0 as i32 + direction.0) as usize,
        (current.1 as i32 + direction.1) as usize,
    )
}
