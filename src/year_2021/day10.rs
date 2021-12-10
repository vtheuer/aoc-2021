use std::collections::VecDeque;

use crate::day::Day;

pub struct Day10<'a> {
    input: &'a str,
}

fn check(line: &str) -> Result<VecDeque<u8>, u8> {
    line.bytes().fold(Ok(VecDeque::new()), |r, c| {
        r.and_then(|mut stack| {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push_back(c),
                _ => {
                    if stack.pop_back()
                        != Some(match c {
                            b')' => b'(',
                            b']' => b'[',
                            b'}' => b'{',
                            b'>' => b'<',
                            _ => unreachable!(c),
                        })
                    {
                        return Err(c);
                    }
                }
            };
            Ok(stack)
        })
    })
}

impl<'a> Day<'a> for Day10<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn part_1(&self) -> Self::T1 {
        self.input
            .lines()
            .filter_map(|l| check(l).err())
            .map(|c| match c {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                _ => unreachable!(c),
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        let mut scores = self
            .input
            .lines()
            .filter_map(|l| check(l).ok())
            .filter(|stack| !stack.is_empty())
            .map(|stack| {
                stack
                    .into_iter()
                    .rev()
                    .map(|c| match c {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => unreachable!(c as char),
                    })
                    .fold(0, |score, s| score * 5 + s)
            })
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}
