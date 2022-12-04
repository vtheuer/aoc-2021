use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day04 {
    pairs: Vec<Vec<u8>>,
}

impl Day<'_> for Day04 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            pairs: input
                .lines()
                .map(|l| {
                    let p = l
                        .split(|c| c == ',' || c == '-')
                        .map(|n| n.parse::<u8>().unwrap())
                        .collect::<Vec<_>>();
                    if p[0] < p[2] {
                        p
                    } else {
                        vec![p[2], p[3], p[0], p[1]]
                    }
                })
                .filter(|p| if p[0] < p[2] { p[2] <= p[1] } else { p[0] <= p[3] })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.pairs
            .iter()
            .filter(|p| p[0] <= p[2] && p[1] >= p[3] || p[2] <= p[0] && p[3] >= p[1])
            .count()
    }

    fn part_2(&self) -> Self::T2 {
        self.pairs.len()
    }
}
