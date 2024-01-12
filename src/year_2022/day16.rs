use std::borrow::Borrow;

use regex::Regex;

use ahash::AHashMap;

use crate::day::Day;

pub struct Day16 {
    rates: Vec<(usize, usize)>,
    distances: Vec<Vec<usize>>,
}

fn distances(from: usize, valves_with_flow: &Vec<(usize, usize)>, connections: &Vec<Vec<usize>>) -> Vec<usize> {
    let (initial_index, _) = valves_with_flow[from];
    let mut distances = vec![usize::MAX; connections.len()];
    distances[initial_index] = 0;
    let mut queue = (0..connections.len()).collect::<Vec<_>>();

    while !queue.is_empty() {
        let index_in_queue = queue
            .iter()
            .enumerate()
            .min_by_key(|&(_, &v)| distances[v])
            .map(|(i, _)| i)
            .unwrap();
        let valve = queue[index_in_queue];
        let distance = distances[valve];
        queue.remove(index_in_queue);

        for &next in &connections[valve] {
            distances[next] = distances[next].min(distance + 1);
        }
    }

    valves_with_flow.iter().map(|&(i, _)| distances[i]).collect()
}

impl Day16 {
    fn part_1(&self, remaining: usize, current: usize, visited: u128, score: usize) -> usize {
        self.distances[current]
            .iter()
            .enumerate()
            .filter(|&(_, &distance)| distance + 1 < remaining)
            .filter(|&(next, _)| visited & (1 << next) == 0)
            .map(|(next, &distance)| {
                let new_remaining = remaining - distance - 1;
                self.part_1(
                    new_remaining,
                    next,
                    visited | (1 << next),
                    score + new_remaining * self.rates[next].1,
                )
            })
            .max()
            .unwrap_or(score)
    }

    fn part_2(
        &self,
        remaining: usize,
        current: usize,
        visited: u128,
        score: usize,
        path: Vec<usize>,
    ) -> (usize, Vec<usize>) {
        self.distances[current]
            .iter()
            .enumerate()
            .filter(|&(_, &distance)| distance + 1 < remaining)
            .filter(|&(next, _)| visited & (1 << next) == 0)
            .map(|(next, &distance)| {
                let new_remaining = remaining - distance - 1;
                self.part_2(
                    new_remaining,
                    next,
                    visited | (1 << next),
                    score + new_remaining * self.rates[next].1,
                    path.iter().copied().chain([next]).collect(),
                )
            })
            .max_by_key(|&(score, _)| score)
            .unwrap_or((score, path))
    }
}

impl Day<'_> for Day16 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let re = Regex::new("^Valve (..) has flow rate=(\\d+); tunnels? leads? to valves? (.+)$").unwrap();
        let valves = AHashMap::from_iter(
            input
                .lines()
                .map_while(|l| {
                    let captures = re.captures(l)?;
                    let mut next = captures.get(3)?.as_str().split(", ").collect::<Vec<_>>();
                    next.sort();
                    Some((
                        captures.get(1)?.as_str(),
                        captures.get(2)?.as_str().parse::<usize>().ok()?,
                        next,
                    ))
                })
                .map(|(valve, rate, next)| (valve, (rate, next))),
        );
        let mut names = valves.keys().copied().collect::<Vec<_>>();
        names.sort();

        let valves_with_flow = names
            .iter()
            .map(|&valve| valves[valve].0)
            .enumerate()
            .filter(|&(i, rate)| i == 0 || rate > 0)
            .collect::<Vec<_>>();

        let connections = names
            .iter()
            .map(|&valve| {
                valves[valve]
                    .1
                    .iter()
                    .map(|v| names.binary_search(v).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            rates: valves_with_flow.clone(),
            distances: (0..valves_with_flow.len())
                .map(|from| distances(from, &valves_with_flow, &connections))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.part_1(30, 0, 0, 0)
    }

    fn part_2(&self) -> Self::T2 {
        let (score, visited) = self.part_2(26, 0, 0, 0, vec![]);
        score
            + self
                .part_2(26, 0, visited.into_iter().fold(0, |v, i| v | (1 << i)), 0, vec![])
                .0
    }
}
