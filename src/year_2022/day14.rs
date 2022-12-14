use std::thread::sleep;
use std::time::Duration;

use Element::*;

use crate::day::Day;
use crate::util::Joinable;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Element {
    Space,
    Rock,
    Sand,
}

pub struct Day14 {
    max_y: usize,
    grid: Vec<Vec<Element>>,
}

impl Day<'_> for Day14 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let paths: Vec<Vec<(usize, usize)>> = input
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|pair| {
                        let (l, r) = pair.split_once(',').unwrap();
                        (l.parse().unwrap(), r.parse().unwrap())
                    })
                    .collect()
            })
            .collect();
        let max_y = paths
            .iter()
            .flat_map(|path| path.iter().map(|&(_, y)| y))
            .max()
            .unwrap()
            + 1;
        let mut grid = vec![vec![Space; 1000]; max_y + 1];
        for path in paths {
            let (mut x, mut y) = path[0];
            for &(nx, ny) in path.iter().skip(1) {
                let dx = (nx as isize - x as isize).signum();
                let dy = (ny as isize - y as isize).signum();
                while x != nx || y != ny {
                    grid[y][x] = Rock;
                    x = (x as isize + dx) as usize;
                    y = (y as isize + dy) as usize;
                }
                grid[y][x] = Rock;
            }
        }
        Self { max_y, grid }
    }

    fn part_1(&self) -> Self::T1 {
        self.fill(|_, y| y == self.max_y)
    }

    fn part_2(&self) -> Self::T2 {
        self.fill(|x, y| (x, y) == (500, 0)) + 1
    }
}

impl Day14 {
    fn fill<F>(&self, is_at_rest: F) -> usize
    where
        F: Fn(usize, usize) -> bool,
    {
        let mut grid = self.grid.clone();
        let mut n = 0;
        loop {
            let (x, y) = grid
                .iter()
                .enumerate()
                .scan(500, |x, (y, row)| {
                    if row[*x] == Space {
                        Some((*x, y))
                    } else if row[*x - 1] == Space {
                        *x -= 1;
                        Some((*x, y))
                    } else if row[*x + 1] == Space {
                        *x += 1;
                        Some((*x, y))
                    } else {
                        None
                    }
                })
                .last()
                .unwrap();
            if is_at_rest(x, y) {
                break;
            }
            grid[y][x] = Sand;
            n += 1;
        }
        n
    }
}
