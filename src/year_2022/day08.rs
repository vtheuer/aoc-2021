use std::convert::identity;
use std::io::BufRead;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day08<'a> {
    trees: Vec<&'a [u8]>,
}

impl<'a> Day<'a> for Day08<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            trees: input.lines().map(|l| l.as_bytes()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let width = self.trees[0].len();
        let height = self.trees.len();
        let mut visibles = vec![vec![false; width]; height];

        for x in 0..width {
            let mut highest = self.trees[0][x];
            visibles[0][x] = true;
            for y in 0..height {
                let h = self.trees[y][x];
                if h > highest {
                    visibles[y][x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
            highest = self.trees[height - 1][x];
            visibles[height - 1][x] = true;
            for y in (0..height).rev() {
                let h = self.trees[y][x];
                if h > highest {
                    visibles[y][x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
        }

        for y in 0..height {
            let mut highest = self.trees[y][0];
            visibles[y][0] = true;
            for x in 0..width {
                let h = self.trees[y][x];
                if h > highest {
                    visibles[y][x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
            highest = self.trees[y][width - 1];
            visibles[y][width - 1] = true;
            for x in (0..width).rev() {
                let h = self.trees[y][x];
                if h > highest {
                    visibles[y][x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
        }

        visibles
            .into_iter()
            .map(|row| row.into_iter().filter(|&visible| visible).count())
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        let width = self.trees[0].len();
        let height = self.trees.len();
        let left = (0..height)
            .map(|y| {
                (0..width)
                    .scan(vec![None; 10], |indices, x| {
                        let h = (self.trees[y][x] - b'0') as usize;
                        let i = indices[h..].iter().filter_map(|&index| index).max().unwrap_or(0);
                        indices[h] = Some(x);
                        Some(x - i)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let right = (0..height)
            .map(|y| {
                let mut r = (0..width)
                    .rev()
                    .scan(vec![None; 10], |indices, x| {
                        let h = (self.trees[y][x] - b'0') as usize;
                        let i = indices[h..]
                            .iter()
                            .filter_map(|&index| index)
                            .min()
                            .unwrap_or(width - 1);
                        indices[h] = Some(x);
                        Some(i - x)
                    })
                    .collect::<Vec<_>>();
                r.reverse();
                r
            })
            .collect::<Vec<_>>();
        0
    }
}
