use crate::day::Day;
use crate::util::{FindIndex, SortableByKey};

pub struct Day01 {
    elves: Vec<usize>,
}

impl Day<'_> for Day01 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            elves: input
                .split("\n\n")
                .map(|elf| elf.lines().map(|l| l.parse::<usize>().unwrap()).sum())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        *self.elves.iter().max().unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        self.elves
            .iter()
            .fold(&mut vec![0, 0, 0], |top3, &c| {
                if let Some((i, _)) = top3.iter().find_index_by(|&&top| c > top) {
                    top3[i..].rotate_right(1);
                    top3[i] = c;
                }

                top3
            })
            .iter()
            .sum()
    }
}
