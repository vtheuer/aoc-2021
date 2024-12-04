use crate::day::Day;
use crate::util::grid::Grid;

pub struct Day04 {
    grid: Grid<u8>,
}

impl Day04 {
    fn is_xmas(&self, (mut x, mut y): (isize, isize), (dx, dy): (isize, isize)) -> bool {
        for &c in b"MAS" {
            x += dx;
            y += dy;
            if !self.grid.contains((x, y)) || self.grid[(x as usize, y as usize)] != c {
                return false;
            }
        }
        true
    }
}

impl Day<'_> for Day04 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            grid: Grid::parse(input, |c| c),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.grid
            .indices()
            .filter(|&(x, y)| self.grid[(x, y)] == b'X')
            .map(|(x, y)| {
                [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]
                    .into_iter()
                    .filter(|&d| self.is_xmas((x as isize, y as isize), d))
                    .count()
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let mut count = 0;
        for x in 1..self.grid.width - 1 {
            for y in 1..self.grid.height - 1 {
                if self.grid[(x, y)] == b'A' {
                    let x = [(x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1), (x + 1, y + 1)]
                        .into_iter()
                        .map(|n| self.grid[n])
                        .collect::<Vec<_>>();
                    if matches!(x.as_slice(), b"MMSS" | b"SSMM" | b"MSMS" | b"SMSM") {
                        count += 1
                    }
                }
            }
        }
        count
    }
}
