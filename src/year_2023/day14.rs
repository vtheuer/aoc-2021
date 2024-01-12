use crate::day::Day;
use crate::util::grid::Grid;

pub struct Day14 {
    grid: Grid<Option<bool>>,
}

fn tilt_north(grid: &Grid<Option<bool>>) -> Grid<Option<bool>> {
    let height = grid.height;
    let width = grid.width;
    let mut new_grid = Grid::init(width, height, None);

    for x in 0..width {
        let mut previous_rock = 0;

        for y in 0..height {
            match grid[(x, y)] {
                Some(true) => {
                    new_grid[(x, previous_rock)] = Some(true);
                    previous_rock += 1;
                }
                Some(false) => {
                    new_grid[(x, y)] = Some(false);
                    previous_rock = y + 1;
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_south(grid: &Grid<Option<bool>>) -> Grid<Option<bool>> {
    let height = grid.height;
    let width = grid.width;
    let mut new_grid = Grid::init(width, height, None);

    for x in 0..width {
        let mut previous_rock = height - 1;

        for y in (0..height).rev() {
            match grid[(x, y)] {
                Some(true) => {
                    new_grid[(x, previous_rock)] = Some(true);
                    previous_rock = previous_rock.saturating_sub(1);
                }
                Some(false) => {
                    new_grid[(x, y)] = Some(false);
                    previous_rock = y.saturating_sub(1);
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_west(grid: &Grid<Option<bool>>) -> Grid<Option<bool>> {
    let height = grid.height;
    let width = grid.width;
    let mut new_grid = Grid::init(width, height, None);

    for y in 0..height {
        let mut previous_rock = 0;

        for x in 0..width {
            match grid[(x, y)] {
                Some(true) => {
                    new_grid[(previous_rock, y)] = Some(true);
                    previous_rock += 1;
                }
                Some(false) => {
                    new_grid[(x, y)] = Some(false);
                    previous_rock = x + 1;
                }
                None => {}
            }
        }
    }

    new_grid
}

fn tilt_east(grid: &Grid<Option<bool>>) -> Grid<Option<bool>> {
    let height = grid.height;
    let width = grid.width;
    let mut new_grid = Grid::init(width, height, None);

    for y in 0..height {
        let mut previous_rock = width - 1;

        for x in (0..width).rev() {
            match grid[(x, y)] {
                Some(true) => {
                    new_grid[(previous_rock, y)] = Some(true);
                    previous_rock = previous_rock.saturating_sub(1);
                }
                Some(false) => {
                    new_grid[(x, y)] = Some(false);
                    previous_rock = x.saturating_sub(1);
                }
                None => {}
            }
        }
    }

    new_grid
}

fn get_load(grid: &Grid<Option<bool>>) -> usize {
    grid.rows()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .filter_map(|r| r.filter(|&rolls| rolls).and(Some(grid.height - y)))
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
            grid: Grid::parse(input, |b| match b {
                b'.' => None,
                _ => Some(b == b'O'),
            }),
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
