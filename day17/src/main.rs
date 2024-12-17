use std::{fs, str::Lines};

use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("./data.txt").unwrap();

    let mut lines: Lines<'_> = file.lines();

    let regs = [
        parse_reg(&mut lines),
        parse_reg(&mut lines),
        parse_reg(&mut lines),
    ];

    _ = lines.next();

    let program = parse_program(&mut lines);

    let mut cpu = CPU {
        pc: 0,
        regs,
        program,
        output_buffer: vec![],
    };
    println!("{:?}", cpu);

    while cpu.tick() {}

    println!(
        "part1: {}",
        cpu.output_buffer
            .iter()
            .map(|v| char::from(*v as u8 + b'0'))
            .join(",")
    );
}

#[derive(Debug, Clone)]
struct CPU {
    pc: usize,
    regs: [usize; 3],
    program: Vec<usize>,
    output_buffer: Vec<u8>,
}

impl CPU {
    pub fn tick(&mut self) -> bool {
        let instruction = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        let combo = match operand {
            0..4 => operand,
            4..7 => self.regs[operand as usize - 4],
            _ => panic!("non existing opperand"),
        };

        match instruction {
            /*adv*/ 0 => self.regs[0] = self.regs[0] / (1 << combo),
            /*bxl*/ 1 => self.regs[1] ^= operand,
            /*bst*/ 2 => self.regs[1] = combo & 0b111,
            /*jnz*/
            3 => {
                if self.regs[0] != 0 {
                    self.pc = operand as usize;
                    return self.pc < self.program.len();
                }
            }
            /*bxc*/ 4 => self.regs[1] = self.regs[1] ^ self.regs[2],
            /*out*/ 5 => self.output_buffer.push((combo & 0b111) as u8),
            /*bdv*/ 6 => self.regs[1] = self.regs[0] / (1 << combo),
            /*cdv*/ 7 => self.regs[2] = self.regs[0] / (1 << combo),
            _ => panic!("non existing instruction"),
        }
        self.pc += 2;

        self.pc < self.program.len()
    }
}

fn parse_reg(lines: &mut Lines<'_>) -> usize {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_program(lines: &mut Lines<'_>) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect_vec()
}
