use ahash::AHashSet;

use crate::day::Day;
use crate::util::Joinable;

pub struct Day09 {
    moves: Vec<((isize, isize), u8)>,
}

fn should_move_node((hx, hy): (isize, isize), (tx, ty): (isize, isize)) -> bool {
    hx.abs_diff(tx) > 1 || hy.abs_diff(ty) > 1
}

fn move_node((hx, hy): (isize, isize), (tx, ty): (isize, isize)) -> (isize, isize) {
    (tx + (hx - tx).signum(), ty + (hy - ty).signum())
}

fn print(seen: &AHashSet<(isize, isize)>, h: (isize, isize), t: (isize, isize)) {
    let min_x = seen.iter().map(|&(x, _)| x).min().unwrap().min(h.0).min(t.0);
    let max_x = seen.iter().map(|&(x, _)| x).max().unwrap().max(h.0).max(t.0);
    let min_y = seen.iter().map(|&(_, y)| y).min().unwrap().min(h.1).min(t.1);
    let max_y = seen.iter().map(|&(_, y)| y).max().unwrap().max(h.1).max(t.1);
    println!(
        "{}\n",
        (min_y..=max_y)
            .map(|y| (min_x..=max_x)
                .map(|x| if (x, y) == h {
                    'H'
                } else if (x, y) == t {
                    'T'
                } else if seen.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>())
            .join("\n")
    );
}

impl Day09 {
    fn solve(&self, len: usize) -> usize {
        self.moves
            .iter()
            .fold(
                (vec![(0, 0); len], AHashSet::from_iter([(0, 0)].into_iter())),
                |(mut nodes, mut seen), &((dx, dy), n)| {
                    for _ in 0..n {
                        let head = &mut nodes[0];
                        head.0 += dx;
                        head.1 += dy;
                        let mut p = *head;
                        for (i, t) in nodes.iter_mut().enumerate().skip(1) {
                            if should_move_node(p, *t) {
                                *t = move_node(p, *t);
                                if i == len - 1 {
                                    seen.insert(*t);
                                }
                            }
                            p = *t;
                        }
                    }
                    (nodes, seen)
                },
            )
            .1
            .len()
    }
}

impl Day<'_> for Day09 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            moves: input
                .lines()
                .map(|l| {
                    (
                        match l.as_bytes()[0] {
                            b'U' => (0, -1),
                            b'D' => (0, 1),
                            b'L' => (-1, 0),
                            b'R' => (1, 0),
                            _ => unreachable!(),
                        },
                        l[2..].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.solve(2)
    }

    fn part_2(&self) -> Self::T2 {
        self.solve(10)
    }
}
