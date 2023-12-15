use crate::day::Day;

pub struct Day03 {
    numbers: Vec<u16>,
    bit_count: u8,
}

fn get_bit(number: u16, i: u8) -> u8 {
    if number & (1 << i) > 0 {
        1
    } else {
        0
    }
}

fn count_ones(numbers: &[u16], i: u8) -> usize {
    numbers.iter().filter(|&&n| get_bit(n, i) == 1).count()
}

impl Day03 {
    fn find_by_bit_criteria(&self, keep_most_common: bool) -> usize {
        (0..self.bit_count).rev().fold(self.numbers.clone(), |candidates, i| {
            if candidates.len() == 1 {
                candidates
            } else {
                let most_common = if count_ones(&candidates, i) as f32 >= candidates.len() as f32 / 2f32 {
                    1
                } else {
                    0
                };
                candidates
                    .into_iter()
                    .filter(|&n| keep_most_common == (get_bit(n, i) == most_common))
                    .collect()
            }
        })[0] as usize
    }
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
            .filter(|&i| count_ones(&self.numbers, i) > number_counts / 2)
            .fold(0, |gamma, i| gamma | (1 << i));
        gamma * (!gamma & ((1 << self.bit_count) - 1))
    }

    fn part_2(&self) -> Self::T2 {
        self.find_by_bit_criteria(true) * self.find_by_bit_criteria(false)
    }
}