use num::Num;
use std::ops::{Add, Sub};
use Direction::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn apply<T>(&self, (x, y): (T, T)) -> (T, T)
    where
        T: Num + Sub<Output = T> + Add<Output = T>,
    {
        let one = num::one();
        match self {
            Up => (x, y - one),
            Right => (x + one, y),
            Down => (x, y + one),
            Left => (x - one, y),
        }
    }
}
