use crate::day::Day;

#[derive(Debug, PartialEq)]
enum Token {
    Number(usize),
    Open,
    Close,
    Plus,
    Times,
}

use std::ops::{Add, Mul};
use Token::*;

pub struct Day18 {
    problems: Vec<Vec<Token>>,
}

fn apply_op(op_stack: &mut Vec<&Token>, num_stack: &mut Vec<usize>) {
    let r = match op_stack.pop().unwrap() {
        Plus => usize::add,
        Times => usize::mul,
        _ => unreachable!(),
    }(num_stack.pop().unwrap(), num_stack.pop().unwrap());
    num_stack.push(r);
}

fn eval(tokens: &[Token], precedence: bool) -> usize {
    let mut op_stack = vec![];
    let mut num_stack = vec![];

    for token in tokens {
        match token {
            &Number(n) => num_stack.push(n),
            &Open => op_stack.push(&Open),
            &Close => {
                while !op_stack.is_empty() && op_stack.last() != Some(&&Open) {
                    apply_op(&mut op_stack, &mut num_stack);
                }
                op_stack.pop();
            }
            op => {
                while op_stack
                    .last()
                    .map(|top| top != &&Open && (!precedence || op != &Plus || top != &&Times))
                    .unwrap_or(false)
                {
                    apply_op(&mut op_stack, &mut num_stack);
                }
                op_stack.push(op);
            }
        }
    }

    while !op_stack.is_empty() {
        apply_op(&mut op_stack, &mut num_stack);
    }

    num_stack[0]
}

impl Day<'_> for Day18 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day18 {
            problems: input
                .lines()
                .map(|l| {
                    l.bytes()
                        .filter_map(|c| match c {
                            b'0'..=b'9' => Some(Number((c - b'0') as usize)),
                            b'(' => Some(Open),
                            b')' => Some(Close),
                            b'+' => Some(Plus),
                            b'*' => Some(Times),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.problems.iter().map(|problem| eval(problem, false)).sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        self.problems.iter().map(|problem| eval(problem, true)).sum::<usize>()
    }
}
