use std::collections::VecDeque;

use crate::day::Day;

pub struct Day12<'a> {
    heightmap: Vec<&'a [u8]>,
    start: (usize, usize),
    end: (usize, usize),
}

impl<'a> Day12<'a> {
    fn height(&self, (x, y): (usize, usize)) -> u8 {
        match self.heightmap[y][x] {
            b'S' => b'a',
            b'E' => b'z',
            h => h,
        }
    }
}

impl<'a> Day<'a> for Day12<'a> {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &'a str) -> Self {
        let heightmap = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
        Self {
            start: heightmap
                .iter()
                .enumerate()
                .find_map(|(y, &row)| {
                    row.iter()
                        .enumerate()
                        .find_map(|(x, &h)| if h == b'S' { Some((x, y)) } else { None })
                })
                .unwrap(),
            end: heightmap
                .iter()
                .enumerate()
                .find_map(|(y, &row)| {
                    row.iter()
                        .enumerate()
                        .find_map(|(x, &h)| if h == b'E' { Some((x, y)) } else { None })
                })
                .unwrap(),
            heightmap,
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.find_distance(
            self.start,
            |p, _| p == self.end,
            |current, next| next <= current || next == current + 1,
        )[self.end.1][self.end.0]
    }

    fn part_2(&self) -> Self::T2 {
        self.find_distance(
            self.end,
            |_, h| h == b'a',
            |current, next| next >= current || next == current - 1,
        )
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &d)| {
                if d != isize::MAX && self.height((x, y)) == b'a' {
                    Some(d)
                } else {
                    None
                }
            })
        })
        .unwrap()
            - 1
    }
}

impl<'a> Day12<'a> {
    fn find_distance<E>(&self, start: (usize, usize), is_end: E, is_reachable: fn(u8, u8) -> bool) -> Vec<Vec<isize>>
    where
        E: Fn((usize, usize), u8) -> bool,
    {
        let width = self.heightmap[0].len();
        let height = self.heightmap.len();
        let mut current = start;
        let mut current_height = self.height(current);
        let mut distances = vec![vec![isize::MAX; width]; height];
        distances[current.1][current.0] = 0;
        let mut to_visit = VecDeque::from([current]);

        while !to_visit.is_empty() && !is_end(current, current_height) {
            current = to_visit.pop_front().unwrap();

            let distance = distances[current.1][current.0] + 1;
            current_height = self.height(current);

            for (x, y) in [(-1isize, 0), (1isize, 0), (0, -1isize), (0, 1isize)]
                .into_iter()
                .map(|(dx, dy)| (current.0 as isize + dx, current.1 as isize + dy))
                .filter(|&(x, y)| x >= 0 && y >= 0 && x < width as isize && y < height as isize)
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|&(x, y)| is_reachable(current_height, self.height((x, y))))
            {
                if distances[y][x] > distance {
                    distances[y][x] = distance;
                    to_visit.push_back((x, y));
                }
            }
        }

        distances
    }
}
