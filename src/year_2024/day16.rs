use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;
use crate::util::Joinable;
use ahash::{AHashMap, AHashSet};
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::ops::Index;
use Ordering::*;

pub struct Day16 {
    grid: Grid<bool>,
    start: (usize, usize),
    end: (usize, usize),
    path_len: Cell<usize>,
}

#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
    prev: ((usize, usize), Direction),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day<'_> for Day16 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (start, end) = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter(|&(_, c)| c == b'S' || c == b'E')
                    .map(move |(x, c)| ((x, y), c))
            })
            .fold(((0, 0), (0, 0)), |(start, end), (position, c)| match c {
                b'S' => (position, end),
                b'E' => (start, position),
                _ => (start, end),
            });
        Self {
            grid: Grid::parse(input, |c| c == b'#'),
            start,
            end,
            path_len: Cell::new(0),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut queue = BinaryHeap::new();
        queue.push(State {
            position: self.start,
            direction: Right,
            cost: 0,
            prev: ((0, 0), Right),
        });
        let mut costs = Grid::init_with(self.grid.width, self.grid.height, || vec![usize::MAX; 4]);
        let mut predecessors: AHashMap<((usize, usize), Direction), AHashSet<((usize, usize), Direction)>> =
            AHashMap::new();

        while let Some(State {
            position,
            direction,
            cost,
            prev,
        }) = queue.pop()
        {
            match cost.cmp(&costs[position][direction.ordinal()]) {
                Less => {
                    costs[position][direction.ordinal()] = cost;
                    predecessors.insert((position, direction), AHashSet::from([prev]));
                }
                Equal => {
                    predecessors
                        .entry((position, direction))
                        .or_insert_with(AHashSet::new)
                        .insert(prev);
                }
                Greater => {
                    continue;
                }
            }

            if position == self.end {
                predecessors
                    .entry((position, direction))
                    .or_insert_with(AHashSet::new)
                    .insert(prev);
                let mut todo = VecDeque::from([(position, direction)]);
                let mut path = AHashSet::from([self.start]);
                while let Some(p) = todo.pop_front() {
                    if p.0 == self.start {
                        break;
                    }
                    path.insert(p.0);
                    for &predecessor in &predecessors[&p] {
                        todo.push_back(predecessor);
                    }
                }
                self.path_len.set(path.len());
                return cost;
            }

            if !self.grid.next(position, direction).unwrap() {
                queue.push(State {
                    position: direction.apply(position),
                    direction,
                    cost: cost + 1,
                    prev: (position, direction),
                });
            }

            let left = direction.turn_left();
            if !self.grid.next(position, left).unwrap() {
                queue.push(State {
                    position,
                    direction: left,
                    cost: cost + 1000,
                    prev: (position, direction),
                });
            }

            let right = direction.turn_right();
            if !self.grid.next(position, right).unwrap() {
                queue.push(State {
                    position,
                    direction: right,
                    cost: cost + 1000,
                    prev: (position, direction),
                });
            }
        }

        unreachable!()
    }

    fn part_2(&self) -> Self::T2 {
        self.path_len.get()
    }
}
