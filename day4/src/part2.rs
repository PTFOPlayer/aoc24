pub fn part2(file: &str) -> usize {
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
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let mid = bytes[(i * cols + j) as usize];
            if mid == b'A' {
                let lu = bytes[((i - 1) * cols + (j - 1)) as usize];
                let ru = bytes[((i - 1) * cols + (j + 1)) as usize];
                let ld = bytes[((i + 1) * cols + (j - 1)) as usize];
                let rd = bytes[((i + 1) * cols + (j + 1)) as usize];

                // M.M
                // .A.
                // S.S
                let cond1= lu == b'M' && rd == b'S' && ru == b'M' && ld == b'S';
                // S.M
                // .A.
                // S.M
                let cond2= lu == b'S' && rd == b'M' && ru == b'M' && ld == b'S';
                // M.S
                // .A.
                // M.S
                let cond3= lu == b'M' && rd == b'S' && ru == b'S' && ld == b'M';
                // S.S
                // .A.
                // M.M
                let cond4= lu == b'S' && rd == b'M' && ru == b'S' && ld == b'M';
                if cond1 || cond2 || cond3 || cond4 {
                    cnt+=1;
                }
            }
        }
    }

    cnt
}
