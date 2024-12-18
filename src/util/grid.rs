use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::util::direction::Direction;
use crate::util::Joinable;

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        Self { grid, height, width }
    }

    pub fn parse(input: &str, map: fn(u8) -> T) -> Self {
        Self::new(input.lines().map(|l| l.bytes().map(map).collect()).collect())
    }

    pub fn init(width: usize, height: usize, v: T) -> Self
    where
        T: Copy,
    {
        Self::new(vec![vec![v; width]; height])
    }

    fn ucontains(&self, (x, y): (usize, usize)) -> bool {
        x < self.width && y < self.height
    }

    pub fn contains<I>(&self, (tx, ty): (I, I)) -> bool
    where
        I: TryInto<usize>,
    {
        tx.try_into()
            .and_then(|x| ty.try_into().map(|y| (x, y)))
            .map(|i| self.ucontains(i))
            .unwrap_or(false)
    }

    pub fn get(&self, i: (usize, usize)) -> Option<T>
    where
        T: Copy,
    {
        if self.ucontains(i) {
            Some(self[i])
        } else {
            None
        }
    }

    pub fn get_next(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<T>
    where
        T: Copy,
    {
        if let Some(next) = x
            .checked_add_signed(dx)
            .filter(|&nx| nx < self.width)
            .and_then(|nx| {
                y.checked_add_signed(dy)
                    .filter(|&ny| ny < self.height)
                    .map(|ny| (nx, ny))
            })
            .filter(|&n| self.ucontains(n))
        {
            Some(self[next])
        } else {
            None
        }
    }

    pub fn next_index(&self, (x, y): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let n = direction.apply((x as isize, y as isize));

        if self.contains(n) {
            let (nx, ny) = n;
            Some((nx as usize, ny as usize))
        } else {
            None
        }
    }

    pub fn next(&self, i: (usize, usize), direction: Direction) -> Option<T>
    where
        T: Copy,
    {
        self.next_index(i, direction).map(|n| self[n])
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.grid.iter()
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.grid[y]
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.rows().flat_map(|row| row.iter())
    }

    pub fn indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| (x, y)))
    }

    pub fn format<R: Display>(&self, format_element: fn(&T) -> R) -> String {
        self.rows()
            .map(|row| row.iter().map(|e| format_element(e).to_string()).join(""))
            .join("\n")
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.rows() {
            for e in row {
                write!(f, "{}", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Copy,
{
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[y][x]
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect::<Vec<_>>())
    }
}
