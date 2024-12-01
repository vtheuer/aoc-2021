use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day01 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Day<'_> for Day01 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let pairs: Vec<(&str, &str)> = input.lines().filter_map(|l| l.split_once("   ")).collect();
        Self {
            left: pairs
                .iter()
                .map(|&(l, _)| l.parse().unwrap())
                .sorted_unstable_by_key(|&n| n)
                .collect(),
            right: pairs
                .iter()
                .map(|&(_, r)| r.parse().unwrap())
                .sorted_unstable_by_key(|&n| n)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.left
            .iter()
            .map(|&l| {
                if let Ok(mut i) = self.right.binary_search(&l) {
                    while l == self.right[i - 1] {
                        i -= 1;
                    }
                    l * self.right[i..].iter().take_while(|&&r| l == r).count()
                } else {
                    0
                }
            })
            .sum()
    }
}
