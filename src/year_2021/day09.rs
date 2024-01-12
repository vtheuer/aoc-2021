use std::cell::RefCell;
use std::collections::VecDeque;

use ahash::AHashSet;

use crate::day::Day;

pub struct Day09 {
    heights: Vec<Vec<u8>>,
    low_points: RefCell<Vec<(usize, usize)>>,
}

impl Day09 {
    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let ix = x as isize;
        let iy = y as isize;
        let w = self.heights[0].len() as isize;
        let h = self.heights.len() as isize;
        vec![(ix, iy - 1), (ix - 1, iy), (ix + 1, iy), (ix, iy + 1)]
            .into_iter()
            .filter(move |&(x, y)| x >= 0 && y >= 0 && x < w && y < h)
            .map(move |(x, y)| (x as usize, y as usize))
    }

    fn basin_size(&self, origin: (usize, usize)) -> usize {
        let mut basin: AHashSet<(usize, usize)> = AHashSet::from_iter([origin].into_iter());
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([origin]);

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            for n in self.neighbors(x, y) {
                if self.heights[n.1][n.0] < 9 && !basin.contains(&n) {
                    basin.insert(n);
                    queue.push_back(n);
                }
            }
        }

        basin.len()
    }
}

impl Day<'_> for Day09 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            heights: input.lines().map(|l| l.bytes().map(|c| c - b'0').collect()).collect(),
            low_points: RefCell::default(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let low_points: Vec<(usize, usize)> = self
            .heights
            .iter()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |&(x, &h)| {
                        h < 9 && self.neighbors(x, y).map(|(x, y)| self.heights[y][x]).all(|n| n > h)
                    })
                    .map(move |(x, _)| (x, y))
            })
            .collect();
        let sum = low_points
            .iter()
            .map(|&(x, y)| self.heights[y][x] as usize + 1)
            .sum::<usize>();
        self.low_points.replace(low_points);
        sum
    }

    fn part_2(&self) -> Self::T2 {
        self.low_points
            .borrow()
            .iter()
            .map(|&low_point| self.basin_size(low_point))
            .fold([0; 3], |[a, b, c], n| {
                if n <= c {
                    [a, b, c]
                } else if n <= b {
                    [a, b, n]
                } else if n <= a {
                    [a, n, b]
                } else {
                    [n, a, b]
                }
            })
            .into_iter()
            .product()
    }
}
