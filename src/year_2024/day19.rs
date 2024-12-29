use crate::day::Day;
use ahash::AHashMap;
use colored::Colorize;
use std::collections::VecDeque;

pub struct Day19<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> Day19<'a> {
    fn is_possible(&self, design: &str) -> bool {
        if design.is_empty() {
            true
        } else {
            self.patterns
                .iter()
                .any(|&pattern| design.ends_with(pattern) && self.is_possible(&design[..design.len() - pattern.len()]))
        }
    }

    fn count_possible(&self, design: &'a str, cache: &mut AHashMap<&'a str, usize>) -> usize {
        if cache.contains_key(design) {
            cache[design]
        } else {
            let count = if design.is_empty() {
                1
            } else {
                self.patterns
                    .iter()
                    .filter(|&&pattern| design.ends_with(pattern))
                    .map(|&pattern| self.count_possible(&design[..design.len() - pattern.len()], cache))
                    .sum()
            };
            cache.insert(design, count);
            count
        }
    }
}

impl<'a> Day<'a> for Day19<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let (patterns, designs) = input.split_once("\n\n").unwrap();
        Self {
            patterns: patterns.split(", ").collect(),
            designs: designs.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.designs.iter().filter(|&&design| self.is_possible(design)).count()
    }

    fn part_2(&self) -> Self::T2 {
        let mut cache = AHashMap::new();
        self.designs
            .iter()
            .map(|&design| self.count_possible(design, &mut cache))
            .sum()
    }
}
