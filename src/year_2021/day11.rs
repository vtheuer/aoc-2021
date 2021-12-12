use crate::day::Day;
use crate::util::Joinable;
use std::convert::identity;

pub struct Day11 {
    jellyfishes: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Day11 {
    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let ix = x as isize;
        let iy = y as isize;
        let w = self.width as isize;
        let h = self.height as isize;
        vec![
            (ix - 1, iy - 1),
            (ix, iy - 1),
            (ix + 1, iy - 1),
            (ix - 1, iy),
            (ix + 1, iy),
            (ix - 1, iy + 1),
            (ix, iy + 1),
            (ix + 1, iy + 1),
        ]
        .into_iter()
        .filter(move |&(x, y)| x >= 0 && y >= 0 && x < w && y < h)
        .map(move |(x, y)| (x as usize, y as usize))
    }

    fn update(&self, grid: &mut Vec<Vec<u8>>, flashed: &mut Vec<Vec<bool>>, x: usize, y: usize) {
        if !flashed[y][x] {
            grid[y][x] = if grid[y][x] == 9 {
                flashed[y][x] = true;
                for (nx, ny) in self.neighbors(x, y) {
                    self.update(grid, flashed, nx, ny);
                }
                0
            } else {
                grid[y][x] + 1
            }
        }
    }

    fn step(&self, prev: &[Vec<u8>]) -> (Vec<Vec<u8>>, usize) {
        let mut next: Vec<Vec<u8>> = prev.to_owned();
        let mut flashed = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                self.update(&mut next, &mut flashed, x, y);
            }
        }

        (
            next,
            flashed
                .into_iter()
                .map(|row| row.into_iter().filter(|&flashed| flashed).count())
                .sum(),
        )
    }
}

impl Day<'_> for Day11 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let jellyfishes: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().map(|j| j - b'0').collect()).collect();
        Self {
            height: jellyfishes.len(),
            width: input.lines().next().unwrap().len(),
            jellyfishes,
        }
    }

    fn part_1(&self) -> Self::T1 {
        (0..100)
            .fold((self.jellyfishes.clone(), 0), |(prev, flashes), _| {
                let (next, new_flashes) = self.step(&prev);
                (next, flashes + new_flashes)
            })
            .1
    }

    fn part_2(&self) -> Self::T2 {
        (1..)
            .scan(self.jellyfishes.clone(), |grid, i| {
                let (next, flashes) = self.step(grid);
                *grid = next;
                Some((i, flashes))
            })
            .find_map(|(i, flashes)| {
                if flashes == self.width * self.height {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap()
    }
}
