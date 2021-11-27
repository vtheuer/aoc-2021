use crate::day::Day;

pub struct Day01 {
    numbers: Vec<u32>,
}

impl Day<'_> for Day01 {
    type T1 = u32;
    type T2 = u32;

    fn new(input: &str) -> Self {
        Day01 {
            numbers: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.numbers
            .iter()
            .enumerate()
            .flat_map(|(i, &a)| self.numbers.iter().skip(i + 1).map(move |&b| (a, b)))
            .into_iter()
            .find(|(a, b)| a + b == 2020)
            .map(|(a, b)| a * b)
            .unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        self.numbers
            .iter()
            .enumerate()
            .flat_map(|(i, &a)| {
                self.numbers
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .filter(move |(_, &b)| a + b < 2020)
                    .flat_map(|(j, &b)| self.numbers.iter().skip(j + 1).map(move |&c| (b, c)))
                    .map(move |(b, c)| (a, b, c))
            })
            .into_iter()
            .find(|&(a, b, c)| a + b + c == 2020)
            .map(|(a, b, c)| a * b * c)
            .unwrap()
    }
}
