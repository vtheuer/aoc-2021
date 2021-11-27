use crate::day::Day;

pub struct Day06 {
    groups: Vec<Vec<u32>>,
}

impl Day06 {
    fn sum_groups_with<F: Fn(u32, &u32) -> u32>(&self, neutral: u32, folder: F) -> u32 {
        self.groups
            .iter()
            .map(|group| group.iter().fold(neutral, &folder).count_ones())
            .sum::<u32>()
    }
}

impl Day<'_> for Day06 {
    type T1 = u32;
    type T2 = u32;

    fn new(input: &str) -> Self {
        Day06 {
            groups: input
                .split("\n\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|l| l.bytes().fold(0, |a, c| a | (1 << (c - b'a'))))
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.sum_groups_with(0, |a, b| a | b)
    }

    fn part_2(&self) -> Self::T2 {
        self.sum_groups_with(2u32.pow(27) - 1, |a, b| a & b)
    }
}
