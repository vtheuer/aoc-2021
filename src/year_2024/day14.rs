use crate::day::Day;
use crate::util::grid::Grid;
use crate::util::Joinable;
use std::thread::sleep;
use std::time::Duration;

pub struct Day14 {
    robots: Vec<((isize, isize), (isize, isize))>,
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

impl Day<'_> for Day14 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            robots: input
                .lines()
                .filter_map(|l| {
                    let (p, v) = l.split_once(' ')?;
                    let (px, py) = p[2..].split_once(',')?;
                    let (vx, vy) = v[2..].split_once(',')?;

                    Some((
                        (px.parse().ok()?, py.parse().ok()?),
                        (vx.parse().ok()?, vy.parse().ok()?),
                    ))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (tl, tr, bl, br) = self
            .robots
            .iter()
            .map(|&((px, py), (vx, vy))| {
                let x = (px + 100 * vx) % WIDTH;
                let y = (py + 100 * vy) % HEIGHT;
                (if x >= 0 { x } else { WIDTH + x }, if y >= 0 { y } else { HEIGHT + y })
            })
            .filter(|&(x, y)| x != WIDTH / 2 && y != HEIGHT / 2)
            .fold((0, 0, 0, 0), |(tl, tr, bl, br), (x, y)| {
                if x < WIDTH / 2 {
                    if y < HEIGHT / 2 {
                        (tl + 1, tr, bl, br)
                    } else {
                        (tl, tr, bl + 1, br)
                    }
                } else {
                    if y < HEIGHT / 2 {
                        (tl, tr + 1, bl, br)
                    } else {
                        (tl, tr, bl, br + 1)
                    }
                }
            });

        tl * tr * bl * br
    }

    fn part_2(&self) -> Self::T2 {
        let mut robots = self.robots.iter().map(|&(p, _)| p).collect::<Vec<_>>();
        for s in 1.. {
            let mut grid = Grid::init(WIDTH as usize, HEIGHT as usize, false);
            for (i, &(_, (vx, vy))) in self.robots.iter().enumerate() {
                let x = (robots[i].0 + vx) % WIDTH;
                let y = (robots[i].1 + vy) % HEIGHT;
                robots[i] = (if x >= 0 { x } else { WIDTH + x }, if y >= 0 { y } else { HEIGHT + y });
                grid[(robots[i].0 as usize, robots[i].1 as usize)] = true;
            }
            if grid
                .rows()
                .any(|row| row.iter().copied().skip_while(|&c| !c).take_while(|&c| c).count() > 10)
            {
                return s;
            }
        }
        unreachable!()
    }
}
