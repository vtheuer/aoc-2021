use std::cell::Cell;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::VecDeque;

use fnv::{FnvHashMap, FnvHashSet};

use crate::day::Day;
use crate::util::{split_pair, Joinable};

pub struct Day12<'a> {
    caves: Vec<&'a str>,
    big_caves: u16,
    edges: Vec<Vec<bool>>,
    part1: Cell<usize>,
}

fn find_index(caves: &[&str], cave: &str) -> usize {
    let mut l = 0;
    let mut r = caves.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        match caves[m].cmp(cave) {
            Less => l = m + 1,
            Equal => return m,
            Greater => r = m - 1,
        }
    }
    unreachable!(cave)
}

fn add(bit_set: u16, bit: usize) -> u16 {
    bit_set | (1 << bit)
}

fn contains(bit_set: u16, bit: usize) -> bool {
    bit_set & (1 << bit) > 0
}

impl<'a> Day12<'a> {
    fn is_big(&self, cave: usize) -> bool {
        contains(self.big_caves, cave)
    }
}

impl<'a> Day<'a> for Day12<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let mut caves = input
            .lines()
            .flat_map(|l| l.split('-'))
            .collect::<FnvHashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        caves.sort_unstable();
        let big_caves = caves
            .iter()
            .enumerate()
            .filter(|(_, &cave)| cave.chars().next().unwrap().is_uppercase())
            .fold(0, |big_caves, (i, _)| add(big_caves, i));
        Self {
            edges: input.lines().map(|l| split_pair(l, "-").unwrap()).fold(
                vec![vec![false; caves.len()]; caves.len()],
                |mut edges, (f, t)| {
                    let fi = find_index(&caves, f);
                    let ti = find_index(&caves, t);
                    edges[fi][ti] = true;
                    edges[ti][fi] = true;
                    edges
                },
            ),
            caves,
            big_caves,
            part1: Cell::new(0),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let end_index = find_index(&self.caves, "end");
        let mut queue = VecDeque::from([(find_index(&self.caves, "start"), 0)]);
        let mut path_count = 0;

        while !queue.is_empty() {
            let (cave, mut visited) = queue.pop_back().unwrap();
            if cave == end_index {
                path_count += 1;
            } else {
                if !self.is_big(cave) {
                    visited = add(visited, cave);
                }
                self.edges[cave]
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(i, has_edge)| has_edge && !contains(visited, i))
                    .for_each(|(i, _)| queue.push_back((i, visited)));
            }
        }

        self.part1.set(path_count);

        path_count
    }

    fn part_2(&self) -> Self::T2 {
        let start_index = find_index(&self.caves, "start");
        let end_index = find_index(&self.caves, "end");

        (0..self.caves.len())
            .filter(|&i| i != start_index && i != end_index && !self.is_big(i))
            .map(|visitable_twice| {
                let mut queue = VecDeque::from([(start_index, 0, false)]);
                let mut path_count = 0;

                while !queue.is_empty() {
                    let (cave, mut visited, mut visited_twice) = queue.pop_back().unwrap();
                    if cave == end_index {
                        path_count += if visited_twice { 1 } else { 0 };
                    } else {
                        if !self.is_big(cave) {
                            if contains(visited, cave) && cave == visitable_twice {
                                visited_twice = true;
                            }
                            visited = add(visited, cave);
                        }
                        self.edges[cave]
                            .iter()
                            .copied()
                            .enumerate()
                            .filter(|&(i, has_edge)| {
                                has_edge && (!contains(visited, i) || i == visitable_twice && !visited_twice)
                            })
                            .for_each(|(i, _)| queue.push_back((i, visited, visited_twice)));
                    }
                }

                path_count
            })
            .sum::<usize>()
            + self.part1.get()
    }
}
