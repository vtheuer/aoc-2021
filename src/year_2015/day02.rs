use crate::day::Day;
use std::cmp::max;

pub struct Day02 {
    presents: Vec<(usize, usize, usize)>,
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            presents: input
                .lines()
                .map(
                    |l| match l.split('x').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] {
                        [l, w, h] => (l, w, h),
                        _ => unreachable!(),
                    },
                )
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.presents
            .iter()
            .map(|(l, w, h)| {
                let top = l * w;
                let front = w * h;
                let side = l * h;
                2 * top + 2 * front + 2 * side + [top, front, side].iter().min().unwrap()
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.presents
            .iter()
            .map(|(l, w, h)| 3 * l + 3 * w + 3 * h - 2 * [l, w, h].into_iter().max().unwrap() + l * w * h)
            .sum()
    }
}
