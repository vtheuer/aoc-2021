use crate::day::Day;

pub struct Day03 {
    numbers: Vec<u16>,
    bit_count: u8,
}

impl Day<'_> for Day03 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            numbers: input
                .lines()
                .map(|l| l.bytes().fold(0, |acc, b| acc << 1 | (if b == b'1' { 1 } else { 0 })))
                .collect(),
            bit_count: input.lines().next().unwrap().len() as u8,
        }
    }

    fn part_1(&self) -> Self::T1 {
        let number_counts = self.numbers.len();
        let gamma = (0..self.bit_count)
            .filter(|i| self.numbers.iter().filter(|&&n| n & (1 << i) > 0).count() > number_counts / 2)
            .fold(0, |gamma, i| gamma | (1 << i));
        gamma * (!gamma & ((1 << self.bit_count) - 1))
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
