use fnv::{FnvHashMap, FnvHashSet};
use std::cmp::min;

use crate::day::Day;
use crate::util::{split_pair, Joinable};

pub struct Day05 {
    lines: Vec<((isize, isize), (isize, isize))>,
}

impl Day05 {
    fn count_overlaps(&self, keep_diagonals: bool) -> usize {
        let filter_diagonals: fn(&&((_, _), (_, _))) -> bool = if keep_diagonals {
            |_| true
        } else {
            |&&((fx, fy), (tx, ty))| fx == tx || fy == ty
        };
        let (max_x, max_y) = self.lines.iter().fold((0, 0), |(max_x, max_y), &((fx, tx), (fy, ty))| {
            (
                [max_x, fx, tx].into_iter().max().unwrap(),
                [max_y, fy, ty].into_iter().max().unwrap(),
            )
        });
        self.lines
            .iter()
            .filter(filter_diagonals)
            .fold(
                vec![vec![0; max_x as usize + 1]; max_y as usize + 1],
                |mut vents, &((fx, fy), (tx, ty))| {
                    (if keep_diagonals && fx != tx && fy != ty {
                        let dx = (tx - fx).signum();
                        let dy = (ty - fy).signum();

                        (0..=(tx - fx).abs())
                            .map(|i| ((fx + i * dx), (fy + i * dy)))
                            .collect::<Vec<_>>()
                    } else {
                        (fx..=tx)
                            .flat_map(move |x| (fy..=ty).map(move |y| (x, y)))
                            .collect::<Vec<_>>()
                    })
                    .into_iter()
                    .for_each(|(x, y)| vents[y as usize][x as usize] += 1);
                    vents
                },
            )
            .into_iter()
            .map(|row| row.into_iter().filter(|&v| v > 0).count())
            .sum()
    }
}

impl Day<'_> for Day05 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            lines: input
                .lines()
                .map(|l| {
                    let mut it = l.split(" -> ").map(|part| {
                        let mut coordinates = part.split(',').map(str::parse).map(Result::unwrap);
                        (coordinates.next().unwrap(), coordinates.next().unwrap())
                    });
                    (it.next().unwrap(), it.next().unwrap())
                })
                .map(|(f, t)| if f.0 > t.0 || f.1 > t.1 { (t, f) } else { (f, t) })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_overlaps(false)
    }

    fn part_2(&self) -> Self::T2 {
        self.count_overlaps(true)
    }
}
