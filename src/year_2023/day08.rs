use fnv::FnvHashMap;
use num::Integer;

use crate::day::Day;
use crate::year_2023::day08::Direction::{Left, Right};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

pub struct Day08<'a> {
    directions: Vec<Direction>,
    nodes: FnvHashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Day08<'a> {
    fn navigate(&self, start: &'a str, is_end: fn(&'a str) -> bool) -> usize {
        let mut directions = self.directions.iter().copied().cycle();
        let mut current = start;
        let mut steps = 0;

        while !is_end(current) {
            let (left, right) = self.nodes[current];
            current = if directions.next().unwrap() == Left {
                left
            } else {
                right
            };
            steps += 1;
        }

        steps
    }
}

impl<'a> Day<'a> for Day08<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let (directions, nodes) = input.split_once("\n\n").unwrap();
        Self {
            directions: directions
                .bytes()
                .map(|d| if d == b'L' { Left } else { Right })
                .collect(),
            nodes: FnvHashMap::from_iter(nodes.lines().map(|l| {
                let (name, nexts) = l.split_once(" = ").unwrap();
                (name, nexts[1..9].split_once(", ").unwrap())
            })),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.navigate("AAA", |node| node == "ZZZ")
    }

    fn part_2(&self) -> Self::T2 {
        self.nodes
            .keys()
            .filter(|&&name| name.ends_with('A'))
            .map(|&start| self.navigate(start, |node| node.ends_with('Z')))
            .reduce(|r, s| r.lcm(&s))
            .unwrap()
    }
}
