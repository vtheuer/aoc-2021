use std::collections::HashSet;

use ahash::{AHashMap, AHashSet};
use Monkey::*;
use Operator::*;

use crate::day::Day;
use crate::year_2022::day21::Monkey::{Operation, Value};

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operator {
    fn parse(c: char) -> Self {
        match c {
            '+' => Plus,
            '-' => Minus,
            '*' => Times,
            '/' => Divide,
            _ => unreachable!("{}", c),
        }
    }

    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Plus => a + b,
            Minus => a - b,
            Times => a * b,
            Divide => a / b,
        }
    }

    fn invert(&self, result: isize, other: isize, next_is_left: bool) -> isize {
        match self {
            Plus => result - other,
            Minus => {
                if next_is_left {
                    result + other
                } else {
                    -result + other
                }
            }
            Times => result / other,
            Divide => {
                if next_is_left {
                    result * other
                } else {
                    other / result
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Monkey<'a> {
    Value(isize),
    Operation(&'a str, Operator, &'a str),
}

impl<'a> Monkey<'a> {
    fn compute(&self, monkeys: &AHashMap<&'a str, Monkey<'a>>) -> isize {
        match self {
            Value(n) => *n,
            Operation(a, op, b) => op.apply(monkeys[a].compute(monkeys), monkeys[b].compute(monkeys)),
        }
    }
}

pub struct Day21<'a> {
    monkeys: AHashMap<&'a str, Monkey<'a>>,
}

impl<'a> Day21<'a> {
    fn compute(&self, m: &'a str) -> isize {
        self.monkeys[m].compute(&self.monkeys)
    }

    fn humn_ancestors(&self, first: &'a str, second: &'a str) -> AHashSet<&str> {
        let mut parents = AHashMap::default();
        let mut queue = vec![first];
        let mut found = false;
        while let Some(parent) = queue.pop() {
            if let Operation(l, _, r) = self.monkeys[parent] {
                parents.insert(l, parent);
                if l == HUMN {
                    found = true;
                } else {
                    queue.push(l)
                }
                parents.insert(r, parent);
                if r == HUMN {
                    found = true;
                } else {
                    queue.push(r)
                }
            }
        }

        if found {
            let mut p = HUMN;
            let mut humn_ancestors = AHashSet::default();
            while let Some(&parent) = parents.get(p) {
                humn_ancestors.insert(parent);
                p = parent;
            }
            humn_ancestors.insert(HUMN);
            humn_ancestors
        } else {
            self.humn_ancestors(second, first)
        }
    }
}

const HUMN: &str = "humn";

impl<'a> Day<'a> for Day21<'a> {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &'a str) -> Self {
        Self {
            monkeys: AHashMap::from_iter(input.lines().map_while(|l| {
                let (k, v) = l.split_once(": ")?;
                Some((
                    k,
                    if v.as_bytes()[0].is_ascii_digit() {
                        Value(v.parse().ok()?)
                    } else {
                        let mut parts = v.split(' ');
                        Operation(
                            parts.next()?,
                            Operator::parse(parts.next()?.chars().next()?),
                            parts.next()?,
                        )
                    },
                ))
            })),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.compute("root")
    }

    fn part_2(&self) -> Self::T2 {
        let (root_left, root_right) = match self.monkeys["root"] {
            Operation(l, _, r) => (l, r),
            _ => unreachable!(),
        };

        let humn_ancestors = self.humn_ancestors(root_left, root_right);

        let mut result = self.compute(root_right);
        let mut m = root_left;
        while HUMN != m {
            if let Operation(l, op, r) = self.monkeys[m] {
                let (next, other) = if humn_ancestors.contains(l) { (l, r) } else { (r, l) };
                result = op.invert(result, self.compute(other), next == l);
                m = next;
            } else {
                unreachable!("{} is not an operation", m);
            }
        }

        result
    }
}
