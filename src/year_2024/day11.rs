use crate::day::Day;
use crate::util::{count_digits, first_line};
use ahash::{AHashMap, AHashSet};

pub struct Day11 {
    stones: Vec<usize>,
}

impl Day11 {
    fn count_stones(&self, blinks: usize) -> usize {
        let mut stones = self.stones.iter().copied().fold(AHashMap::default(), |mut r, stone| {
            *r.entry(stone).or_insert(0) += 1;
            r
        });

        for _ in 0..blinks {
            stones = stones
                .into_iter()
                .fold(AHashMap::default(), |mut next, (stone, count)| {
                    if stone == 0 {
                        *next.entry(1).or_insert(0) += count;
                    } else {
                        let digit_count = count_digits(stone);
                        if digit_count % 2 == 0 {
                            let x = 10usize.pow(digit_count / 2);
                            *next.entry(stone / x).or_insert(0) += count;
                            *next.entry(stone % x).or_insert(0) += count;
                        } else {
                            *next.entry(stone * 2024).or_insert(0) += count;
                        }
                    }

                    next
                });
        }

        stones.values().sum()
    }
}

impl Day<'_> for Day11 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            stones: first_line(input)
                .split(' ')
                .filter_map(|stone| stone.parse().ok())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.count_stones(25)
    }

    fn part_2(&self) -> Self::T2 {
        self.count_stones(75)
    }
}
