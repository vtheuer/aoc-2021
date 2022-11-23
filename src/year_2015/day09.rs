use std::cmp::min;

use fnv::FnvHashSet;

use crate::day::Day;
use crate::util::split_pair;

pub struct Day09 {
    distances: Vec<(String, String, usize)>,
    edges: Vec<Vec<usize>>,
}

impl Day09 {
    fn get_edge(&self, from: usize, to: usize) -> usize {
        let distance = if from > to {
            self.get_edge(to, from)
        } else {
            self.edges[from][to - from]
        };
        dbg!((from, to, distance));
        distance
    }
}

impl Day09 {
    fn new2(input: &str) -> Self {
        Self {
            distances: vec![],
            edges: input
                .lines()
                .map(|l| {
                    let (places, distance) = split_pair(l, " = ").unwrap();
                    let (from, _) = split_pair(places, " to ").unwrap();
                    (from, distance.parse::<usize>().unwrap())
                })
                .fold(
                    (vec![], None),
                    |(mut edges, prev): (Vec<Vec<usize>>, Option<&str>), (from, distance)| {
                        if prev == Some(from) {
                            let last = edges.len() - 1;
                            edges[last].push(distance)
                        } else {
                            edges.push(vec![distance])
                        }
                        (edges, Some(from))
                    },
                )
                .0,
        }
    }
}

impl Day<'_> for Day09 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            distances: input
                .lines()
                .map(|l| {
                    let (places, distance) = split_pair(l, " = ").unwrap();
                    let (from, to) = split_pair(places, " to ").unwrap();
                    (from.to_string(), to.to_string(), distance.parse::<usize>().unwrap())
                })
                .collect(),
            edges: vec![],
        }
    }

    fn part_1(&self) -> Self::T1 {
        (0..self.distances.len())
            .map(|i| {
                self.distances
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| i != j)
                    .map(|(_, (_, _, d))| d)
                    .sum::<usize>()
            })
            .min()
            .unwrap()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
