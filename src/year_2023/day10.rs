use std::cell::Cell;

use num::Integer;

use ahash::AHashSet;
use Direction::*;
use Pipe::*;

use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Pipe {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
    None,
}

impl Pipe {
    fn directions(&self) -> Option<(Direction, Direction)> {
        match self {
            NE => Some((Up, Right)),
            NS => Some((Up, Down)),
            NW => Some((Up, Left)),
            ES => Some((Right, Down)),
            EW => Some((Right, Left)),
            SW => Some((Down, Left)),
            None => Option::None,
        }
    }

    fn connects_to(&self, direction: Direction) -> bool {
        self.directions()
            .filter(|&(a, b)| direction == a || direction == b)
            .is_some()
    }

    fn next(&self, coming_from: Direction) -> Direction {
        let (a, b) = self.directions().unwrap();
        if coming_from == a {
            b
        } else if coming_from == b {
            a
        } else {
            unreachable!()
        }
    }
}

pub struct Day10 {
    start: (usize, usize),
    grid: Grid<Pipe>,
    path: Cell<Vec<(usize, usize)>>,
}

fn handle_bend((crosses, previous_bend_direction): (&mut usize, &mut Option<Direction>), direction: Direction) {
    if let Some(d) = previous_bend_direction.take() {
        if d == direction.opposite() {
            *crosses += 1;
        }
    } else {
        *previous_bend_direction = Some(direction);
    }
}

impl Day<'_> for Day10 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut grid = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'L' => NE,
                        b'|' => NS,
                        b'J' => NW,
                        b'F' => ES,
                        b'-' => EW,
                        b'7' => SW,
                        b'.' => None,
                        b'S' => {
                            start = (x, y);
                            None
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (sx, sy) = start;
        grid[sy][sx] = match (
            grid[sy - 1][sx].connects_to(Down),
            grid[sy][sx + 1].connects_to(Left),
            grid[sy + 1][sx].connects_to(Up),
        ) {
            (true, true, false) => NE,
            (true, false, true) => NS,
            (true, false, false) => NW,
            (false, true, true) => ES,
            (false, true, false) => EW,
            (false, false, true) => SW,
            _ => unreachable!(),
        };

        Self {
            start,
            grid: Grid::new(grid),
            path: Cell::new(Vec::new()),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (sx, sy) = self.start;
        let (mut direction, mut current) = [Up, Right, Down, Left]
            .into_iter()
            .map(|d| (d, d.apply((sx, sy))))
            .find(|&(d, i)| self.grid[i].connects_to(d.opposite()))
            .unwrap();
        let mut path = vec![current];

        while current != self.start {
            direction = self.grid[current].next(direction.opposite());
            current = direction.apply(current);
            path.push(current);
        }

        let len = path.len();
        self.path.set(path);

        len / 2
    }

    fn part_2(&self) -> Self::T2 {
        let path = self.path.take();
        let (min_x, max_x, min_y, max_y) = path.iter().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(min_x, max_x, min_y, max_y), &(x, y)| (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y)),
        );
        let path_set = AHashSet::from_iter(path.iter().copied());

        (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .scan((0, Option::None), |(crosses, previous_bend_direction), x| {
                        Some(if path_set.contains(&(x, y)) {
                            match self.grid[(x, y)] {
                                NS => *crosses += 1,
                                NE | NW => handle_bend((crosses, previous_bend_direction), Up),
                                ES | SW => handle_bend((crosses, previous_bend_direction), Down),
                                _ => {}
                            };
                            0
                        } else if crosses.is_odd() {
                            1
                        } else {
                            0
                        })
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
