use crate::day::Day;

pub struct Day02<'a> {
    passwords: Vec<(usize, usize, char, &'a str)>,
}

impl<'a> Day<'a> for Day02<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            passwords: input
                .lines()
                .map(|l| {
                    let (rule, password) = l.split_once(": ")?;
                    let (range, c) = rule.split_once(' ')?;
                    let (min, max) = range.split_once('-')?;

                    Some((min.parse().ok()?, max.parse().ok()?, c.chars().next()?, password))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.passwords
            .iter()
            .filter(|(min, max, required_char, password)| {
                let count = password.chars().filter(|&c| c == *required_char).count();
                count >= *min && count <= *max
            })
            .count()
    }

    fn part_2(&self) -> Self::T2 {
        self.passwords
            .iter()
            .filter(|(a, b, required_char, password)| {
                [*a, *b]
                    .iter()
                    .filter(|&c| password.as_bytes()[c - 1] == *required_char as u8)
                    .count()
                    == 1
            })
            .count()
    }
}
