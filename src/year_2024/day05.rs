use crate::day::Day;
use ahash::{AHashMap, AHashSet};
use std::cell::Cell;
use std::cmp::Ordering;
use Ordering::*;

pub struct Day05 {
    pairs: AHashMap<usize, AHashSet<usize>>,
    updates: Vec<Vec<usize>>,
    already_ordered: Cell<Vec<usize>>,
}

impl Day05 {
    fn is_sorted(&self, update: &[usize]) -> bool {
        update.iter().enumerate().all(|(i, &n)| {
            let rs = &self.pairs[&n];
            update[0..i].iter().all(|a| !rs.contains(a))
        })
    }

    fn sort(&self, update: &[usize]) -> Vec<usize> {
        let mut sorted = update.to_vec();

        sorted.sort_unstable_by(|a, b| {
            self.pairs
                .get(a)
                .filter(|rs| rs.contains(b))
                .map(|_| Less)
                .unwrap_or(Equal)
        });

        sorted
    }
}

impl Day<'_> for Day05 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (pairs, updates) = input.split_once("\n\n").unwrap();
        Self {
            pairs: pairs
                .lines()
                .filter_map(|l| l.split_once('|'))
                .filter_map(|(l, r)| Some((l.parse().ok()?, r.parse().ok()?)))
                .fold(AHashMap::default(), |mut map, (l, r)| {
                    map.entry(l).or_insert_with(AHashSet::default).insert(r);
                    map
                }),
            updates: updates
                .lines()
                .map(|update| update.split(',').filter_map(|n| n.parse::<usize>().ok()).collect())
                .collect(),
            already_ordered: Cell::new(Vec::new()),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let already_ordered = self
            .updates
            .iter()
            .enumerate()
            .filter(|(_, update)| self.is_sorted(update))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let sum = already_ordered
            .iter()
            .map(|&i| &self.updates[i])
            .map(|update| update[update.len() / 2])
            .sum();

        self.already_ordered.set(already_ordered);

        sum
    }

    fn part_2(&self) -> Self::T2 {
        let already_ordered = self.already_ordered.take();
        self.updates
            .iter()
            .enumerate()
            .filter(|(i, _)| already_ordered.binary_search(i).is_err())
            .map(|(_, update)| self.sort(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }
}
