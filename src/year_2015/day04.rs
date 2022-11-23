use crate::day::Day;

pub struct Day04<'a> {
    key: &'a str,
}

impl Day04<'_> {
    fn find(&self, needle: &str) -> usize {
        (0..)
            .find(|&n| format!("{:x}", md5::compute(format!("{}{}", self.key, n))).starts_with(needle))
            .unwrap()
    }
}

impl<'a> Day<'a> for Day04<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self { key: input.trim() }
    }

    fn part_1(&self) -> Self::T1 {
        self.find("00000")
    }

    fn part_2(&self) -> Self::T2 {
        self.find("000000")
    }
}
