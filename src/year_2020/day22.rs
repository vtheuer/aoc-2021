use crate::day::Day;
use fnv::{FnvHashSet, FnvHasher};
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::ptr::hash;

pub struct Day22 {
    decks: (Vec<u8>, Vec<u8>),
}

fn update_decks(a: &mut VecDeque<u8>, b: &mut VecDeque<u8>, a_card: u8, b_card: u8, a_wins: bool) {
    if a_wins {
        a.push_back(a_card);
        a.push_back(b_card);
    } else {
        b.push_back(b_card);
        b.push_back(a_card);
    }
}

fn score(winner: &VecDeque<u8>) -> usize {
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| card as usize * (i + 1))
        .sum::<usize>()
}
fn recursive_combat(mut a: VecDeque<u8>, mut b: VecDeque<u8>) -> (bool, VecDeque<u8>) {
    let mut known_decks = FnvHashSet::default();

    loop {
        let mut hasher = FnvHasher::default();
        a.hash(&mut hasher);
        b.hash(&mut hasher);
        if !known_decks.insert(hasher.finish()) {
            return (true, a);
        } else {
            let a_card = a.pop_front().unwrap();
            let b_card = b.pop_front().unwrap();

            let a_wins = if a.len() >= a_card as usize && b.len() >= b_card as usize {
                recursive_combat(
                    a.iter().take(a_card as usize).copied().collect(),
                    b.iter().take(b_card as usize).copied().collect(),
                )
                .0
            } else {
                a_card > b_card
            };

            update_decks(&mut a, &mut b, a_card, b_card, a_wins);

            if a.is_empty() {
                return (false, b);
            } else if b.is_empty() {
                return (true, a);
            }
        }
    }
}

impl Day<'_> for Day22 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let (a, b) = input.split_once("\n\n").unwrap();
        Day22 {
            decks: (
                a.lines().skip(1).map(|n| n.parse().unwrap()).collect(),
                b.lines().skip(1).map(|n| n.parse().unwrap()).collect(),
            ),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut a = VecDeque::from(self.decks.0.clone());
        let mut b = VecDeque::from(self.decks.1.clone());

        while !a.is_empty() && !b.is_empty() {
            let a_card = a.pop_front().unwrap();
            let b_card = b.pop_front().unwrap();
            update_decks(&mut a, &mut b, a_card, b_card, a_card > b_card);
        }

        score(&if a.is_empty() { b } else { a })
    }

    fn part_2(&self) -> Self::T2 {
        score(
            &recursive_combat(
                VecDeque::from(self.decks.0.clone()),
                VecDeque::from(self.decks.1.clone()),
            )
            .1,
        )
    }
}
