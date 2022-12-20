use std::cell::Cell;
use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::thread::sleep;
use std::time::Duration;

use fnv::{FnvBuildHasher, FnvHashSet};

use crate::day::Day;

pub struct Day18 {
    cubes: Vec<(usize, usize, usize)>,
    space: Vec<Vec<Vec<bool>>>,
}

fn check_coord(v: usize, d: isize, max: usize) -> bool {
    match d {
        0 => true,
        -1 => v > 0,
        1 => v < max,
        _ => unreachable!(),
    }
}

impl Day<'_> for Day18 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let cubes = input
            .lines()
            .map_while(|l| {
                let mut coordinates = l.split(',').map(|n| n.parse().unwrap());
                Some((coordinates.next()?, coordinates.next()?, coordinates.next()?))
            })
            .collect::<Vec<(usize, usize, usize)>>();
        let max_x = cubes.iter().map(|&(x, _, _)| x).max().unwrap() + 1;
        let max_y = cubes.iter().map(|&(_, y, _)| y).max().unwrap() + 1;
        let max_z = cubes.iter().map(|&(_, _, z)| z).max().unwrap() + 1;

        let mut space = vec![vec![vec![false; max_x]; max_y]; max_z];
        for &(x, y, z) in &cubes {
            space[z][y][x] = true;
        }
        Self { cubes, space }
    }

    fn part_1(&self) -> Self::T1 {
        self.cubes.iter().map(|&c| 6 - self.count_neighbours(c)).sum()
    }

    fn part_2(&self) -> Self::T2 {
        let max_x = self.space[0][0].len() - 1;
        let max_y = self.space[0].len() - 1;
        let max_z = self.space.len() - 1;
        let mut visited = vec![vec![vec![false; max_x + 1]; max_y + 1]; max_z + 1];
        visited[0][0][0] = true;
        let mut queue = VecDeque::from([(0, 0, 0)]);
        let mut sides = 0;

        while let Some((x, y, z)) = queue.pop_front() {
            for (nx, ny, nz) in [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]
                .into_iter()
                .filter(|&(dx, dy, dz)| {
                    check_coord(x, dx, max_x) && check_coord(y, dy, max_y) && check_coord(z, dz, max_z)
                })
                .map(|(dx, dy, dz)| {
                    (
                        (x as isize + dx) as usize,
                        (y as isize + dy) as usize,
                        (z as isize + dz) as usize,
                    )
                })
            {
                if self.space[nz][ny][nx] {
                    sides += 1;
                } else if !visited[nz][ny][nx] {
                    visited[nz][ny][nx] = true;
                    queue.push_back((nx, ny, nz));
                }
            }
        }

        sides
            + self
                .cubes
                .iter()
                .filter(|&&(x, y, z)| x == 0 || x == max_x || y == 0 || y == max_y || z == 0 || z == max_z)
                .count()
    }
}

impl Day18 {
    fn count_neighbours(&self, (x, y, z): (usize, usize, usize)) -> usize {
        let max_x = self.space[0][0].len() - 1;
        let max_y = self.space[0].len() - 1;
        let max_z = self.space.len() - 1;

        [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]
            .into_iter()
            .filter(|&(dx, dy, dz)| {
                (dx >= 0 || x > 0)
                    && (dx <= 0 || x < max_x)
                    && (dy >= 0 || y > 0)
                    && (dy <= 0 || y < max_y)
                    && (dz >= 0 || z > 0)
                    && (dz <= 0 || z < max_z)
            })
            .map(|(dx, dy, dz)| {
                (
                    (x as isize + dx) as usize,
                    (y as isize + dy) as usize,
                    (z as isize + dz) as usize,
                )
            })
            .filter(|&(x, y, z)| self.space[z][y][x])
            .count()
    }
}
