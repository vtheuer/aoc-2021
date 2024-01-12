use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use Direction::*;

use crate::day::Day;

pub struct Day17 {
    grid: Vec<Vec<u8>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    fn apply(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }
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
        let width = self.grid[0].len() as isize;
        let height = self.grid.len() as isize;
        if straight >= max_straight {
            None
        } else {
            let (nx, ny) = direction.apply((x as isize, y as isize));
            if (0..width).contains(&nx) && (0..height).contains(&ny) {
                Some(State {
                    x: nx as usize,
                    y: ny as usize,
                    straight: straight + 1,
                    direction,
                    heat_loss,
                })
            } else {
                None
            }
        }
    }

    fn turn_left(&self, state: &State, min_straight: u8) -> Option<State> {
        self.turn(
            state,
            match state.direction {
                Up => Left,
                Right => Up,
                Down => Right,
                Left => Down,
            },
            min_straight,
        )
    }

    fn turn_right(&self, state: &State, min_straight: u8) -> Option<State> {
        self.turn(
            state,
            match state.direction {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
            min_straight,
        )
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
            return None;
        }

        let width = self.grid[0].len() as isize;
        let height = self.grid.len() as isize;
        let (nx, ny) = direction.apply((x as isize, y as isize));
        if (0..width).contains(&nx) && (0..height).contains(&ny) {
            Some(State {
                x: nx as usize,
                y: ny as usize,
                straight: 1,
                direction,
                heat_loss,
            })
        } else {
            None
        }
    }

    fn neighbors<'a>(&self, state: &'a State, min_straight: u8, max_straight: u8) -> impl Iterator<Item = State> + 'a {
        vec![
            self.forward(state, max_straight),
            self.turn_right(state, min_straight),
            self.turn_left(state, min_straight),
        ]
        .into_iter()
        .flatten()
    }

    fn shortest_path(&self, min_straight: u8, max_straight: u8) -> usize {
        let width = self.grid[0].len();
        let height = self.grid.len();

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
            if current.x == width - 1 && current.y == height - 1 && current.straight >= min_straight {
                return current.heat_loss;
            }

            if current.heat_loss > *heat_losses.get(&current).unwrap_or(&usize::MAX) {
                continue;
            }

            for mut neighbor in self.neighbors(&current, min_straight, max_straight) {
                let new_heat_loss = current.heat_loss + self.grid[neighbor.y][neighbor.x] as usize;
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
            grid: input.lines().map(|l| l.bytes().map(|b| b - b'0').collect()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.shortest_path(1, 3)
    }

    fn part_2(&self) -> Self::T2 {
        self.shortest_path(4, 10)
    }
}
