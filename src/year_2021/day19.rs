use crate::day::Day;

pub struct Day19 {
    beacons: Vec<Vec<(isize, isize, isize)>>,
}

impl Day<'_> for Day19 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            beacons: input
                .split("\n\n")
                .map(|s| {
                    s.lines()
                        .skip(1)
                        .map(|l| {
                            let mut coordinates = l.split(',').map(str::parse).map(Result::unwrap);
                            Some((coordinates.next()?, coordinates.next()?, coordinates.next()?))
                        })
                        .map(Option::unwrap)
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        0
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
