use std::cell::Cell;

use ahash::AHashSet;

use crate::day::Day;

pub struct Day04 {
    numbers: Vec<usize>,
    boards: Vec<Vec<Vec<usize>>>,
}

struct Board {
    grid: Vec<Vec<Cell<(usize, bool)>>>,
}

impl Board {
    fn new(board: &[Vec<usize>]) -> Self {
        Self {
            grid: board
                .iter()
                .map(|row| row.iter().map(|&n| Cell::new((n, false))).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    fn play_and_wins(&self, n: usize) -> bool {
        if let Some(c) = self.grid.iter().find_map(|row| row.iter().find(|c| c.get().0 == n)) {
            c.set((n, true));

            self.grid.iter().any(|row| row.iter().all(|c| c.get().1))
                || (0..self.grid[0].len()).any(|i| self.grid.iter().all(|row| row[i].get().1))
        } else {
            false
        }
    }

    fn score(&self, n: usize) -> usize {
        n * self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(Cell::get)
                    .filter_map(|(n, b)| if b { None } else { Some(n) })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Day<'_> for Day04 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut lines = input.lines().filter(|l| !l.is_empty());
        Self {
            numbers: lines
                .next()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
            boards: lines
                .collect::<Vec<_>>()
                .chunks(5)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|l| {
                            l.split(' ')
                                .filter(|p| !p.is_empty())
                                .map(str::parse)
                                .map(Result::unwrap)
                                .collect()
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let boards = self.boards.iter().map(|board| Board::new(board)).collect::<Vec<_>>();
        self.numbers
            .iter()
            .find_map(|&n| {
                boards.iter().find_map(|board| {
                    if board.play_and_wins(n) {
                        Some(board.score(n))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        let boards = self.boards.iter().map(|board| Board::new(board)).collect::<Vec<_>>();
        self.numbers
            .iter()
            .scan(AHashSet::from_iter(0..boards.len()), |indices, &n| {
                let remaining_indices = indices
                    .iter()
                    .copied()
                    .filter(|&i| !boards[i].play_and_wins(n))
                    .collect::<AHashSet<_>>();

                if remaining_indices.is_empty() {
                    Some(Some(boards[*indices.iter().next().unwrap()].score(n)))
                } else {
                    *indices = remaining_indices;
                    Some(None)
                }
            })
            .find_map(|score| score)
            .unwrap()
    }
}
