use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;

use ahash::AHashMap;

use ModuleState::*;
use Type::*;

use crate::day::Day;
use crate::util::Joinable;

#[derive(Copy, Clone, Debug)]
enum Type {
    FlipFlop,
    Conjunction,
}

enum ModuleState<'a> {
    FlipFlopState(bool),
    ConjunctionState(AHashMap<&'a str, bool>),
    UntypedState(bool),
}

impl<'a> ModuleState<'a> {
    fn receive(&mut self, pulse: bool, from: &'a str) -> Option<bool> {
        match self {
            FlipFlopState(b) => {
                if !pulse {
                    *b = !*b;
                    Some(*b)
                } else {
                    None
                }
            }
            ConjunctionState(remembered) => {
                remembered.insert(from, pulse);
                Some(!remembered.values().all(|&pulse| pulse))
            }
            _ => None,
        }
    }

    fn is_initial(&self) -> bool {
        match self {
            FlipFlopState(b) | UntypedState(b) => !b,
            ConjunctionState(remembered) => remembered.values().all(|&pulse| !pulse),
        }
    }

    fn format(&self) -> String {
        match self {
            FlipFlopState(b) | UntypedState(b) => (if *b { "high" } else { "low" }).to_string(),
            ConjunctionState(remembered) => remembered
                .iter()
                .map(|(name, high)| format!("{}: {}", name, if *high { "high" } else { "low" }))
                .join(", "),
        }
    }
}

#[derive(Clone, Debug)]
struct Module<'a> {
    name: &'a str,
    ty: Type,
    destination_modules: Vec<&'a str>,
}

pub struct Day20<'a> {
    modules: AHashMap<&'a str, Module<'a>>,
    broadcast_modules: Vec<&'a str>,
}

impl<'a> Day20<'a> {
    fn process(&'a self, states: &'a AHashMap<&'a str, RefCell<ModuleState<'a>>>) -> (usize, usize) {
        let mut low = 1 + self.broadcast_modules.len();
        let mut high = 0;
        let mut queue: VecDeque<(bool, &str, &str)> = VecDeque::new();
        for &broadcast_module in &self.broadcast_modules {
            queue.push_back((false, "broadcaster", broadcast_module));
        }

        while let Some((pulse, from, to)) = queue.pop_front() {
            if !self.modules.contains_key(to) {
                if to == "rx" && !pulse {
                    println!("rx received a low pulse");
                }
                continue;
            }

            let module = &self.modules[to];
            if let Some(pulse_to_send) = (&states[to]).borrow_mut().receive(pulse, from) {
                for &destination in &module.destination_modules {
                    *(if pulse_to_send { &mut high } else { &mut low }) += 1;
                    queue.push_back((pulse_to_send, module.name, destination));
                }
            }
        }

        (low, high)
    }
}

impl<'a> Day<'a> for Day20<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let mut broadcast_modules = vec![];
        let modules = input
            .lines()
            .filter_map(|l| {
                let (type_and_name, destination) = l.split_once(" -> ")?;
                let destination_modules = destination.split(", ").collect();

                if type_and_name == "broadcaster" {
                    broadcast_modules = destination_modules;
                    None
                } else {
                    let name = &type_and_name[1..];
                    Some((
                        name,
                        Module {
                            name,
                            ty: if type_and_name.as_bytes()[0] == b'%' {
                                FlipFlop
                            } else {
                                Conjunction
                            },
                            destination_modules,
                        },
                    ))
                }
            })
            .collect();
        Self {
            broadcast_modules,
            modules,
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut states = self
            .modules
            .iter()
            .map(|(&name, module)| {
                (
                    name,
                    RefCell::new(match module.ty {
                        FlipFlop => FlipFlopState(false),
                        Conjunction => ConjunctionState(
                            self.modules
                                .values()
                                .filter(|m| m.destination_modules.contains(&name))
                                .map(|m| (m.name, false))
                                .collect(),
                        ),
                    }),
                )
            })
            .collect::<AHashMap<_, _>>();

        for &name in self.modules.values().flat_map(|m| m.destination_modules.iter()) {
            states.entry(name).or_insert(RefCell::new(UntypedState(false)));
        }

        let mut low = 0;
        let mut high = 0;
        let mut remaining = 1000;
        while remaining > 0 {
            let (l, h) = self.process(&states);
            low += l;
            high += h;
            remaining -= 1;
            if states.values().all(|s| s.borrow().is_initial()) {
                let cycle_length = 1000 - remaining;
                low *= remaining / cycle_length + 1;
                high *= remaining / cycle_length + 1;
                remaining %= cycle_length;
            }
        }

        low * high
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
