use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::convert::identity;
use std::hash::{Hash, Hasher};

use fnv::FnvHashMap;

use Direction::*;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day17 {
    grid: Vec<Vec<u8>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

    fn mask(&self, straight: u8) -> u16 {
        if straight > 2 {
            panic!()
        }
        let shift = match self {
            Up => 0,
            Right => 4,
            Down => 8,
            Left => 12,
        };
        (1 << shift) | (1 << straight)
    }
}

#[derive(Copy, Clone, Debug)]
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

impl Eq for State {}

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
        self.heat_loss.cmp(&other.heat_loss).reverse()
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

    fn turn_left(&self, state: &State) -> Option<State> {
        self.turn(
            state,
            match state.direction {
                Up => Left,
                Right => Up,
                Down => Right,
                Left => Down,
            },
        )
    }

    fn turn_right(&self, state: &State) -> Option<State> {
        self.turn(
            state,
            match state.direction {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        )
    }

    fn turn(
        &self,
        &State {
            x,
            y,
            straight: _,
            direction: _,
            heat_loss,
        }: &State,
        direction: Direction,
    ) -> Option<State> {
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

    fn neighbors<'a>(&self, state: &'a State, max_straight: u8) -> impl Iterator<Item = State> + 'a {
        vec![
            self.forward(state, max_straight),
            self.turn_right(state),
            self.turn_left(state),
        ]
        .into_iter()
        .flatten()
    }

    fn shortest_path(&self, max_straight: u8) -> usize {
        let width = self.grid[0].len();
        let height = self.grid.len();

        let mut queue = BinaryHeap::new();
        let mut prev: FnvHashMap<State, State> = FnvHashMap::default();
        let mut heat_losses: FnvHashMap<State, usize> = FnvHashMap::default();

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
            if current.x == width - 1 && current.y == height - 1 {
                let mut result = self
                    .grid
                    .iter()
                    .map(|row| row.iter().map(|&c| b'0' + c).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let mut c = &current;
                while let Some(p) = prev.get(c) {
                    result[p.y][p.x] = match p.direction {
                        Up => b'^',
                        Right => b'>',
                        Down => b'v',
                        Left => b'<',
                    };
                    c = p;
                }
                println!(
                    "{}",
                    result
                        .into_iter()
                        .map(|row| row.into_iter().map(|c| c as char).collect::<String>())
                        .join("\n")
                );
                return current.heat_loss;
            }

            if current.heat_loss > *heat_losses.get(&current).unwrap_or(&usize::MAX) {
                continue;
            }

            for mut neighbor in self.neighbors(&current, max_straight) {
                let new_heat_loss = current.heat_loss + self.grid[neighbor.y][neighbor.x] as usize;
                let neighbor_heat_loss = heat_losses.entry(neighbor).or_insert(usize::MAX);

                if new_heat_loss < *neighbor_heat_loss {
                    *neighbor_heat_loss = new_heat_loss;
                    neighbor.heat_loss = new_heat_loss;
                    prev.insert(neighbor, current);
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
        self.shortest_path(3)
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "112999
911111";

        dbg!(Day17::new(input).part_1());
    }
}
