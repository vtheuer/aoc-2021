use std::cmp::max;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day13 {
    grid: Vec<Vec<bool>>,
    folds: Vec<(bool, usize)>,
}

fn fold(grid: &[Vec<bool>], along_x: bool) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let (w, h) = if along_x {
        (grid[0].len() / 2, grid.len())
    } else {
        (grid[0].len(), grid.len() / 2)
    };
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    grid[y][x] || {
                        if along_x {
                            grid[y][width - 1 - x]
                        } else {
                            grid[height - 1 - y][x]
                        }
                    }
                })
                .collect()
        })
        .collect()
}

impl Day<'_> for Day13 {
    type T1 = usize;
    type T2 = String;

    fn new(input: &str) -> Self {
        let (dots, folds_str) = input.split_once("\n\n").unwrap();
        let folds = folds_str
            .lines()
            .map(|l| {
                let (axis, n) = &l["fold along ".len()..].split_once('=')?;
                Some((axis == "x", n.parse().ok()?))
            })
            .map(Option::unwrap)
            .collect::<Vec<(bool, usize)>>();
        let width = folds
            .iter()
            .find(|&&(along_x, _)| along_x)
            .map(|&(_, y)| y * 2 + 1)
            .unwrap();
        let height = folds
            .iter()
            .find(|&&(along_x, _)| !along_x)
            .map(|&(_, x)| x * 2 + 1)
            .unwrap();
        Self {
            grid: dots
                .lines()
                .map(|l| {
                    let (x, y) = l.split_once(',')?;
                    Some((x.parse().ok()?, y.parse().ok()?))
                })
                .map(Option::unwrap)
                .fold(vec![vec![false; width]; height], |mut grid, (x, y): (usize, usize)| {
                    grid[y][x] = true;
                    grid
                }),
            folds,
        }
    }

    fn part_1(&self) -> Self::T1 {
        fold(&self.grid, self.folds[0].0)
            .into_iter()
            .map(|row| row.into_iter().filter(|&dot| dot).count())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let grid = self
            .folds
            .iter()
            .fold(self.grid.clone(), |grid, &(along_x, _)| fold(&grid, along_x));

        println!(
            "{}\n",
            grid.into_iter()
                .map(|row| row
                    .into_iter()
                    .map(|dot| if dot { '#' } else { ' ' })
                    .collect::<String>())
                .join("\n")
        );

        String::from("see above")
    }
}
