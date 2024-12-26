use crate::day::Day;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const MAX: usize = 70;

pub struct Day18 {
    ram: Grid<Option<usize>>,
    pub byte_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    S(usize, (usize, usize)),
}

use State::*;

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let &S(d, (x, y)) = self;
        let &S(od, (ox, oy)) = other;

        d.cmp(&od).reverse().then((x + y).cmp(&(ox + oy)))
    }
}

impl Day18 {
    fn is_reachable(&self, i: usize) -> Option<usize> {
        let mut queue = BinaryHeap::from([S(0, (0, 0))]);
        let mut distances = Grid::init(self.ram.width, self.ram.height, usize::MAX);

        while let Some(S(distance, position)) = queue.pop() {
            if distance < distances[position] {
                distances[position] = distance;
            } else {
                continue;
            }

            if position == (MAX, MAX) {
                return Some(distance);
            }

            for next in [Up, Right, Down, Left]
                .into_iter()
                .filter_map(|d| self.ram.next_index(position, d))
                .filter(|&p| self.ram[p].map(|b| b > i).unwrap_or(true))
                .filter(|&p| distances[p] > distance)
            {
                queue.push(S(distance + 1, next));
            }
        }

        None
    }
}

impl Day<'_> for Day18 {
    type T1 = usize;
    type T2 = String;

    fn new(input: &str) -> Self {
        Self {
            byte_count: input.lines().count(),
            ram: input
                .lines()
                .filter_map(|l| l.split_once(','))
                .filter_map(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
                .enumerate()
                .fold(Grid::init(MAX + 1, MAX + 1, None), |mut ram, (i, b)| {
                    ram[b] = Some(i);
                    ram
                }),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.is_reachable(1024).unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        let mut a = 0;
        let mut b = self.byte_count - 1;

        while a <= b {
            let i = (a + b) / 2;
            if self.is_reachable(i).is_some() {
                a = i + 1;
            } else {
                b = i - 1;
            }
        }

        let (x, y) = self.ram.indices().find(|&i| self.ram[i] == Some(a)).unwrap();
        format!("{x},{y}")
    }
}
