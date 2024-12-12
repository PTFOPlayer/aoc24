use std::{
    fs,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut data = Data::new();

    for row in file.lines() {
        data.push(row.bytes().map(|byte| (0, byte)).collect_vec());
    }

    let mut result = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if let Some((visited, byte)) = data.get(i, j) {
                if visited == 0 {
                    let mut size = 0;
                    let mut edges = 0;
                    search_recursive(&mut data, &mut size, &mut edges, i, j, byte);
                    result += size * edges
                }
            }
        }
    }

    println!("{}", result);
}

fn search_recursive(
    data: &mut Data,
    size: &mut i64,
    edges: &mut i64,
    i: usize,
    j: usize,
    byte: u8,
) {
    *size += 1;
    data.get_mut(i, j).unwrap().0 = 1;

    let ops: [(
        Option<fn(usize, usize) -> usize>,
        Option<fn(usize, usize) -> usize>,
    ); 4] = [
        (None, Some(usize::wrapping_sub)),
        (None, Some(usize::wrapping_add)),
        (Some(usize::wrapping_sub), None),
        (Some(usize::wrapping_add), None),
    ];

    for (opi, opj) in ops {
        let new_i = opi.map_or(i, |op| op(i, 1));
        let new_j = opj.map_or(j, |op| op(j, 1));
        if let Some((visited, new_byte)) = data.get(new_i, new_j) {
            if new_byte == byte && visited == 0 {
                search_recursive(data, size, edges, new_i, new_j, byte);
            } else if new_byte != byte {
                *edges += 1;
            }
        } else {
            *edges += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    data: Vec<Vec<(u8, u8)>>,
}

impl Data {
    pub fn new() -> Self {
        Data { data: vec![] }
    }

    pub fn get(&mut self, i: usize, j: usize) -> Option<(u8, u8)> {
        self.data.get(i)?.get(j).copied()
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut (u8, u8)> {
        self.data.get_mut(i)?.get_mut(j)
    }
}

impl Deref for Data {
    type Target = Vec<Vec<(u8, u8)>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
