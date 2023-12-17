use Direction::*;
use Element::*;

use crate::day::Day;

#[derive(Copy, Clone)]
enum Element {
    HorizontalSplitter,
    VerticalSplitter,
    LeftReflector,
    RightReflector,
}

impl Element {
    fn is_splitter(&self) -> bool {
        matches!(self, HorizontalSplitter | VerticalSplitter)
    }

    fn splits(&self, direction: Direction) -> bool {
        match self {
            HorizontalSplitter => matches!(direction, Up | Down),
            VerticalSplitter => matches!(direction, Left | Right),
            _ => false,
        }
    }

    fn reflects_left(&self, direction: Direction) -> bool {
        match self {
            LeftReflector => matches!(direction, Left | Right),
            RightReflector => matches!(direction, Up | Down),
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn mask(&self) -> u8 {
        match self {
            Up => 0b0001,
            Right => 0b0010,
            Down => 0b0100,
            Left => 0b1000,
        }
    }
}

pub struct Day16 {
    grid: Vec<Vec<Option<Element>>>,
}

type Beam = (usize, usize, Direction);

impl Day16 {
    fn is_in_grid(&self, (x, y): &(isize, isize)) -> bool {
        (0..self.grid[0].len() as isize).contains(x) && (0..self.grid.len() as isize).contains(y)
    }

    fn next(&self, (x, y, direction): Beam) -> Vec<Beam> {
        let ix = x as isize;
        let iy = y as isize;
        Some(match direction {
            Up => (ix, iy - 1),
            Right => (ix + 1, iy),
            Down => (ix, iy + 1),
            Left => (ix - 1, iy),
        })
        .filter(|next| self.is_in_grid(next))
        .map(|(nx, ny)| (nx as usize, ny as usize))
        .map(|(nx, ny)| {
            (
                nx,
                ny,
                match self.grid[ny][nx] {
                    None => vec![direction],
                    Some(e) => {
                        if e.is_splitter() {
                            if e.splits(direction) {
                                vec![direction.left(), direction.right()]
                            } else {
                                vec![direction]
                            }
                        } else if e.reflects_left(direction) {
                            vec![direction.left()]
                        } else {
                            vec![direction.right()]
                        }
                    }
                },
            )
        })
        .map(|(nx, ny, directions)| directions.into_iter().map(|d| (nx, ny, d)).collect())
        .unwrap_or_default()
    }

    fn count_energized(&self, start: Beam) -> usize {
        let mut beams = vec![start];
        let mut energized = vec![vec![0u8; self.grid[0].len()]; self.grid.len()];

        while let Some((x, y, direction)) = beams.pop() {
            energized[y][x] |= direction.mask();
            self.next((x, y, direction))
                .into_iter()
                .filter(|&(nx, ny, d)| energized[ny][nx] & d.mask() == 0)
                .for_each(|beam| beams.push(beam));
        }

        energized
            .into_iter()
            .map(|row| row.into_iter().filter(|&e| e > 0).count())
            .sum()
    }
}

impl Day<'_> for Day16 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|l| {
                    l.bytes()
                        .map(|b| match b {
                            b'.' => None,
                            b'-' => Some(HorizontalSplitter),
                            b'|' => Some(VerticalSplitter),
                            b'/' => Some(LeftReflector),
                            b'\\' => Some(RightReflector),
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_energized((0, 0, Right))
    }

    fn part_2(&self) -> Self::T2 {
        let height = self.grid.len();
        let width = self.grid[0].len();
        (0..height)
            .flat_map(|y| [(0, y, Right), (width - 1, y, Left)])
            .chain((0..width).flat_map(|x| [(x, 0, Down), (x, height - 1, Up)]))
            .map(|beam| self.count_energized(beam))
            .max()
            .unwrap()
    }
}
