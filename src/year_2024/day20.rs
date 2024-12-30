use crate::day::Day;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;

pub struct Day20 {
    distances: Grid<Option<usize>>,
    path: Vec<(usize, usize)>,
}

impl Day20 {
    fn count_cheats(&self, cheat_distance: isize) -> usize {
        let deltas = (-cheat_distance..=cheat_distance)
            .flat_map(|y| {
                let abs_y = y.abs();
                (-(cheat_distance - abs_y)..=(cheat_distance - abs_y))
                    .map(move |x| (x, y))
                    .filter(|&d| d != (0, 0))
            })
            .collect::<Vec<_>>();

        self.path
            .iter()
            .copied()
            .enumerate()
            .map(|(i, (x, y))| {
                deltas
                    .iter()
                    .copied()
                    .map(|(dx, dy)| (x as isize + dx, y as isize + dy, dx.abs() + dy.abs()))
                    .filter_map(|(nx, ny, cheat)| {
                        self.distances
                            .get((nx as usize, ny as usize))
                            .flatten()
                            .map(|d| (d, cheat))
                    })
                    .filter(|&(distance, cheat)| distance >= i + 100 + cheat as usize)
                    .count()
            })
            .sum()
    }
}

impl Day<'_> for Day20 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let grid = Grid::parse(input, |c| c == b'#');
        let (start, end) = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter(|&(_, c)| c == b'S' || c == b'E')
                    .map(move |(x, c)| ((x, y), c))
            })
            .fold(((0, 0), (0, 0)), |(start, end), (position, c)| match c {
                b'S' => (position, end),
                b'E' => (start, position),
                _ => (start, end),
            });
        let mut position = start;
        let mut distance = 0;
        let mut distances = Grid::init(grid.width, grid.height, None);
        let mut path = vec![position];
        distances[position] = Some(distance);

        while position != end {
            distance += 1;
            position = [Up, Right, Down, Left]
                .into_iter()
                .filter_map(|d| grid.next_index(position, d))
                .filter(|&next| !grid[next])
                .find(|&next| distances[next].is_none())
                .unwrap();
            distances[position] = Some(distance);
            path.push(position);
        }

        Self { distances, path }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_cheats(2)
    }

    fn part_2(&self) -> Self::T2 {
        self.count_cheats(20)
    }
}
