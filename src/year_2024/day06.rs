use crate::day::Day;
use crate::util::direction::Direction;
use crate::util::grid::Grid;
use crate::util::Joinable;

pub struct Day06 {
    grid: Grid<bool>,
    position: (usize, usize),
    direction: Direction,
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

        while visited[y][x] != Some(direction) {
            visited[y][x] = Some(direction);

            if let Some((nx, ny)) = self.grid.next_index((x, y), direction).filter(|&n| !self.grid[n]) {
                (x, y) = (nx, ny);
            } else {
                direction = direction.turn_right();
            }

            println!(
                "{}\n",
                visited
                    .iter()
                    .enumerate()
                    .map(|(py, row)| row
                        .iter()
                        .enumerate()
                        .map(|(px, v)| if v.is_some() {
                            'X'
                        } else if (x, y) == (px, py) {
                            match direction {
                                Direction::Up => '^',
                                Direction::Right => '>',
                                Direction::Down => 'v',
                                Direction::Left => '<',
                            }
                        } else if self.grid[(px, py)] {
                            '#'
                        } else {
                            '.'
                        })
                        .collect::<String>())
                    .join("\n")
            );
        }

        visited
            .into_iter()
            .map(|row| row.into_iter().filter(Option::is_some).count())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
