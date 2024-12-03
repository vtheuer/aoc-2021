use crate::day::Day;
use regex::{Match, Regex};

pub struct Day03<'a> {
    input: &'a str,
}

fn parse_match(m: Match) -> usize {
    m.as_str().parse().unwrap()
}

impl<'a> Day<'a> for Day03<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn part_1(&self) -> Self::T1 {
        Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)")
            .unwrap()
            .captures_iter(self.input)
            .filter_map(|c| Some(parse_match(c.get(1)?) * parse_match(c.get(2)?)))
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)|(do\\(\\))|don't\\(\\)")
            .unwrap()
            .captures_iter(self.input)
            .fold((true, 0), |(enabled, sum), c| {
                if let Some(n) = c.get(1).map(parse_match) {
                    (
                        enabled,
                        sum + if enabled { n * parse_match(c.get(2).unwrap()) } else { 0 },
                    )
                } else {
                    (c.get(3).is_some(), sum)
                }
            })
            .1
    }
}
