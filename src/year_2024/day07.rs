use crate::day::Day;
use crate::util::Joinable;

pub struct Day07 {
    equations: Vec<(usize, Vec<usize>)>,
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}
use Operator::*;

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Add => a + b,
            Mul => a * b,
            Concat => a * 10usize.pow(b.checked_ilog10().unwrap_or(0) + 1) + b,
        }
    }
}

fn is_valid(result: usize, parts: &[usize], operators: &[Operator], acc: usize) -> bool {
    if parts.is_empty() {
        return result == acc;
    }

    let part = parts[0];

    for operator in operators {
        if is_valid(result, &parts[1..], operators, operator.apply(acc, part)) {
            return true;
        }
    }

    false
}

impl Day07 {
    fn sum_valid(&self, operators: &[Operator]) -> usize {
        self.equations
            .iter()
            .filter(|(result, parts)| is_valid(*result, parts, operators, 0))
            .map(|(result, _)| result)
            .sum()
    }
}

impl Day<'_> for Day07 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            equations: input
                .lines()
                .map(|l| {
                    let (result, parts) = l.split_once(": ").unwrap();
                    (
                        result.parse().unwrap(),
                        parts.split(' ').map(|p| p.parse().unwrap()).collect(),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.sum_valid(&[Add, Mul])
    }

    fn part_2(&self) -> Self::T2 {
        self.sum_valid(&[Add, Mul, Concat])
    }
}
