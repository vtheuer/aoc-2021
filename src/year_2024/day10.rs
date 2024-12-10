use crate::day::Day;
use crate::util::direction::Direction::*;
use crate::util::grid::Grid;
use ahash::AHashSet;
use std::collections::VecDeque;
use std::convert::identity;

pub struct Day10 {
    grid: Grid<u8>,
}

impl Day10 {
    fn count_paths<Nine>(
        &self,
        init: fn() -> Nine,
        add: fn(&mut Nine, (usize, usize)),
        count: fn(Nine) -> usize,
    ) -> usize {
        self.grid
            .indices()
            .filter(|&p| self.grid[p] == 0)
            .map(|p| {
                let mut heads = VecDeque::new();
                heads.push_back(p);
                let mut nines = init();
                while let Some(h) = heads.pop_front() {
                    let v = self.grid[h];
                    if v == 9 {
                        add(&mut nines, h);
                    } else {
                        for n in [Up, Right, Down, Left]
                            .into_iter()
                            .filter_map(|direction| self.grid.next_index(h, direction))
                            .filter(|&n| self.grid[n] == v + 1)
                        {
                            heads.push_back(n);
                        }
                    }
                }
                count(nines)
            })
            .sum()
    }
}

impl Day<'_> for Day10 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: Grid::parse(input, |c| c - b'0'),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_paths(
            AHashSet::default,
            |nines, h| {
                nines.insert(h);
            },
            |nines| nines.len(),
        )
    }

    fn part_2(&self) -> Self::T2 {
        self.count_paths(|| 0, |nines, _| *nines += 1, identity)
    }
}
