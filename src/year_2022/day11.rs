use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::VecDeque;

use Operand::*;
use Operation::*;

use crate::day::Day;
use crate::util::SortableByKey;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Old,
    Value(usize),
}

impl Operand {
    fn parse(s: &str) -> Operand {
        match s {
            "old" => Old,
            value => Value(value.parse().unwrap_or_else(|_| panic!("{}", s))),
        }
    }

    fn get(&self, old: usize) -> usize {
        match self {
            Old => old,
            Value(v) => *v,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn parse(op_line: &str) -> Self {
        match op_line.split(' ').collect::<Vec<_>>().as_slice() {
            &[a, op, b] => {
                let op_a = Operand::parse(a);
                let op_b = Operand::parse(b);
                match op {
                    "+" => Add(op_a, op_b),
                    "*" => Mul(op_a, op_b),
                    _ => unreachable!("{}", op),
                }
            }
            _ => unreachable!("{}", op_line),
        }
    }

    fn apply(&self, old: usize) -> usize {
        match self {
            Add(a, b) => a.get(old) + b.get(old),
            Mul(a, b) => a.get(old) * b.get(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn next(&self, worry: usize) -> usize {
        if worry % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

fn product_of_2_most_active(monkeys: Vec<RefCell<(usize, VecDeque<usize>)>>) -> usize {
    monkeys
        .iter()
        .map(|m| m.borrow().0)
        .sorted_unstable_by_key(|&i| i)
        .rev()
        .take(2)
        .product()
}

impl Day<'_> for Day11 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            monkeys: input
                .split("\n\n")
                .map_while(|s| {
                    let mut lines = s.lines().skip(1);
                    Some(Monkey {
                        items: lines.next()?["  Starting items: ".len()..]
                            .split(", ")
                            .map_while(|i| i.parse().ok())
                            .collect(),
                        operation: Operation::parse(&lines.next()?["  Operation: new = ".len()..]),
                        divisible_by: lines.next()?["  Test: divisible by ".len()..].parse().ok()?,
                        if_true: lines.next()?["    If true: throw to monkey ".len()..].parse().ok()?,
                        if_false: lines.next()?["    If false: throw to monkey ".len()..].parse().ok()?,
                    })
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let monkeys = self
            .monkeys
            .iter()
            .map(|m| RefCell::new((0usize, VecDeque::from(m.items.clone()))))
            .collect::<Vec<_>>();

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let monkey = &self.monkeys[i];

                let mut e = monkeys[i].borrow_mut();
                while !e.1.is_empty() {
                    e.0 += 1;
                    let worry = monkey.operation.apply(e.1.pop_front().unwrap()) / 3;
                    monkeys[monkey.next(worry)].borrow_mut().1.push_back(worry);
                }
            }
        }

        product_of_2_most_active(monkeys)
    }

    fn part_2(&self) -> Self::T2 {
        let mut items = self
            .monkeys
            .iter()
            .flat_map(|m| m.items.iter())
            .map(|&item| {
                self.monkeys
                    .iter()
                    .map(|m| (m.divisible_by, item % m.divisible_by))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (_, monkeys) = self.monkeys.iter().fold((0, vec![]), |(n, mut monkeys), monkey| {
            let item_count = monkey.items.len();
            monkeys.push(RefCell::new((
                0usize,
                (0..item_count).map(|i| n + i).collect::<VecDeque<_>>(),
            )));
            (n + item_count, monkeys)
        });

        for _ in 0..10_000 {
            for (monkey_index, cell) in monkeys.iter().enumerate() {
                let monkey = &self.monkeys[monkey_index];

                let mut e = cell.borrow_mut();
                while !e.1.is_empty() {
                    e.0 += 1;
                    let item_index = e.1.pop_front().unwrap();
                    let remainders = &mut items[item_index];
                    for (divider, remainder) in remainders.iter_mut() {
                        *remainder = monkey.operation.apply(*remainder) % *divider;
                    }
                    let next = if remainders[monkey_index].1 == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };
                    monkeys[next].borrow_mut().1.push_back(item_index);
                }
            }
        }

        product_of_2_most_active(monkeys)
    }
}
