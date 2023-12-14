use crate::day::Day;

pub struct Day14 {
    grid: Vec<Vec<Option<bool>>>,
}

fn tilt_north(grid: &[Vec<Option<bool>>]) -> Vec<Vec<Option<bool>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![None; width]; height];

    for x in 0..width {
        let mut previous_rock = 0;

        for y in 0..height {
            match grid[y][x] {
                Some(true) => {
                    new_grid[previous_rock][x] = Some(true);
                    previous_rock += 1;
                }
                Some(false) => {
                    new_grid[y][x] = Some(false);
                    previous_rock = y + 1;
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_south(grid: &[Vec<Option<bool>>]) -> Vec<Vec<Option<bool>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![None; width]; height];

    for x in 0..width {
        let mut previous_rock = height - 1;

        for y in (0..height).rev() {
            match grid[y][x] {
                Some(true) => {
                    new_grid[previous_rock][x] = Some(true);
                    previous_rock = previous_rock.saturating_sub(1);
                }
                Some(false) => {
                    new_grid[y][x] = Some(false);
                    previous_rock = y.saturating_sub(1);
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_west(grid: &[Vec<Option<bool>>]) -> Vec<Vec<Option<bool>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![None; width]; height];

    for y in 0..height {
        let mut previous_rock = 0;

        for x in 0..width {
            match grid[y][x] {
                Some(true) => {
                    new_grid[y][previous_rock] = Some(true);
                    previous_rock += 1;
                }
                Some(false) => {
                    new_grid[y][x] = Some(false);
                    previous_rock = x + 1;
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_east(grid: &[Vec<Option<bool>>]) -> Vec<Vec<Option<bool>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![None; width]; height];

    for y in 0..height {
        let mut previous_rock = width - 1;

        for x in (0..width).rev() {
            match grid[y][x] {
                Some(true) => {
                    new_grid[y][previous_rock] = Some(true);
                    previous_rock = previous_rock.saturating_sub(1);
                }
                Some(false) => {
                    new_grid[y][x] = Some(false);
                    previous_rock = x.saturating_sub(1);
                }
                None => {}
            }
        }
    }

    new_grid
}

fn get_load(grid: &[Vec<Option<bool>>]) -> usize {
    let height = grid.len();
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .filter_map(|r| r.filter(|&rolls| rolls).and(Some(height - y)))
                .sum::<usize>()
        })
        .sum()
}

fn find_period(array: &[usize], min_len: usize) -> Option<(usize, usize)> {
    if array.len() < 2 * min_len {
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

impl Day<'_> for Day14 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|l| {
                    l.bytes()
                        .map(|b| match b {
                            b'.' => None,
                            _ => Some(b == b'O'),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        get_load(&tilt_north(&self.grid))
    }

    fn part_2(&self) -> Self::T2 {
        let mut load_per_cycle = vec![];
        let mut grid = self.grid.clone();
        let mut period = None;

        while period.is_none() {
            grid = tilt_north(&grid);
            grid = tilt_west(&grid);
            grid = tilt_south(&grid);
            grid = tilt_east(&grid);
            load_per_cycle.push(get_load(&grid));

            period = find_period(&load_per_cycle, 3);
        }

        let (start, len) = period.unwrap();
        load_per_cycle[start + ((1000000000 - start) % len) - 1]
    }
}
