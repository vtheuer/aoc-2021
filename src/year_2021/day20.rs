use crate::day::Day;

pub struct Day20 {
    enhancement: Vec<bool>,
    grid: Vec<Vec<bool>>,
}

impl Day20 {
    fn enhance_pixel(&self, grid: &[Vec<bool>], x: isize, y: isize, w: isize, h: isize, default_state: bool) -> bool {
        let index = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter()
        .map(|(x, y)| {
            if x >= 0 && y >= 0 && x < w && y < h {
                grid[y as usize][x as usize]
            } else {
                default_state
            }
        })
        .fold(0, |i, b| (i << 1) | if b { 1 } else { 0 });
        self.enhancement[index]
    }

    fn enhance(&self, grid: &[Vec<bool>], default_state: bool) -> Vec<Vec<bool>> {
        let w = grid[0].len() as isize;
        let h = grid.len() as isize;
        (-1..=h)
            .map(|y| {
                (-1..=w)
                    .map(|x| self.enhance_pixel(grid, x, y, w, h, default_state))
                    .collect()
            })
            .collect()
    }

    fn solve(&self, steps: usize) -> usize {
        (1..=steps)
            .fold(self.grid.clone(), |grid, step| self.enhance(&grid, step % 2 == 0))
            .into_iter()
            .map(|row| row.into_iter().filter(|&b| b).count())
            .sum()
    }
}

impl Day<'_> for Day20 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (enhancement, grid) = input.split_once("\n\n").unwrap();
        let parse_line: fn(&str) -> Vec<bool> = |l| l.bytes().map(|b| b == b'#').collect();
        Self {
            enhancement: parse_line(enhancement),
            grid: grid.lines().map(parse_line).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.solve(2)
    }

    fn part_2(&self) -> Self::T2 {
        self.solve(50)
    }
}
