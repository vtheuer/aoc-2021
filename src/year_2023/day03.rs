use crate::day::Day;

pub struct Day03<'a> {
    grid: Vec<&'a [u8]>,
}

fn overlaps((a1, a2, ya): (usize, usize, usize), (b1, b2, yb): (usize, usize, usize)) -> bool {
    ya == yb && a1 <= b2 && b1 <= a2
}

impl<'a> Day03<'a> {
    fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 || x >= self.grid[0].len() as isize || y >= self.grid.len() as isize {
            0
        } else {
            self.grid[y as usize][x as usize]
        }
    }

    fn is_symbol(&self, x: isize, y: isize) -> bool {
        let c = self.get(x, y);
        c != b'.' && !c.is_ascii_digit()
    }

    fn read(&self, x: isize, y: isize) -> (usize, (usize, usize, usize)) {
        let mut start = x;
        while self.get(start, y).is_ascii_digit() {
            start -= 1;
        }
        start += 1;
        let mut end = x;
        while self.get(end, y).is_ascii_digit() {
            end += 1;
        }
        end -= 1;

        let ustart = start as usize;
        let uend = end as usize;
        (
            std::str::from_utf8(&self.grid[y as usize][ustart..=uend])
                .unwrap()
                .parse()
                .unwrap(),
            (ustart, uend, y as usize),
        )
    }

    fn gear_ratio(&self, x: isize, y: isize) -> Option<usize> {
        let mut numbers = (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
            .filter(|&n| n != (x, y))
            .filter(|&(nx, ny)| self.get(nx, ny).is_ascii_digit())
            .map(|(nx, ny)| self.read(nx, ny))
            .fold(Vec::new(), |mut numbers: Vec<(usize, (usize, usize, usize))>, n| {
                let len = numbers.len();
                if len == 0 || len == 1 && !overlaps(n.1, numbers[0].1) {
                    numbers.push(n);
                }
                numbers
            })
            .into_iter()
            .map(|(n, _)| n);
        Some(numbers.next()? * numbers.next()?)
    }
}

impl<'a> Day<'a> for Day03<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            grid: input.lines().map(|l| l.as_bytes()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut sum_of_parts = 0;

        for (y, &row) in self.grid.iter().enumerate() {
            let mut x = 0;

            while x < row.len() {
                if row[x].is_ascii_digit() {
                    let start = x;
                    while x < row.len() && row[x].is_ascii_digit() {
                        x += 1;
                    }

                    let ix = x as isize;
                    let iy = y as isize;
                    let is = start as isize;
                    if self.is_symbol(is - 1, iy)
                        || self.is_symbol(ix, iy)
                        || (is - 1..=ix).any(|t| self.is_symbol(t, iy - 1))
                        || (is - 1..=ix).any(|t| self.is_symbol(t, iy + 1))
                    {
                        sum_of_parts += std::str::from_utf8(&row[start..x]).unwrap().parse::<usize>().unwrap();
                    }
                } else {
                    x += 1;
                }
            }
        }

        sum_of_parts
    }

    fn part_2(&self) -> Self::T2 {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, &row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == b'*')
                    .map(move |(x, _)| (x, y))
            })
            .filter_map(|(x, y)| self.gear_ratio(x as isize, y as isize))
            .sum()
    }
}
