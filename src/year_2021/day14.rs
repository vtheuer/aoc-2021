use ahash::AHashMap;

use crate::day::Day;

pub struct Day14 {
    start: AHashMap<(u8, u8), usize>,
    pairs: AHashMap<(u8, u8), u8>,
}

impl Day14 {
    fn solve(&self, steps: usize) -> usize {
        let atom_counts = (0..steps)
            .fold(self.start.clone(), |counts, _| {
                counts.iter().fold(AHashMap::default(), |mut nc, (&(a, b), &n)| {
                    let c = self.pairs[&(a, b)];
                    *nc.entry((a, c)).or_insert(0) += n;
                    *nc.entry((c, b)).or_insert(0) += n;
                    nc
                })
            })
            .into_iter()
            .fold(AHashMap::default(), |mut atom_counts, ((a, b), n)| {
                *atom_counts.entry(a).or_insert(0) += n;
                *atom_counts.entry(b).or_insert(0) += n;
                atom_counts
            });

        (atom_counts.values().max().unwrap() - atom_counts.values().min().unwrap()) / 2 + 1
    }
}

impl Day<'_> for Day14 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (start, pairs) = input.split_once("\n\n").unwrap();
        Self {
            start: start.as_bytes().windows(2).fold(AHashMap::default(), |mut start, w| {
                *start.entry((w[0], w[1])).or_insert(0) += 1;
                start
            }),
            pairs: pairs
                .lines()
                .map(|l| {
                    let mut bytes = l.bytes().filter(|&b| (b as char).is_uppercase());
                    Some(((bytes.next()?, bytes.next()?), bytes.next()?))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.solve(10)
    }

    fn part_2(&self) -> Self::T2 {
        self.solve(40)
    }
}
