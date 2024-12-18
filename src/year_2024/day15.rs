use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Empty,
    Box(bool),
    Wall,
}
use Cell::*;

impl Cell {
    fn is_box(&self) -> bool {
        matches!(self, Box(_))
    }
}

pub struct Day15 {
    robot: (usize, usize),
    grid: Grid<Cell>,
    instructions: Vec<Direction>,
}

fn can_push(grid: &Grid<Cell>, position: (usize, usize), direction: Direction) -> bool {
    let (x, y) = position;
    if grid[position] == Box(false) {
        return can_push(grid, (x - 1, y), direction);
    }

    let left = direction.apply(position);
    let right = direction.apply((x + 1, y));

    match (grid[left], grid[right]) {
        (Wall, _) | (_, Wall) => false,
        (l, r) => (!l.is_box() || can_push(grid, left, direction)) && (!r.is_box() || can_push(grid, right, direction)),
    }
}

fn push(grid: &mut Grid<Cell>, position: (usize, usize), direction: Direction) {
    let (x, y) = position;
    if grid[position] == Box(false) {
        push(grid, (x - 1, y), direction);
        return;
    }

    let left = direction.apply(position);
    if grid[left].is_box() {
        push(grid, left, direction);
    }
    let right = direction.apply((x + 1, y));
    if grid[right].is_box() {
        push(grid, right, direction);
    }

    grid[left] = Box(true);
    grid[right] = Box(false);
    grid[position] = Empty;
    grid[(x + 1, y)] = Empty;
}

fn sum_coordinates(grid: Grid<Cell>) -> usize {
    grid.indices()
        .filter(|&i| grid[i] == Box(true))
        .map(|(x, y)| x + 100 * y)
        .sum()
}

impl Day<'_> for Day15 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (grid, instructions) = input.split_once("\n\n").unwrap();

        Self {
            robot: grid
                .lines()
                .enumerate()
                .find_map(|(y, l)| l.bytes().enumerate().find(|&(_, c)| c == b'@').map(|(x, _)| (x, y)))
                .unwrap(),
            grid: Grid::parse(grid, |c| match c {
                b'#' => Wall,
                b'.' | b'@' => Empty,
                b'O' => Box(true),
                _ => unreachable!("unexpected character {}", c),
            }),
            instructions: instructions
                .bytes()
                .filter(|&c| c != b'\n')
                .map(Direction::parse)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut grid = self.grid.clone();
        let mut robot = self.robot;

        for &d in self.instructions.iter() {
            let n = grid.next_index(robot, d).unwrap();
            match grid[n] {
                Wall => {}
                Empty => robot = n,
                Box(_) => {
                    let mut m = n;
                    while grid[m].is_box() {
                        m = grid.next_index(m, d).unwrap();
                    }
                    if grid[m] == Empty {
                        grid[m] = Box(false);
                        grid[n] = Empty;
                        robot = n;
                    }
                }
            }
        }

        sum_coordinates(grid)
    }

    fn part_2(&self) -> Self::T2 {
        let mut grid = Grid::new(
            self.grid
                .rows()
                .map(|row| {
                    row.iter()
                        .flat_map(|&c| {
                            if c.is_box() {
                                [Box(true), Box(false)].into_iter()
                            } else {
                                [c, c].into_iter()
                            }
                        })
                        .collect()
                })
                .collect(),
        );
        let mut robot = self.robot;
        robot.0 *= 2;

        for &direction in self.instructions.iter() {
            let n = grid.next_index(robot, direction).unwrap();
            match grid[n] {
                Wall => {}
                Empty => robot = n,
                Box(_) => {
                    if direction == Left || direction == Right {
                        let mut m = n;
                        while grid[m].is_box() {
                            m = grid.next_index(m, direction).unwrap();
                        }
                        if grid[m] == Empty {
                            while m != n {
                                let p = direction.opposite().apply(m);
                                grid[m] = grid[p];
                                m = p
                            }
                            grid[n] = Empty;
                            robot = n;
                        }
                    } else if can_push(&grid, n, direction) {
                        push(&mut grid, n, direction);
                        robot = n;
                    }
                }
            }
        }

        sum_coordinates(grid)
    }
}
