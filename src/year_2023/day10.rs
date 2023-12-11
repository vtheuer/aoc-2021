use std::cell::Cell;

use fnv::FnvHashSet;
use num::Integer;

use Direction::*;
use Pipe::*;

use crate::day::Day;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            N => (x, y - 1),
            E => (x + 1, y),
            S => (x, y + 1),
            W => (x - 1, y),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
}

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
            NE => Some((N, E)),
            NS => Some((N, S)),
            NW => Some((N, W)),
            ES => Some((E, S)),
            EW => Some((E, W)),
            SW => Some((S, W)),
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
    grid: Vec<Vec<Pipe>>,
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
            grid[sy - 1][sx].connects_to(S),
            grid[sy][sx + 1].connects_to(W),
            grid[sy + 1][sx].connects_to(N),
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
            grid,
            path: Cell::new(Vec::new()),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (sx, sy) = self.start;
        let (mut direction, mut current) = [N, E, S, W]
            .into_iter()
            .map(|d| (d, d.apply((sx, sy))))
            .find(|&(d, (x, y))| self.grid[y][x].connects_to(d.opposite()))
            .unwrap();
        let mut path = vec![current];

        while current != self.start {
            let (x, y) = current;
            direction = self.grid[y][x].next(direction.opposite());
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
        let path_set = FnvHashSet::from_iter(path.iter().copied());

        (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .scan((0, Option::None), |(crosses, previous_bend_direction), x| {
                        Some(if path_set.contains(&(x, y)) {
                            match self.grid[y][x] {
                                NS => *crosses += 1,
                                NE | NW => handle_bend((crosses, previous_bend_direction), N),
                                ES | SW => handle_bend((crosses, previous_bend_direction), S),
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
