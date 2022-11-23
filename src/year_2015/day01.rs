use crate::day::Day;

pub struct Day01 {
    parentheses: Vec<isize>,
}

impl Day<'_> for Day01 {
    type T1 = isize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            parentheses: input.chars().map(|c| if c == '(' { 1 } else { -1 }).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.parentheses.iter().sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.parentheses
            .iter()
            .enumerate()
            .scan(0, |floor, (i, &p)| {
                *floor += p;
                Some((i, *floor))
            })
            .find_map(|(i, floor)| if floor < 0 { Some(i + 1) } else { None })
            .unwrap()
    }
}
