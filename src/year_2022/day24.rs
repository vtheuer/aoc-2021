use std::cell::Cell;
use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};
use Direction::*;

use crate::day::Day;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn mask(&self) -> u8 {
        match *self {
            Up => 1,
            Right => 0b10,
            Down => 0b100,
            Left => 0b1000,
        }
    }

    fn parse(b: u8) -> Self {
        match b {
            b'^' => Up,
            b'>' => Right,
            b'v' => Down,
            b'<' => Left,
            _ => unreachable!(),
        }
    }
}

const WALL: u8 = u8::MAX;

#[derive(Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    time: usize,
}

pub struct Day24 {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    target: (usize, usize),
    first_trip: Cell<usize>,
}

fn blizzards(cell: u8) -> impl Iterator<Item = Direction> {
    [Up, Right, Down, Left]
        .into_iter()
        .filter(move |&d| cell & d.mask() > 0)
}

fn possible_moves(
    map: &[Vec<u8>],
    (sx, sy): (usize, usize),
    x: usize,
    y: usize,
) -> Box<dyn Iterator<Item = (usize, usize)> + '_> {
    if (x, y) == (sx, sy) {
        Box::new(
            [(sx, if sy == 0 { sy + 1 } else { sy - 1 }), (sx, sy)]
                .into_iter()
                .filter(|&(nx, ny)| map[ny][nx] == 0),
        )
    } else {
        Box::new(
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1), (x, y)]
                .into_iter()
                .filter(move |&(_, ny)| ny != sy)
                .filter(|&(nx, ny)| map[ny][nx] == 0),
        )
    }
}

fn add(value: usize, delta: usize, length: usize) -> usize {
    if value + delta <= length {
        value + delta
    } else {
        (value - 1 + delta) % length + 1
    }
}

fn subtract(value: usize, delta: usize, length: usize) -> usize {
    if value > delta {
        value - delta
    } else {
        let opposite = length - value + 1;
        let opposite_destination = (opposite + delta) % length;
        1 + (length - opposite_destination) % length
    }
}

impl Day24 {
    fn destination(&self, (x, y): (usize, usize), direction: Direction, times: usize) -> (usize, usize) {
        let height = self.map.len() - 2;
        let width = self.map[0].len() - 2;

        match direction {
            Up => (x, subtract(y, times, height)),
            Down => (x, add(y, times, height)),
            Left => (subtract(x, times, width), y),
            Right => (add(x, times, width), y),
        }
    }

    fn at_time(&self, time: usize) -> Vec<Vec<u8>> {
        let mut new_map = vec![vec![0; self.map[0].len()]; self.map.len()];

        for (y, row) in self.map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                match c {
                    WALL => new_map[y][x] = WALL,
                    0 => {}
                    b => {
                        for direction in blizzards(b) {
                            let (nx, ny) = self.destination((x, y), direction, time);
                            new_map[ny][nx] |= direction.mask();
                        }
                    }
                }
            }
        }

        new_map
    }

    fn navigate(&self, start: (usize, usize), target: (usize, usize), start_time: usize) -> usize {
        let mut cache: AHashMap<usize, Vec<Vec<u8>>> = AHashMap::default();
        let mut states = VecDeque::from([State {
            position: start,
            time: start_time,
        }]);
        let mut visited = AHashSet::default();

        while let Some(s) = states.pop_front() {
            let State { position: (x, y), time } = s;
            if visited.contains(&s) {
                continue;
            }

            visited.insert(s);

            let t = time + 1;

            for (nx, ny) in possible_moves(cache.entry(t).or_insert_with(|| self.at_time(t)), start, x, y) {
                if (nx, ny) == target {
                    return t;
                }

                states.push_back(State {
                    position: (nx, ny),
                    time: t,
                });
            }
        }

        unreachable!()
    }
}

impl Day<'_> for Day24 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|c| match c {
                        b'#' => WALL,
                        b'.' => 0,
                        _ => Direction::parse(c).mask(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<u8>>>();
        Self {
            start: (map[0].iter().position(|&c| c == 0).unwrap(), 0),
            target: (map[map.len() - 1].iter().position(|&c| c == 0).unwrap(), map.len() - 1),
            map,
            first_trip: Cell::new(0),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let first_trip = self.navigate(self.start, self.target, 0);
        self.first_trip.set(first_trip);
        first_trip
    }

    fn part_2(&self) -> Self::T2 {
        let second_trip = self.navigate(self.target, self.start, self.first_trip.get());
        self.navigate(self.start, self.target, second_trip)
    }
}
