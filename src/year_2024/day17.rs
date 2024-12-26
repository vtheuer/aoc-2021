use crate::day::Day;
use crate::util::Joinable;
use num::Integer;

pub struct Day17 {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<u8>,
}

fn combo(operand: u8, a: usize, b: usize, c: usize) -> usize {
    match operand {
        0..=3 => operand as usize,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

fn dv(a: usize, b: usize, c: usize, operand: u8) -> usize {
    2usize.pow(combo(operand, a, b, c) as u32)
}

impl Day17 {
    fn compute(&self, initial: usize) -> Vec<u8> {
        let mut a = initial;
        let mut b = self.b;
        let mut c = self.c;
        let mut pointer = 0;
        let mut output = Vec::new();

        while pointer < self.instructions.len() {
            let operand = self.instructions[pointer + 1];
            let mut next = pointer + 2;
            match self.instructions[pointer] {
                0 => a /= dv(a, b, c, operand),
                1 => b ^= operand as usize,
                2 => b = combo(operand, a, b, c) % 8,
                3 => {
                    if a != 0 {
                        next = operand as usize
                    }
                }
                4 => b ^= c,
                5 => output.push((combo(operand, a, b, c) % 8) as u8),
                6 => b = a / dv(a, b, c, operand),
                7 => c = a / dv(a, b, c, operand),
                _ => unreachable!(),
            }
            pointer = next;
        }

        output
    }
}

impl Day17 {
    fn solve(&self, i: usize, current: usize) -> Option<usize> {
        // 2,4,
        // b = a % 8
        // 1,7,
        // b = b ^ 7
        // 7,5,
        // c = a / 2.pow(b)
        // 1,7,
        // b = b ^ 7
        // 0,3,
        // a = a / 2.pow(3)
        // 4,1,
        // b = b ^ c
        // 5,5,
        // ouput += b
        // 3,0
        // if a == 0 { break }

        // while a != 0 {
        // b = (a % 8) ^ 7
        // c = a / 2.pow(b)
        // b = b ^ 7
        // a = a / 8
        // b = b ^ c
        // ouput += b
        // }

        // while a != 0 {
        //   b = 7 - (a % 8) // 0 <= b <= 7
        //   ouput += (7 - b) ^ (a / 2.pow(b))
        //   a = a / 8
        // }
        for j in 0..8 {
            let a = current + j * 8usize.pow(i as u32);
            let output = self.compute(a);
            if output.len() == self.instructions.len() && output[i..] == self.instructions[i..] {
                if i == 0 {
                    return Some(a);
                } else if let Some(r) = self.solve(i - 1, a) {
                    return Some(r);
                }
            }
        }

        None
    }
}

impl Day<'_> for Day17 {
    type T1 = String;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut lines = input.lines().filter_map(|l| Some(l.split_once(": ")?.1));
        Self {
            a: lines.next().map(|r| r.parse().unwrap()).unwrap(),
            b: lines.next().map(|r| r.parse().unwrap()).unwrap(),
            c: lines.next().map(|r| r.parse().unwrap()).unwrap(),
            instructions: lines.next().unwrap().split(',').map(|r| r.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut pointer = 0;
        let mut output = Vec::new();

        while pointer < self.instructions.len() {
            let operand = self.instructions[pointer + 1];
            let mut next = pointer + 2;
            match self.instructions[pointer] {
                0 => a /= dv(a, b, c, operand),
                1 => b ^= operand as usize,
                2 => b = combo(operand, a, b, c) % 8,
                3 => {
                    if a != 0 {
                        next = operand as usize
                    }
                }
                4 => b ^= c,
                5 => output.push((combo(operand, a, b, c) % 8) as u8),
                6 => b = a / dv(a, b, c, operand),
                7 => c = a / dv(a, b, c, operand),
                _ => unreachable!(),
            }
            pointer = next;
        }

        output.into_iter().map(|e| (b'0' + e) as char).join(",")
    }

    fn part_2(&self) -> Self::T2 {
        self.solve(self.instructions.len() - 1, 0).unwrap()
    }
}
