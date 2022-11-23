use std::cmp::min;

use fnv::{FnvHashMap, FnvHashSet};

use crate::day::Day;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
    on: bool,
}

fn intersection((fa, ta): (isize, isize), (fb, tb): (isize, isize)) -> Option<(isize, isize)> {
    if fa > fb {
        intersection((fb, tb), (fa, ta))
    } else if fb <= ta {
        Some((fb, min(ta, tb)))
    } else {
        None
    }
}

impl Cuboid {
    fn intersection(&self, other: &Self) -> Option<Self> {
        Some(Self {
            x: intersection(self.x, other.x)?,
            y: intersection(self.y, other.y)?,
            z: intersection(self.z, other.z)?,
            on: !self.on,
        })
    }

    fn size(&self) -> isize {
        let &Self {
            x: (fx, tx),
            y: (fy, ty),
            z: (fz, tz),
            on,
        } = self;
        (if on { 1 } else { -1 }) * ((tx - fx + 1) * (ty - fy + 1) * (tz - fz + 1))
    }
}

pub struct Day22 {
    cuboids: Vec<Cuboid>,
}

fn intersect(cuboids: &[Cuboid]) -> usize {
    cuboids
        .iter()
        .fold(vec![], |mut r: Vec<Cuboid>, &cuboid| {
            for i in 0..r.len() {
                if let Some(intersection) = r[i].intersection(&cuboid) {
                    r.push(intersection);
                }
            }
            if cuboid.on {
                r.push(cuboid);
            }
            r
        })
        .into_iter()
        .map(|cuboid| cuboid.size())
        .sum::<isize>() as usize
}

impl Day<'_> for Day22 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            cuboids: input
                .lines()
                .map(|l| {
                    let (state, rest) = l.split_once(' ')?;
                    let mut values = rest
                        .split(',')
                        .flat_map(|pair| pair[2..].split(".."))
                        .map(str::parse)
                        .map(Result::unwrap);
                    Some(Cuboid {
                        x: (values.next()?, values.next()?),
                        y: (values.next()?, values.next()?),
                        z: (values.next()?, values.next()?),
                        on: state == "on",
                    })
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        intersect(
            &self
                .cuboids
                .iter()
                .filter(
                    |&&Cuboid {
                         x: (fx, tx),
                         y: (fy, ty),
                         z: (fz, tz),
                         on: _,
                     }| { [fx, tx, fy, ty, fz, tz].into_iter().all(|v| (-50..=50).contains(&v)) },
                )
                .copied()
                .collect::<Vec<_>>(),
        )
    }

    fn part_2(&self) -> Self::T2 {
        intersect(&self.cuboids)
    }
}
