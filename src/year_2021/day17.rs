use std::cell::Cell;
use std::cmp::{max, min};

use crate::day::Day;

pub struct Day17 {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

fn max_distance(initial_velocity: isize) -> isize {
    initial_velocity * (initial_velocity + 1) / 2
}

impl Day17 {
    fn hits_y(&self, initial_y_velocity: isize) -> Option<usize> {
        let mut vy = initial_y_velocity;
        let mut y = 0;
        let mut steps = 0;

        while y > self.max_y {
            y += vy;
            vy -= 1;
            steps += 1;
        }

        if y >= self.min_y {
            Some(steps)
        } else {
            None
        }
    }

    fn hits(&self, initial_x_velocity: isize, initial_y_velocity: isize, steps: usize) -> bool {
        if steps as isize >= initial_x_velocity {
            (self.min_x..=self.max_x).contains(&max_distance(initial_x_velocity))
        } else {
            let mut x = 0;
            let mut y = 0;
            let mut vx = initial_x_velocity;
            let mut vy = initial_y_velocity;
            let mut hits_x = false;
            let mut hits_y = false;
            while !(hits_x && hits_y) && x < self.max_x && y >= self.min_y {
                x += vx;
                y += vy;
                vx = max(0, vx - 1);
                vy -= 1;
                hits_x = (self.min_x..=self.max_x).contains(&x);
                hits_y = (self.min_y..=self.max_y).contains(&y);
            }

            hits_x && hits_y
        }
    }
}

impl Day<'_> for Day17 {
    type T1 = isize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (_, target) = input.lines().next().unwrap().split_once(": ").unwrap();
        let t = target
            .split(", ")
            .flat_map(|p| p[2..].split(".."))
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        Self {
            min_x: min(t[0], t[1]),
            max_x: max(t[0], t[1]),
            min_y: min(t[2], t[3]),
            max_y: max(t[2], t[3]),
        }
    }

    fn part_1(&self) -> Self::T1 {
        (0..=-self.min_y)
            .filter(|&vy| self.hits_y(vy).is_some())
            .map(max_distance)
            .max()
            .unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        (self.min_y..=-self.min_y)
            .filter_map(|vy| self.hits_y(vy).map(|steps| (vy, steps)))
            .map(|(vy, steps)| (0..=self.max_x).filter(|&vx| self.hits(vx, vy, steps)).count())
            .sum()
    }
}
