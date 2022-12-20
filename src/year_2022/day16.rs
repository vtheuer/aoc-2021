use crate::day::Day;
use fnv::FnvHashMap;
use regex::Regex;
use std::borrow::Borrow;
use std::cell::RefCell;

pub struct Day16<'a> {
    valves: FnvHashMap<&'a str, (usize, Vec<(&'a str, usize)>)>,
}

impl<'a> Day<'a> for Day16<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let re = Regex::new("^Valve (..) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$").unwrap();
        let mut valves = FnvHashMap::from_iter(
            input
                .lines()
                .map_while(|l| {
                    let captures = re.captures(l)?;
                    let mut next = captures
                        .get(3)?
                        .as_str()
                        .split(", ")
                        .map(|n| (n, 1))
                        .collect::<Vec<_>>();
                    next.sort();
                    Some((
                        captures.get(1)?.as_str(),
                        captures.get(2)?.as_str().parse::<usize>().ok()?,
                        next,
                    ))
                })
                .map(|(valve, rate, next)| (valve, (rate, RefCell::new(next)))),
        );

        while let Some(valve) = valves
            .iter()
            .find(|&(&valve, c)| "AA" != valve && c.borrow().0 == 0)
            .map(|(&valve, _)| valve)
        {
            let next = valves[valve].1.borrow().iter().copied().collect::<Vec<_>>();
            let (left, cost_to_left) = next[0];
            let (right, cost_to_right) = next[1];
            valves.remove(valve);
            valves.entry(left).and_modify(|n| {
                let vec = n.1.borrow();
                let mut vec_mut = n.1.borrow_mut();
                vec_mut.remove(vec.binary_search_by_key(&valve, |(v, _)| *v).unwrap());
                vec_mut.insert(
                    vec.binary_search_by_key(&right, |(v, _)| *v).unwrap_err(),
                    (right, cost_to_right + 1),
                );
            });
            valves.entry(right).and_modify(|n| {
                let vec = n.1.borrow();
                let mut vec_mut = n.1.borrow_mut();
                vec_mut.remove(vec.binary_search_by_key(&valve, |(v, _)| *v).unwrap());
                vec_mut.insert(
                    vec.binary_search_by_key(&left, |(v, _)| *v).unwrap_err(),
                    (left, cost_to_left + 1),
                );
            });
        }

        Self {
            valves: FnvHashMap::from_iter(valves.into_iter().map(|(k, (r, n))| (k, (r, n.take())))),
        }
    }

    fn part_1(&self) -> Self::T1 {
        0
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
