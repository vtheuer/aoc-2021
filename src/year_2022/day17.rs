use num::Num;
use std::fmt::Display;
use std::io::Read;

use crate::day::Day;
use crate::util::Joinable;

const WIDTH: u8 = 7;

pub struct Day17 {
    pushes: Vec<isize>,
}

fn shapes() -> Vec<(u8, Vec<u8>)> {
    vec![
        (4, vec![0b1111]),
        (3, vec![0b010, 0b111, 0b010]),
        (3, vec![0b111, 0b001, 0b001]),
        (1, vec![0b1; 4]),
        (2, vec![0b11; 2]),
    ]
}

fn at(row: u8, width: u8, x: isize) -> u8 {
    row << (WIDTH - width - x as u8)
}

fn collides(grid: &[u8], (width, shape): &(u8, Vec<u8>), x: isize, y: usize) -> bool {
    shape
        .iter()
        .enumerate()
        .any(|(sy, &row)| at(row, *width, x) & grid[y + sy] > 0)
}

fn can_shift(grid: &[u8], shape: &(u8, Vec<u8>), x: isize, y: usize, dx: isize) -> bool {
    (if dx == -1 { x > 0 } else { (x as u8) < WIDTH - shape.0 }) && !collides(grid, shape, x + dx, y)
}

fn can_fall(grid: &[u8], shape: &(u8, Vec<u8>), x: isize, y: usize) -> bool {
    y > 0 && !collides(grid, shape, x, y - 1)
}

fn drop_shape<I>(grid: &mut Vec<u8>, pushes: &mut I, shape: &(u8, Vec<u8>)) -> usize
where
    I: Iterator<Item = isize>,
{
    let highest_rock = get_highest_block(grid);
    let height = shape.1.len();
    for _ in grid.len()..highest_rock + height + 3 {
        grid.push(0);
    }
    let mut x = 2;
    let mut y = highest_rock + 3;

    loop {
        let dx = pushes.next().unwrap();
        if can_shift(grid, shape, x, y, dx) {
            x += dx;
        }

        if can_fall(grid, shape, x, y) {
            y -= 1;
        } else {
            break;
        }
    }

    for (sy, &row) in shape.1.iter().enumerate() {
        grid[y + sy] |= at(row, shape.0, x);
    }

    highest_rock - y
}

fn get_highest_block(grid: &[u8]) -> usize {
    grid.iter()
        .enumerate()
        .rev()
        .find(|&(_, &row)| row > 0)
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

fn find_period(array: &[impl Num], min_len: usize) -> Option<(usize, usize)> {
    if array.len() < min_len {
        return None;
    }

    for start in (0..array.len() - 2 * min_len).rev() {
        let slice = &array[start..];
        for len in (min_len..(slice.len() / 2)).rev() {
            let next_slice = &array[start + len..];
            let mut i = 0;
            while i < len && slice[i] == next_slice[i] {
                i += 1;
            }
            if i == len {
                return Some((start, len));
            }
        }
    }

    None
}

impl Day<'_> for Day17 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            pushes: input
                .lines()
                .next()
                .unwrap()
                .bytes()
                .map(|c| if c == b'<' { -1 } else { 1 })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let shapes_vec = shapes();
        let mut shapes = shapes_vec.iter().cycle();
        let mut pushes = self.pushes.iter().copied().cycle();
        let mut grid: Vec<u8> = vec![];

        for _ in 0..2022 {
            drop_shape(&mut grid, &mut pushes, shapes.next().unwrap());
        }

        grid.len() - grid.iter().rev().filter(|&&row| row == 0).count()
    }

    fn part_2(&self) -> Self::T2 {
        let shapes_vec = shapes();
        let mut shapes = shapes_vec.iter().cycle();
        let mut pushes = self.pushes.iter().copied().cycle();
        let mut grid: Vec<u8> = vec![];
        let min_period_len = self.pushes.len() / shapes_vec.len();
        let mut height_diff = vec![];
        let mut previous_height = 0;

        for rocks_dropped in 1.. {
            drop_shape(&mut grid, &mut pushes, shapes.next().unwrap());

            let height = get_highest_block(&grid);
            height_diff.push(height.saturating_sub(previous_height));
            previous_height = height;

            if rocks_dropped > self.pushes.len() {
                if let Some((start, len)) = find_period(&height_diff, min_period_len) {
                    let period_height = height_diff[start..start + len].iter().copied().sum::<usize>();
                    let remaining_to_next_period = len - (rocks_dropped - start) % len;
                    let remaining_rocks = 1000000000000 - (rocks_dropped + remaining_to_next_period);

                    for _ in 0..remaining_to_next_period + (remaining_rocks % len) {
                        drop_shape(&mut grid, &mut pushes, shapes.next().unwrap());
                    }

                    return get_highest_block(&grid) + (remaining_rocks / len) * period_height;
                }
            }
        }

        unreachable!()
    }
}
