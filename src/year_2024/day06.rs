use ahash::AHashSet;

use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::grid::Grid;

pub struct Day06 {
    grid: Grid<bool>,
    position: (usize, usize),
    direction: Direction,
}

impl Day06 {
    fn is_infinite_loop(
        &self,
        visited: &[Vec<AHashSet<Direction>>],
        (mut x, mut y): (usize, usize),
        mut direction: Direction,
    ) -> bool {
        let mut sub_visited = visited.to_vec();

        while let Some((nx, ny)) = self.grid.next_index((x, y), direction) {
            if self.grid[(nx, ny)] {
                direction = direction.turn_right();
            } else {
                (x, y) = (nx, ny);
                if sub_visited[y][x].contains(&direction) {
                    return true;
                }
            }

            sub_visited[y][x].insert(direction);
        }

        false
    }
}

impl Day<'_> for Day06 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (position, direction) = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| row.bytes().enumerate().map(move |(x, c)| ((x, y), c)))
            .find(|&(_, c)| c != b'.' && c != b'#')
            .unwrap();
        Self {
            grid: Grid::parse(input, |c| c == b'#'),
            position,
            direction: Direction::parse(direction),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (mut x, mut y) = self.position;
        let mut direction = self.direction;
        let mut visited = vec![vec![None; self.grid.width]; self.grid.height];
        visited[y][x] = Some(direction);

        while let Some((nx, ny)) = self.grid.next_index((x, y), direction) {
            if self.grid[(nx, ny)] {
                direction = direction.turn_right();
            } else {
                (x, y) = (nx, ny);
            }
            visited[y][x] = Some(direction);
        }

        visited
            .into_iter()
            .map(|row| row.into_iter().filter(Option::is_some).count())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let (mut x, mut y) = self.position;
        let mut direction = self.direction;
        let mut visited: Vec<Vec<AHashSet<Direction>>> =
            vec![vec![AHashSet::default(); self.grid.width]; self.grid.height];
        visited[y][x].insert(direction);
        let mut possible_blocks: AHashSet<(usize, usize)> = AHashSet::default();

        while let Some((nx, ny)) = self.grid.next_index((x, y), direction) {
            if self.grid[(nx, ny)] {
                direction = direction.turn_right();
            } else {
                (x, y) = (nx, ny);
                if self.is_infinite_loop(&visited, (x, y), direction.turn_right()) {
                    possible_blocks.insert(self.grid.next_index((x, y), direction).unwrap());
                }
            }
            visited[y][x].insert(direction);

            // println!(
            //     "{}\n",
            //     visited
            //         .iter()
            //         .enumerate()
            //         .map(|(py, row)| row
            //             .iter()
            //             .enumerate()
            //             .map(|(px, v)| if v.is_some() {
            //                 'X'
            //             } else if (x, y) == (px, py) {
            //                 match direction {
            //                     Direction::Up => '^',
            //                     Direction::Right => '>',
            //                     Direction::Down => 'v',
            //                     Direction::Left => '<',
            //                 }
            //             } else if self.grid[(px, py)] {
            //                 '#'
            //             } else {
            //                 '.'
            //             })
            //             .collect::<String>())
            //         .join("\n")
            // );
        }

        possible_blocks.len()
    }
}
