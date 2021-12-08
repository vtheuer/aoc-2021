use crate::day::Day;
use crate::util::{split_pair, Joinable, SortableByKey};

pub struct Day08 {
    entries: Vec<(Vec<u8>, Vec<u8>)>,
}

fn parse_segments(left: &str) -> Vec<u8> {
    left.split(' ')
        .map(|x| x.bytes().fold(0u8, |n, b| n | (1 << (b - b'a'))))
        .collect()
}

fn contains(larger: u8, smaller: u8) -> bool {
    larger & smaller == smaller
}

fn segments_to_digit(segments: u8) -> usize {
    match segments.count_ones() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if contains(segments, 1 << 5) {
                5
            } else if contains(segments, 1 << 4) {
                2
            } else {
                3
            }
        }
        6 => {
            let contains_segment_4 = contains(segments, 1 << 4);
            if contains_segment_4 && contains(segments, 1 << 1) {
                0
            } else if contains_segment_4 {
                6
            } else {
                9
            }
        }
        7 => 8,
        _ => unreachable!(segments),
    }
}

fn solve(wirings: &[u8]) -> Vec<u8> {
    let candidates = wirings.iter().fold(vec![0b1111111; 7], |mut candidates, &wiring| {
        match wiring.count_ones() {
            2 => {
                // 1
                candidates[1] = wiring;
                candidates[2] = wiring;
                candidates[3] &= !wiring;
                candidates[4] &= !wiring;
                candidates[5] &= !wiring;
                candidates[6] &= !wiring;
            }
            3 => {
                // 7
                let top_segment = wiring & !candidates[1];
                candidates[0] = top_segment;
                candidates[3] &= !top_segment;
                candidates[4] &= !top_segment;
                candidates[5] &= !top_segment;
                candidates[6] &= !top_segment;
            }
            4 => {
                // 4
                let remaining = wiring & !candidates[0] & !candidates[1];
                candidates[5] = remaining;
                candidates[6] = remaining;
                candidates[3] &= !remaining;
                candidates[4] &= !remaining;
            }
            5 => {
                candidates[3] &= wiring;
                candidates[6] &= wiring;
                if contains(wiring, candidates[5]) {
                    // 5
                    candidates[2] &= wiring;
                    candidates[5] &= wiring;
                } else if contains(wiring, candidates[4]) {
                    // 2
                    candidates[1] &= wiring;
                    candidates[4] &= wiring;
                } else {
                    // 3
                    candidates[1] &= wiring;
                    candidates[2] &= wiring;
                }
            }
            6 => {
                let contains_segment_4 = contains(wiring, candidates[4]);
                let segment_to_update = if contains_segment_4 && contains(wiring, candidates[1]) {
                    // 0
                    6
                } else if contains_segment_4 {
                    // 6
                    1
                } else {
                    // 9
                    4
                };
                candidates[segment_to_update] &= !(candidates[segment_to_update] & wiring);
            }
            _ => unreachable!(),
        };
        candidates
    });

    let knowns = candidates
        .iter()
        .copied()
        .filter(|&n| n.count_ones() == 1)
        .collect::<Vec<u8>>();

    candidates
        .into_iter()
        .map(|n| {
            knowns
                .iter()
                .fold(n, |n, &known| if n != known { n & !known } else { n })
        })
        .collect()
}

impl Day<'_> for Day08 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            entries: input
                .lines()
                .map(|l| {
                    let (left, right) = split_pair(l, " | ").unwrap();
                    (
                        parse_segments(left)
                            .iter()
                            .copied()
                            .filter(|n| n.count_ones() < 7)
                            .sorted_unstable_by_key(|n| n.count_ones())
                            .collect::<Vec<_>>(),
                        parse_segments(right),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.entries
            .iter()
            .map(|(_, right)| {
                right
                    .iter()
                    .filter(|n| [2u8, 3u8, 4u8, 7u8].contains(&(n.count_ones() as u8)))
                    .count()
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        self.entries
            .iter()
            .map(|(wirings, digits)| {
                let wiring = solve(wirings);
                // println!(
                //     "wiring:\n{}\n",
                //     wiring
                //         .iter()
                //         .copied()
                //         .enumerate()
                //         .map(|(i, n)| format!("{}: {}", i, format!("{:07b}", n).replace('0', ".")))
                //         .join("\n")
                // );
                digits.iter().fold(0, |n, &digit| {
                    n * 10
                        + segments_to_digit(wiring.iter().enumerate().fold(0, |segments, (segment, &wire)| {
                            if contains(digit, wire) {
                                segments | (1 << segment)
                            } else {
                                segments
                            }
                        }))
                })
            })
            .sum::<usize>()
    }
}
