use crate::day::Day;
use fnv::FnvHashSet;
use std::hash::Hash;

pub struct Day17 {
    space: FnvHashSet<(i8, i8)>,
}

fn adjacent(n: i8) -> impl Iterator<Item = i8> {
    n - 1..=n + 1
}

fn neighbors_3d(&(x, y, z): &(i8, i8, i8)) -> impl Iterator<Item = (i8, i8, i8)> {
    adjacent(x)
        .flat_map(move |nx| adjacent(y).flat_map(move |ny| adjacent(z).map(move |nz| (nx, ny, nz))))
}

fn neighbors_4d(&(x, y, z, w): &(i8, i8, i8, i8)) -> impl Iterator<Item = (i8, i8, i8, i8)> {
    adjacent(w)
        .flat_map(move |nw| neighbors_3d(&(x, y, z)).map(move |(nx, ny, nz)| (nx, ny, nz, nw)))
}

fn run<T, F, I>(mut space: FnvHashSet<T>, neighbors: F) -> usize
where
    T: Clone + Eq + Hash + Copy,
    F: Fn(&T) -> I,
    I: Iterator<Item = T>,
{
    for _ in 0..6 {
        space = space
            .iter()
            .flat_map(|cell| neighbors(cell))
            .collect::<FnvHashSet<_>>()
            .into_iter()
            .filter_map(|cell| {
                let active_neighbors = neighbors(&cell)
                    .filter(|&n| n != cell)
                    .filter(|n| space.contains(n))
                    .take(4)
                    .count();
                if active_neighbors == 3 || space.contains(&cell) && active_neighbors == 2 {
                    Some(cell)
                } else {
                    None
                }
            })
            .collect();
    }

    space.len()
}

impl Day<'_> for Day17 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day17 {
            space: input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.bytes()
                        .enumerate()
                        .filter(|&(_, c)| c == b'#')
                        .map(move |(x, _)| (x as i8, y as i8))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        run(
            self.space.iter().map(|&(x, y)| (x, y, 0)).collect(),
            neighbors_3d,
        )
    }

    fn part_2(&self) -> Self::T2 {
        run(
            self.space.iter().map(|&(x, y)| (x, y, 0, 0)).collect(),
            neighbors_4d,
        )
    }
}
