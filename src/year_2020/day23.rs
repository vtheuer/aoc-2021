use crate::day::Day;
use itertools::Itertools;

pub struct Day23 {
    numbers: Vec<u8>,
}

fn play(initial: &Vec<u8>, cup_count: usize, turns: usize) -> impl Iterator<Item = usize> {
    let mut numbers = (0..cup_count).collect::<Vec<_>>();
    for (i, &n) in initial.iter().enumerate() {
        numbers[i] = n as usize;
    }
    let min = numbers.iter().map(|&n| n).min().unwrap();
    let max = numbers.iter().map(|&n| n).max().unwrap();
    let mut current = 0;
    for _ in 0..turns {
        let current_value = numbers[current];
        let pickup = (1..=3)
            .map(|i| (current + i) % numbers.len())
            .map(|i| numbers[i])
            .collect::<Vec<_>>();
        let mut destination = current_value;
        while destination == current_value || pickup.contains(&destination) {
            destination = if destination > min {
                destination - 1
            } else {
                max
            };
        }

        for _ in 0..3 {
            numbers.remove(if current < numbers.len() - 1 {
                current + 1
            } else {
                0
            });
        }

        let destination_index = numbers
            .iter()
            .enumerate()
            .find(|&(_, &n)| n == destination)
            .map(|(i, _)| i)
            .expect(&format!("{} not found in {:?}", destination, numbers));
        for (i, n) in pickup.into_iter().enumerate() {
            numbers.insert(destination_index + i + 1, n);
        }

        current = numbers
            .iter()
            .enumerate()
            .find(|&(_, &n)| n == current_value)
            .map(|(i, _)| (i + 1) % numbers.len())
            .unwrap();
    }

    let index_of_1 = numbers
        .iter()
        .enumerate()
        .find(|&(_, &n)| n == 1)
        .map(|(i, _)| i)
        .unwrap();

    let len = numbers.len();
    (0..initial.len() - 1)
        .map(move |i| (index_of_1 + 1 + i) % len)
        .map(move |i| numbers[i])
}

impl Day<'_> for Day23 {
    type T1 = String;
    type T2 = String;

    fn new(input: &str) -> Self {
        Day23 {
            numbers: input.bytes().map(|n| n - b'0').collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        play(&self.numbers, self.numbers.len(), 100)
            .map(|n| (b'0' + n as u8) as char)
            .collect::<String>()
    }

    fn part_2(&self) -> Self::T2 {
        play(&self.numbers, 1_000, 10_000);
        "".to_string()
    }
}
