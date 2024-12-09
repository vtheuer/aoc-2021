use crate::day::Day;
use crate::util::Joinable;
use ahash::{AHashMap, AHashSet};
use num::Integer;

pub struct Day08 {
    antenas: AHashMap<u8, Vec<(isize, isize)>>,
    width: isize,
    height: isize,
}

impl Day08 {
    fn count_antinodes(
        &self,
        mut add_antinodes: impl FnMut((isize, isize), (isize, isize), &mut AHashSet<(isize, isize)>),
    ) -> usize {
        self.antenas
            .iter()
            .fold(AHashSet::default(), |mut antinodes, (_, antenas)| {
                for i in 0..antenas.len() - 1 {
                    let a = antenas[i];
                    for &b in &antenas[i + 1..] {
                        add_antinodes(a, b, &mut antinodes)
                    }
                }
                antinodes
            })
            .into_iter()
            .filter(|&(x, y)| x >= 0 && x < self.width && y >= 0 && y < self.height)
            .count()
    }
}

impl Day<'_> for Day08 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            antenas: input
                .lines()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.bytes()
                        .enumerate()
                        .filter(|&(_, c)| c != b'.')
                        .map(move |(x, c)| (c, (x as isize, y as isize)))
                })
                .fold(AHashMap::default(), |mut map, (a, p)| {
                    map.entry(a).or_insert(Vec::new()).push(p);
                    map
                }),
            width: input.lines().next().unwrap().len() as isize,
            height: input.lines().count() as isize,
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_antinodes(|(ax, ay), (bx, by), antinodes| {
            let dx = ax - bx;
            let dy = ay - by;

            antinodes.insert((ax + dx, ay + dy));
            antinodes.insert((bx - dx, by - dy));
        })
    }

    fn part_2(&self) -> Self::T2 {
        self.count_antinodes(|(ax, ay), (bx, by), antinodes| {
            let mut dx = ax - bx;
            let mut dy = ay - by;
            let gcd = dx.gcd(&dy);
            dx /= gcd;
            dy /= gcd;

            let mut nx = ax;
            let mut ny = ay;

            while nx >= 0 && nx < self.width {
                antinodes.insert((nx, ny));
                nx += dx;
                ny += dy;
            }

            nx = ax;
            ny = ay;

            while ny >= 0 && ny < self.height {
                antinodes.insert((nx, ny));
                nx -= dx;
                ny -= dy;
            }
        })
    }
}
