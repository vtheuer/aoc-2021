use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};
use Ordering::Greater;

use regex::internal::Input;

use Packet::*;

use crate::day::Day;
use crate::util::SortableByKey;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &str, i: &mut usize) -> Self {
        let bytes = input.as_bytes();
        match bytes[*i] {
            b'0'..=b'9' => {
                let s = &input[*i..];
                let j = s
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .map(|(j, _)| j)
                    .last()
                    .unwrap_or(s.len() - 1);
                *i += j + 1;
                Integer(s[..=j].parse().unwrap_or_else(|_| panic!("{}", s)))
            }
            b'[' => {
                let mut sub_packets = vec![];
                *i += 1;
                while *i < input.len() && bytes[*i] != b']' {
                    match bytes[*i] {
                        b',' => *i += 1,
                        _ => sub_packets.push(Packet::parse(input, i)),
                    }
                }
                *i += 1;
                List(sub_packets)
            }
            _ => panic!("{}", input),
        }
    }

    fn is_integer(&self) -> bool {
        matches!(self, Integer(_))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (List(a), List(b)) => {
                let mut i = 0;
                while i < a.len() && i < b.len() {
                    match a[i].cmp(&b[i]) {
                        Equal => i += 1,
                        r => return Some(r),
                    }
                }

                if i == a.len() {
                    if i == b.len() {
                        Equal
                    } else {
                        Less
                    }
                } else {
                    Greater
                }
            }
            _ => {
                if self.is_integer() {
                    List(vec![self.clone()]).cmp(other)
                } else {
                    self.cmp(&List(vec![other.clone()]))
                }
            }
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Day13 {
    packets: Vec<Packet>,
}

impl Day<'_> for Day13 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            packets: input
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| Packet::parse(l, &mut 0))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.packets
            .chunks(2)
            .enumerate()
            .filter(|(_, pair)| pair[0].cmp(&pair[1]) == Less)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        let k1 = List(vec![List(vec![Integer(2)])]);
        let k2 = List(vec![List(vec![Integer(6)])]);
        let (nk1, nk2) = self.packets.iter().fold((1, 2), |(mut nk1, mut nk2), packet| {
            if packet <= &k1 {
                nk1 += 1;
                nk2 += 1;
            } else if packet <= &k2 {
                nk2 += 1;
            }
            (nk1, nk2)
        });
        nk1 * nk2
    }
}
