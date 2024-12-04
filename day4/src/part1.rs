use crate::Vec2;

fn process(bytes: &[u8], direction: Vec2, size: Vec2, limits: Vec2, start: Vec2) -> usize {
    let xmas = b"XMAS";
    let rxmas = b"SAMX";

    let (modi, modj) = (direction.0, direction.1);
    let (col, _) = (size.0, size.1);
    let (starti, startj) = (start.0, start.1);
    let (lcol, lrow) = (limits.0, limits.1);

    let mut cnt = 0;
    for i in starti..lrow {
        for j in startj..lcol {
            let bytes: [u8; 4] = [
                bytes[(((i + (modi * 0)) * col) + (j + (modj * 0))) as usize],
                bytes[(((i + (modi * 1)) * col) + (j + (modj * 1))) as usize],
                bytes[(((i + (modi * 2)) * col) + (j + (modj * 2))) as usize],
                bytes[(((i + (modi * 3)) * col) + (j + (modj * 3))) as usize],
            ];

            if &bytes == xmas || &bytes == rxmas {
                // println!(
                //     "({},{}) ({},{}) ({},{}) ({},{}), {}",
                //     (i + (modi * 0)),
                //     (j + (modj * 0)),
                //     (i + (modi * 1)),
                //     (j + (modj * 1)),
                //     (i + (modi * 2)),
                //     (j + (modj * 2)),
                //     (i + (modi * 3)),
                //     (j + (modj * 3)),
                //     String::from_utf8_lossy(&bytes)
                // );
                cnt += 1
            }
        }
    }

    cnt
}

pub fn part1(file: &str) -> usize {
    let mut lines = file.lines();
    let fst = lines.next().unwrap();
    let mut bytes = fst.as_bytes().to_vec();
    let columns = fst.len();

    for line in lines {
        assert!(line.len() == columns);
        bytes.append(&mut line.as_bytes().to_vec())
    }

    let rows = (bytes.len() / columns) as isize;
    let cols = columns as isize;

    let mut cnt = 0;
    cnt += process(
        &bytes,
        Vec2(0, 1),
        Vec2(cols, rows),
        Vec2(cols - 3, rows),
        Vec2::zero(),
    );
    cnt += process(
        &bytes,
        Vec2(1, 0),
        Vec2(cols, rows),
        Vec2(cols, rows - 3),
        Vec2::zero(),
    );
    cnt += process(
        &bytes,
        Vec2(1, 1),
        Vec2(cols, rows),
        Vec2(cols - 3, rows - 3),
        Vec2::zero(),
    );
    cnt += process(
        &bytes,
        Vec2(-1, 1),
        Vec2(cols, rows),
        Vec2(cols, rows),
        Vec2(3, 0),
    );
    return cnt;
}
