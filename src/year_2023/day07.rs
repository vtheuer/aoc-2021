use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day07<'a> {
    hands: Vec<(&'a [u8], usize)>,
}

fn score(hand: &[u8], index: fn(u8) -> usize, combinaison_score: fn(&[u8]) -> usize) -> usize {
    let counts = hand.iter().fold([0u8; 13], |mut c, &card| {
        c[index(card)] += 1;
        c
    });

    let mut score = combinaison_score(&counts) << (4 * 5);

    for (i, &card) in hand.iter().enumerate() {
        score += index(card) << (4 * (4 - i));
    }

    score
}

impl<'a> Day07<'a> {
    fn compute(&self, index: fn(u8) -> usize, combinaison_score: fn(&[u8]) -> usize) -> usize {
        self.hands
            .iter()
            .map(|&(hand, bid)| (score(hand, index, combinaison_score), bid))
            .sorted_unstable_by_key(|&(score, _)| score)
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }
}

impl<'a> Day<'a> for Day07<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            hands: input
                .lines()
                .map_while(|l| {
                    let (hand, bid) = l.split_once(' ')?;
                    Some((hand.as_bytes(), bid.parse().ok()?))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let index = |card| match card {
            b'2'..=b'9' => (card - b'2') as usize,
            b'T' => 8,
            b'J' => 9,
            b'Q' => 10,
            b'K' => 11,
            b'A' => 12,
            _ => unreachable!(),
        };
        let combinaison_score = |counts: &[u8]| {
            let max_count = counts.iter().copied().max().unwrap();
            if max_count == 5 {
                6
            } else if max_count == 4 {
                5
            } else if max_count == 3 {
                if counts.contains(&2) {
                    4
                } else {
                    3
                }
            } else {
                match counts.iter().filter(|&&c| c == 2).count() {
                    2 => 2,
                    1 => 1,
                    0 => 0,
                    _ => unreachable!(),
                }
            }
        };
        self.compute(index, combinaison_score)
    }

    fn part_2(&self) -> Self::T2 {
        let index = |card| match card {
            b'J' => 0,
            b'2'..=b'9' => (card - b'1') as usize,
            b'T' => 9,
            b'Q' => 10,
            b'K' => 11,
            b'A' => 12,
            _ => unreachable!(),
        };
        let combinaison_score = |counts: &[u8]| {
            let jokers = counts[0];
            let max_count = counts.iter().skip(1).copied().max().unwrap() + jokers;
            let pairs_without_joker = counts.iter().skip(1).filter(|&&c| c == 2).count();

            if max_count == 5 {
                6
            } else if max_count == 4 {
                5
            } else if max_count == 3 {
                if pairs_without_joker > if jokers > 0 { 1 } else { 0 } {
                    4
                } else {
                    3
                }
            } else if jokers > 0 {
                1
            } else {
                match pairs_without_joker {
                    2 => 2,
                    1 => 1,
                    0 => 0,
                    _ => unreachable!(),
                }
            }
        };
        self.compute(index, combinaison_score)
    }
}
