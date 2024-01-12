use std::collections::VecDeque;

use fnv::FnvHashSet;

use Direction::*;

use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;
use crate::util::Joinable;

pub struct Day18<'a> {
    instructions: Vec<(Direction, u8, &'a [u8])>,
}

fn make_grid(instructions: Vec<(Direction, usize)>) -> Grid<bool> {
    let mut dug = FnvHashSet::from_iter([(0, 0)]);
    let mut p = (0, 0);
    for (direction, distance) in instructions {
        for _ in 0..distance {
            p = direction.apply(p);
            dug.insert(p);
        }
    }

    let (min_x, max_x, min_y, max_y) = dug.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y)),
    );

    let mut grid = Grid::init((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize, false);
    for (x, y) in dug {
        grid[((x - min_x) as usize, (y - min_y) as usize)] = true;
    }

    grid
}

fn print(grid: &[Vec<bool>]) {
    println!(
        "{}",
        grid.iter()
            .map(|row| row.iter().map(|&b| if b { '#' } else { '.' }).collect::<String>())
            .join("\n")
    );
}

fn find_start(grid: &Grid<bool>) -> (usize, usize) {
    let mut x = grid
        .row(0)
        .iter()
        .enumerate()
        .find(|&(_, &b)| b)
        .map(|(x, _)| x + 1)
        .unwrap();
    while grid[(x, 1)] {
        x += 1;
    }
    (x, 1)
}

fn fill(grid: &mut Grid<bool>) {
    let mut queue = VecDeque::from_iter([find_start(grid)]);

    while let Some((x, y)) = queue.pop_front() {
        grid[(x, y)] = true;
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .into_iter()
            .filter(|&n| !grid[n])
            .filter(|n| !queue.contains(n))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }
}

impl<'a> Day<'a> for Day18<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            instructions: input
                .lines()
                .map_while(|l| {
                    let mut parts = l.split(' ');
                    Some((
                        match parts.next()? {
                            "U" => Up,
                            "R" => Right,
                            "D" => Down,
                            "L" => Left,
                            _ => unreachable!(),
                        },
                        parts.next()?.parse().ok()?,
                        parts.next()?.trim_matches(|c| matches!(c, '#' | '(' | ')')).as_bytes(),
                    ))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut grid = make_grid(
            self.instructions
                .iter()
                .map(|&(direction, distance, _)| (direction, distance as usize))
                .collect::<Vec<_>>(),
        );

        fill(&mut grid);

        grid.values().filter(|&&b| b).count()
    }

    fn part_2(&self) -> Self::T2 {
        let mut grid = make_grid(
            self.instructions
                .iter()
                .map(|&(_, _, hex)| {
                    (
                        [Right, Down, Left, Right][(hex[5] - b'0') as usize],
                        usize::from_str_radix(std::str::from_utf8(&hex[..5]).unwrap(), 16).unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        );

        fill(&mut grid);

        grid.values().filter(|&&b| b).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let day18 = Day18::new(input);
        dbg!(day18.part_2());
    }
}
