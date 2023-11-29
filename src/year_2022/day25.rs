use crate::day::Day;
use std::collections::VecDeque;

pub struct Day25<'a> {
    numbers: Vec<&'a str>,
}

fn parse(snafu: &str) -> usize {
    let len = snafu.len() - 1;
    snafu.bytes().enumerate().fold(0, |r, (i, b)| {
        r + 5usize.pow((len - i) as u32) as isize
            * match b {
                b'0'..=b'2' => (b - b'0') as isize,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!(),
            }
    }) as usize
}

fn format(number: usize) -> String {
    let mut n = number as isize;
    let mut r = VecDeque::new();

    while n >= 5 {
        r.push_front(n % 5);
        n /= 5;
    }

    r.push_front(n);

    let l = r.len();
    for i in (0..l).rev() {
        if r[i] > 2 {
            r[i] -= 5;
            if i > 0 {
                r[i - 1] += 1;
            } else {
                r.push_front(1);
            }
        }
    }

    r.into_iter()
        .map(|c| match c {
            -2 => '=',
            -1 => '-',
            0..=2 => (b'0' + c as u8) as char,
            _ => unreachable!(),
        })
        .collect()
}

impl<'a> Day<'a> for Day25<'a> {
    type T1 = String;
    type T2 = &'static str;

    fn new(input: &'a str) -> Self {
        Self {
            numbers: input.lines().filter(|l| !l.is_empty()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        format(self.numbers.iter().copied().map(parse).sum())
    }

    fn part_2(&self) -> Self::T2 {
        "Will it blend ?"
    }
}
