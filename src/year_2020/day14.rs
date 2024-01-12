use ahash::AHashMap;
use Instruction::*;

use crate::day::Day;

#[derive(Debug)]
enum Instruction {
    Mask(Vec<Option<bool>>),
    Mem(usize, usize),
}

pub struct Day14 {
    instructions: Vec<Instruction>,
}

fn get(value: usize, i: usize) -> bool {
    value & (1 << (35 - i)) != 0
}

fn set(value: usize, i: usize, bit: bool) -> usize {
    let mask = 1 << (35 - i);
    if bit {
        value | mask
    } else {
        value & !mask
    }
}

fn addresses(mask: &[Option<bool>], address: usize) -> Vec<usize> {
    let (mut addresses, last_part) = mask
        .iter()
        .enumerate()
        .fold((vec![0], 0), |(addresses, part), (i, &b)| match b {
            Some(bit) => (addresses, set(part, i, bit || get(address, i))),
            None => (
                addresses
                    .iter()
                    .fold(Vec::with_capacity(addresses.len() * 2), |mut addresses, &address| {
                        addresses.push(set(address | part, i, false));
                        addresses.push(set(address | part, i, true));
                        addresses
                    }),
                0,
            ),
        });

    if last_part > 0 {
        for address in addresses.iter_mut() {
            *address |= last_part;
        }
    }

    addresses
}

impl Day<'_> for Day14 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day14 {
            instructions: input
                .lines()
                .map(|l| match &l[..4] {
                    "mask" => Mask(
                        l.bytes()
                            .skip(7)
                            .map(|c| match c {
                                b'0' => Some(false),
                                b'1' => Some(true),
                                b'X' => None,
                                c => unreachable!("unexpected char {}", c),
                            })
                            .collect(),
                    ),
                    "mem[" => {
                        let (address, value) = &l[4..].split_once("] = ").unwrap();
                        Mem(address.parse().unwrap(), value.parse().unwrap())
                    }
                    s => unreachable!("unexpected start {}", s),
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.instructions
            .iter()
            .fold(
                (AHashMap::default(), vec![]),
                |(mut memory, mask), instruction| match instruction {
                    Mask(m) => (
                        memory,
                        m.iter()
                            .enumerate()
                            .filter_map(|(i, bit)| bit.map(|b| (b, i)))
                            .collect(),
                    ),
                    Mem(address, value) => {
                        memory.insert(
                            *address,
                            mask.iter()
                                .fold(*value, |masked_value, &(bit, i)| set(masked_value, i, bit)),
                        );
                        (memory, mask)
                    }
                },
            )
            .0
            .values()
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        self.instructions
            .iter()
            .fold(
                (AHashMap::default(), &vec![]),
                |(mut memory, mask), instruction| match instruction {
                    Mask(m) => (memory, m),
                    Mem(address, value) => {
                        addresses(mask, *address).into_iter().for_each(|masked_address| {
                            memory.insert(masked_address, *value);
                        });

                        (memory, mask)
                    }
                },
            )
            .0
            .values()
            .sum::<usize>()
    }
}
