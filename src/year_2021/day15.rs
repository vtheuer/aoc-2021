use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::day::Day;

pub struct Day15 {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day15 {
    fn get(&self, x: usize, y: usize) -> usize {
        if x < self.width && y < self.height {
            self.grid[y][x] as usize
        } else {
            let v = self.grid[y % self.height][x % self.width] as usize + x / self.width + y / self.height;
            if v > 9 {
                (v % 10) + 1
            } else {
                v
            }
        }
    }

    fn neighbors(&self, x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
        let ix = x as isize;
        let iy = y as isize;
        let iw = w as isize;
        let ih = h as isize;
        vec![(ix, iy - 1), (ix - 1, iy), (ix + 1, iy), (ix, iy + 1)]
            .into_iter()
            .filter(move |&(x, y)| x >= 0 && y >= 0 && x < iw && y < ih)
            .map(move |(x, y)| (x as usize, y as usize))
    }

    fn find_distance(&self, w: usize, h: usize) -> usize {
        let mut heap = BinaryHeap::from([State { cost: 0, x: 0, y: 0 }]);
        let mut distances = vec![vec![usize::MAX; w as usize]; h as usize];
        distances[0][0] = 0;

        while let Some(State { cost, x, y }) = heap.pop() {
            if (x, y) == (w - 1, h - 1) {
                return cost;
            } else if cost <= distances[y][x] {
                for (nx, ny) in self.neighbors(x, y, w, h) {
                    let d = cost + self.get(nx, ny);
                    if d < distances[ny][nx] {
                        distances[ny][nx] = d;
                        heap.push(State { cost: d, x: nx, y: ny });
                    }
                }
            }
        }

        distances[h - 1][w - 1]
    }
}

impl Day<'_> for Day15 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input.lines().map(|l| l.bytes().map(|c| c - b'0').collect()).collect();
        Self {
            height: (&grid).len(),
            width: (&grid[0]).len(),
            grid,
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.find_distance(self.width, self.height)
    }

    fn part_2(&self) -> Self::T2 {
        self.find_distance(self.width * 5, self.height * 5)
    }
}
