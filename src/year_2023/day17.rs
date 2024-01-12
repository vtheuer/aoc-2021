use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;

pub struct Day17 {
    grid: Grid<u8>,
}

#[derive(Copy, Clone, Eq)]
struct State {
    x: usize,
    y: usize,
    straight: u8,
    direction: Direction,
    heat_loss: usize,
}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        let &Self {
            x,
            y,
            straight,
            direction,
            heat_loss: _,
        } = self;
        x == other.x && y == other.y && straight == other.straight && direction == other.direction
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.straight.hash(state);
        self.direction.hash(state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss
            .cmp(&other.heat_loss)
            .reverse()
            .then_with(|| (self.x + self.y).cmp(&(other.x + other.y)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day17 {
    fn forward(
        &self,
        &State {
            x,
            y,
            straight,
            direction,
            heat_loss,
        }: &State,
        max_straight: u8,
    ) -> Option<State> {
        if straight >= max_straight {
            None
        } else {
            self.grid.next_index((x, y), direction).map(|(nx, ny)| State {
                x: nx,
                y: ny,
                straight: straight + 1,
                direction,
                heat_loss,
            })
        }
    }

    fn turn(
        &self,
        &State {
            x,
            y,
            straight,
            direction: _,
            heat_loss,
        }: &State,
        direction: Direction,
        min_straight: u8,
    ) -> Option<State> {
        if straight < min_straight {
            None
        } else {
            self.grid.next_index((x, y), direction).map(|(nx, ny)| State {
                x: nx,
                y: ny,
                straight: 1,
                direction,
                heat_loss,
            })
        }
    }

    fn neighbors<'a>(&self, state: &'a State, min_straight: u8, max_straight: u8) -> impl Iterator<Item = State> + 'a {
        vec![
            self.forward(state, max_straight),
            self.turn(state, state.direction.turn_right(), min_straight),
            self.turn(state, state.direction.turn_left(), min_straight),
        ]
        .into_iter()
        .flatten()
    }

    fn shortest_path(&self, min_straight: u8, max_straight: u8) -> usize {
        let mut queue = BinaryHeap::new();
        let mut heat_losses: HashMap<State, usize> = HashMap::default();

        queue.push(State {
            x: 0,
            y: 0,
            straight: 0,
            direction: Down,
            heat_loss: 0,
        });
        queue.push(State {
            x: 0,
            y: 0,
            straight: 0,
            direction: Right,
            heat_loss: 0,
        });

        while let Some(current) = queue.pop() {
            if current.x == self.grid.width - 1 && current.y == self.grid.height - 1 && current.straight >= min_straight
            {
                return current.heat_loss;
            }

            if current.heat_loss > *heat_losses.get(&current).unwrap_or(&usize::MAX) {
                continue;
            }

            for mut neighbor in self.neighbors(&current, min_straight, max_straight) {
                let new_heat_loss = current.heat_loss + self.grid[(neighbor.x, neighbor.y)] as usize;
                let neighbor_heat_loss = heat_losses.entry(neighbor).or_insert(usize::MAX);

                if new_heat_loss < *neighbor_heat_loss {
                    *neighbor_heat_loss = new_heat_loss;
                    neighbor.heat_loss = new_heat_loss;
                    queue.push(neighbor);
                }
            }
        }

        unreachable!()
    }
}

impl Day<'_> for Day17 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: Grid::parse(input, |b| b - b'0'),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.shortest_path(1, 3)
    }

    fn part_2(&self) -> Self::T2 {
        self.shortest_path(4, 10)
    }
}
