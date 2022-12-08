use std::cmp::max;
use std::convert::identity;
use std::io::BufRead;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day08<'a> {
    trees: Vec<&'a [u8]>,
}

impl Day08<'_> {
    fn horizontal_scores(&self, rev: bool) -> Vec<Vec<usize>> {
        let width = self.trees[0].len();
        let height = self.trees.len();
        (0..height)
            .map(|y| {
                let mut it: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(0..width);
                if rev {
                    it = Box::new(it.rev());
                }

                let mut scores = it
                    .scan(vec![None; 10], |indices, x| {
                        let h = (self.trees[y][x] - b'0') as usize;
                        let i = indices[h..].iter().filter_map(|&index| index).max().unwrap_or(if rev {
                            width - 1
                        } else {
                            0
                        });
                        indices[h] = Some(x);
                        Some(if rev { i - x } else { x - i })
                    })
                    .collect::<Vec<_>>();
                if rev {
                    scores.reverse();
                }
                scores
            })
            .collect::<Vec<_>>()
    }

    fn vertical_scores(&self, rev: bool) -> Vec<Vec<usize>> {
        let width = self.trees[0].len();
        let height = self.trees.len();
        (0..width)
            .map(|x| {
                let mut it: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(0..height);
                if rev {
                    it = Box::new(it.rev());
                }

                let mut scores = it
                    .scan(vec![None; 10], |indices, y| {
                        let h = (self.trees[y][x] - b'0') as usize;
                        let i = indices[h..].iter().filter_map(|&index| index).max().unwrap_or(if rev {
                            height - 1
                        } else {
                            0
                        });
                        indices[h] = Some(y);
                        Some(if rev { i - y } else { y - i })
                    })
                    .collect::<Vec<_>>();
                if rev {
                    scores.reverse();
                }
                scores
            })
            .collect::<Vec<_>>()
    }
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
            for (y, row) in visibles.iter_mut().enumerate() {
                let h = self.trees[y][x];
                if h > highest {
                    row[x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
            highest = self.trees[height - 1][x];
            visibles[height - 1][x] = true;
            for (y, row) in visibles.iter_mut().enumerate().rev() {
                let h = self.trees[y][x];
                if h > highest {
                    row[x] = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
        }

        for (y, row) in visibles.iter_mut().enumerate() {
            let mut highest = self.trees[y][0];
            row[0] = true;
            for (x, visible) in row.iter_mut().enumerate() {
                let h = self.trees[y][x];
                if h > highest {
                    *visible = true;
                    highest = h;
                    if h == b'9' {
                        break;
                    }
                }
            }
            highest = self.trees[y][width - 1];
            row[width - 1] = true;
            for (x, visible) in row.iter_mut().enumerate().rev() {
                let h = self.trees[y][x];
                if h > highest {
                    *visible = true;
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
        let left = self.horizontal_scores(false);
        let right = self.horizontal_scores(true);
        let up = self.vertical_scores(false);
        let down = self.vertical_scores(true);
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| left[y][x] * right[y][x] * up[x][y] * down[x][y])
            .max()
            .unwrap()
    }
}
