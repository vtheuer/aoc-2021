use std::collections::VecDeque;
use std::convert::identity;

use crate::day::Day;

pub struct Day18 {
    numbers: Vec<Vec<(u8, u8)>>,
}

fn add(a: &[(u8, u8)], b: &[(u8, u8)]) -> Vec<(u8, u8)> {
    reduce(a.iter().chain(b.iter()).copied().map(|(v, d)| (v, d + 1)).collect())
}

fn reduce(mut number: Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut reduced = true;
    while reduced {
        reduced = if let Some((i, &(value, _))) = number.iter().enumerate().find(|(_, &(_, depth))| depth == 4) {
            number.remove(i);
            if i > 0 {
                number[i - 1].0 += value;
            }
            if i < number.len() - 1 {
                number[i + 1].0 += number[i].0;
            }
            number[i] = (0, 3);
            true
        } else if let Some((i, &(value, d))) = number.iter().enumerate().find(|(_, &(value, _))| value >= 10) {
            let left = ((value as f32 / 2f32).floor() as u8, d + 1);
            let right = ((value as f32 / 2f32).ceil() as u8, d + 1);
            number[i] = left;
            number.insert(i + 1, right);
            true
        } else {
            false
        };
    }
    number
}

fn magnitude(number: &[(u8, u8)]) -> usize {
    let mut m = number.iter().map(|&(n, d)| (n as usize, d + 1)).collect::<Vec<_>>();
    let mut len = m.len();
    let mut max_depth = m[..len].iter().map(|&(_, d)| d).max().unwrap();

    while len > 1 {
        let mut i = 0;
        while i < len {
            if m[i].1 == max_depth && i < len - 1 && m[i].1 == m[i + 1].1 {
                m[i] = (3 * m[i].0 + 2 * m[i + 1].0, m[i].1 - 1);
                len -= 1;
                for j in i + 1..len {
                    m[j] = m[j + 1];
                }
            }
            i += 1;
        }
        max_depth -= 1;
    }

    m[0].0
}

impl Day<'_> for Day18 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            numbers: input
                .lines()
                .map(|l| {
                    l.bytes()
                        .take(l.len() - 1)
                        .skip(1)
                        .scan(0, |depth, c| {
                            Some(match c {
                                b'[' => {
                                    *depth += 1;
                                    None
                                }
                                b']' => {
                                    *depth -= 1;
                                    None
                                }
                                b',' => None,
                                n => Some((n - b'0', *depth)),
                            })
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .map(reduce)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut number = self.numbers[0].clone();
        for other in self.numbers.iter().skip(1) {
            number = add(&number, other);
        }
        magnitude(&number)
    }

    fn part_2(&self) -> Self::T2 {
        self.numbers
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                self.numbers
                    .iter()
                    .enumerate()
                    .filter(move |&(j, _)| i != j)
                    .map(move |(_, b)| magnitude(&add(a, b)))
            })
            .max()
            .unwrap()
    }
}
