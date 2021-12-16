use std::cell::Cell;
use std::cmp::min;

use crate::day::Day;

struct Bits<'a> {
    bytes: &'a [u8],
    total_bits: usize,
    current_bit: usize,
}

impl<'a> Bits<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            total_bits: bytes.len() * 8,
            current_bit: 0,
        }
    }

    fn read_number(&mut self, bit_count: usize) -> usize {
        self.take(bit_count)
            .fold(0, |number, bit| (number << 1) | (bit as usize))
    }

    fn read_bytes(&mut self, bit_count: usize) -> Vec<u8> {
        self.take(bit_count).enumerate().fold(vec![], |mut bytes, (i, bit)| {
            if i / 8 >= bytes.len() {
                bytes.push(0);
            }
            bytes[i / 8] |= if bit == 1 { 0b10000000 >> (i % 8) } else { 0 };
            bytes
        })
    }
}

impl Iterator for Bits<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.current_bit < self.total_bits {
            Some((self.bytes[self.current_bit / 8] & 0b10000000 >> (self.current_bit % 8)).count_ones() as u8)
        } else {
            None
        };
        self.current_bit += 1;
        r
    }
}

fn read(bits: &mut Bits) -> (usize, usize) {
    let version = bits.read_number(3);
    let packet_type = bits.read_number(3);

    if packet_type == 4 {
        let mut value = 0;
        let mut is_last = false;
        while !is_last {
            is_last = bits.next() == Some(0);
            value = (value << 4) | bits.read_number(4);
        }
        (version, value)
    } else {
        let length_type_id = bits.read_number(1);

        let (versions, operands) = if length_type_id == 0 {
            let sub_packets_total_length = bits.read_number(15);
            let sub_bytes = bits.read_bytes(sub_packets_total_length);
            let mut sub_bits = Bits::new(&sub_bytes);
            let mut versions = 0;
            let mut operands = vec![];
            while sub_bits.current_bit < sub_packets_total_length {
                let (v, operand) = read(&mut sub_bits);
                versions += v;
                operands.push(operand);
            }
            (versions, operands)
        } else {
            let sub_packets_count = bits.read_number(11);
            (0..sub_packets_count).map(|_| read(bits)).fold(
                (0, vec![]),
                |(mut versions, mut operands), (v, operand)| {
                    versions += v;
                    operands.push(operand);
                    (versions, operands)
                },
            )
        };
        (
            version + versions,
            match packet_type {
                0 => operands.into_iter().sum::<usize>(),
                1 => operands.into_iter().product::<usize>(),
                2 => operands.into_iter().min().unwrap(),
                3 => operands.into_iter().max().unwrap(),
                5 => {
                    if operands[0] > operands[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if operands[0] < operands[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if operands[0] == operands[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        )
    }
}

pub struct Day16 {
    bytes: Vec<u8>,
    value: Cell<usize>,
}

impl Day<'_> for Day16 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let length = input.len();
        Self {
            bytes: (0..length)
                .step_by(2)
                .map(|i| u8::from_str_radix(&input[i..min(i + 2, length)], 16))
                .filter_map(Result::ok)
                .collect(),
            value: Cell::new(0),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let (version, value) = read(&mut Bits::new(&self.bytes));
        self.value.set(value);
        version
    }

    fn part_2(&self) -> Self::T2 {
        self.value.get()
    }
}
