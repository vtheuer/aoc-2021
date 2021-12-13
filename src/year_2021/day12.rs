use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

use fnv::{FnvHashMap, FnvHashSet};

use crate::day::Day;
use crate::util::{split_pair, Joinable};

pub struct Day12<'a> {
    graph: FnvHashMap<&'a str, FnvHashSet<&'a str>>,
}

impl<'a> Day12<'a> {
    fn paths2(&self, cave: &'a str, visited: &FnvHashSet<&'a str>, path: &[&'a str]) -> usize {
        if cave == "end" {
            // println!("{},end", path.iter().join(","));
            1
        } else {
            let new_visited = visited
                .union(&if cave.chars().next().unwrap().is_uppercase() {
                    FnvHashSet::default()
                } else {
                    FnvHashSet::from_iter([cave].into_iter())
                })
                .copied()
                .collect::<FnvHashSet<_>>();

            let new_path = path.iter().chain(vec![cave].iter()).copied().collect::<Vec<_>>();
            self.graph[cave]
                .iter()
                .filter(|&&next| !visited.contains(next))
                .collect::<Vec<_>>()
                .into_iter()
                .map(|&next| self.paths2(next, &new_visited, &new_path))
                .sum()
        }
    }

    fn paths(&self) -> usize {
        let mut queue = VecDeque::from([("start", FnvHashSet::default())]);
        let mut path_count = 0;

        while !queue.is_empty() {
            let (cave, visited) = queue.pop_front().unwrap();
            if cave == "end" {
                path_count += 1;
            } else {
                let new_visited = if cave.chars().next().unwrap().is_uppercase() {
                    visited
                } else {
                    visited
                        .union(&FnvHashSet::from_iter([cave].into_iter()))
                        .copied()
                        .collect::<FnvHashSet<_>>()
                };
                self.graph[cave]
                    .iter()
                    .filter(|&&next| !new_visited.contains(next))
                    .for_each(|&next| queue.push_back((next, new_visited.clone())));
            }
        }

        path_count
    }
}

impl<'a> Day<'a> for Day12<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            graph: input.lines().map(|l| split_pair(l, "-").unwrap()).fold(
                FnvHashMap::default(),
                |mut graph, (f, t)| {
                    graph.entry(f).or_insert_with(FnvHashSet::default).insert(t);
                    graph.entry(t).or_insert_with(FnvHashSet::default).insert(f);
                    graph
                },
            ),
        }
    }

    fn part_1(&self) -> Self::T1 {
        // self.paths2("start", &FnvHashSet::default(), &vec![])
        self.paths()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
