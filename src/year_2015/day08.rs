use crate::day::Day;

pub struct Day08<'a> {
    strings: Vec<&'a str>,
}

impl<'a> Day<'a> for Day08<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            strings: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.strings
            .iter()
            .map(|s| {
                s.len()
                    - s.bytes()
                        .skip(1)
                        .take(s.len() - 2)
                        .scan((false, 0), |(escaping, chars_to_escape), c| {
                            if *chars_to_escape > 0 {
                                *chars_to_escape -= 1;
                                Some(0)
                            } else if *escaping {
                                if c == b'x' {
                                    *chars_to_escape = 2
                                }
                                *escaping = false;
                                Some(0)
                            } else {
                                *escaping = c == b'\\';
                                Some(1)
                            }
                        })
                        .sum::<usize>()
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.strings
            .iter()
            .map(|s| {
                s.bytes()
                    .map(|c| match c {
                        b'"' | b'\\' => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
                    + 2
            })
            .sum()
    }
}
