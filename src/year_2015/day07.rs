use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};

use fnv::FnvHashMap;

use crate::day::Day;
use crate::util::split_pair;
use crate::year_2015::day07::Operator::{And, Assign, Lshift, Not, Or, Rshift};
use crate::year_2015::day07::Value::{Literal, Variable};

#[derive(Debug, Copy, Clone)]
enum Value<'a> {
    Literal(usize),
    Variable(&'a str),
}

#[derive(Debug, Copy, Clone)]
enum Operator<'a> {
    Assign(Value<'a>),
    Not(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Lshift(Value<'a>, Value<'a>),
    Rshift(Value<'a>, Value<'a>),
}

struct Wires<'a> {
    instructions: &'a FnvHashMap<&'a str, Operator<'a>>,
    cache: FnvHashMap<&'a str, usize>,
}

impl<'a> Wires<'a> {
    fn eval(&mut self, value: Value<'a>) -> usize {
        match value {
            Literal(l) => l,
            Variable(v) => self.eval_wire(v),
        }
    }

    fn eval_wire(&mut self, wire: &'a str) -> usize {
        let x = self.instructions[wire];
        match self.cache.get(wire) {
            Some(v) => *v,
            None => {
                let v = match x {
                    Assign(a) => self.eval(a),
                    Not(v) => !self.eval(v),
                    And(a, b) => self.eval(a) & self.eval(b),
                    Or(a, b) => self.eval(a) | self.eval(b),
                    Lshift(a, b) => self.eval(a) << self.eval(b),
                    Rshift(a, b) => self.eval(a) >> self.eval(b),
                };
                self.cache.insert(wire, v);
                v
            }
        }
    }
}

pub struct Day07<'a> {
    instructions: FnvHashMap<&'a str, Operator<'a>>,
    a: Cell<usize>,
}

impl<'a> Day<'a> for Day07<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let parse_value: fn(&str) -> Value = |s| s.parse::<usize>().ok().map(Literal).unwrap_or(Variable(s));

        Self {
            instructions: input
                .lines()
                .map(|l| {
                    let (op, dest) = split_pair(l, " -> ").unwrap();
                    let op_parts = op.split(' ').collect::<Vec<_>>();
                    (
                        dest,
                        match op_parts.as_slice() {
                            &[a, o, c] => match o {
                                "AND" => And(parse_value(a), parse_value(c)),
                                "OR" => Or(parse_value(a), parse_value(c)),
                                "LSHIFT" => Lshift(parse_value(a), parse_value(c)),
                                "RSHIFT" => Rshift(parse_value(a), parse_value(c)),
                                _ => unreachable!(op),
                            },
                            [_, v] => Not(parse_value(v)),
                            [a] => Assign(parse_value(a)),
                            _ => unreachable!(op),
                        },
                    )
                })
                .collect(),
            a: Cell::new(0),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let a = Wires {
            instructions: &self.instructions,
            cache: FnvHashMap::default(),
        }
        .eval_wire("a");
        self.a.set(a);
        a
    }

    fn part_2(&self) -> Self::T2 {
        let mut new_instructions = self.instructions.clone();
        new_instructions.insert("b", Assign(Literal(self.a.get())));
        Wires {
            instructions: &new_instructions,
            cache: FnvHashMap::default(),
        }
        .eval_wire("a")
    }
}
