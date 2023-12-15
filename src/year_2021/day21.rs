use std::cmp::{max, min};

use num::Integer;

use crate::day::Day;

pub struct Day21 {
    p1: u8,
    p2: u8,
}

fn circular<T: Integer + Copy>(n: T, max: T) -> T {
    let r = n % max;
    if r == T::zero() {
        max
    } else {
        r
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    p1: u8,
    s1: u16,
    p2: u8,
    s2: u16,
    p1_turn: bool,
}

impl State {
    fn new(p1: u8, p2: u8) -> Self {
        Self {
            p1,
            s1: 0,
            p2,
            s2: 0,
            p1_turn: true,
        }
    }

    fn play(&self, roll: u16) -> Self {
        let &Self {
            p1,
            s1,
            p2,
            s2,
            p1_turn,
        } = self;
        let p = circular(if p1_turn { p1 } else { p2 } as u16 + roll, 10) as u8;
        if p1_turn {
            State {
                p1: p,
                s1: s1 + p as u16,
                p2,
                s2,
                p1_turn: !p1_turn,
            }
        } else {
            State {
                p1,
                s1,
                p2: p,
                s2: s2 + p as u16,
                p1_turn: !p1_turn,
            }
        }
    }

    fn to_usize(self) -> usize {
        let Self {
            p1,
            s1,
            p2,
            s2,
            p1_turn,
        } = self;
        let mut r = p1 as usize;
        r = (r << 5) | s1 as usize;
        r = (r << 4) | p2 as usize;
        r = (r << 5) | s2 as usize;
        r = (r << 1) | usize::from(p1_turn);
        r
    }
}

struct Die {
    value: u8,
    rolls: usize,
}

impl Die {
    fn new() -> Self {
        Self { value: 1, rolls: 0 }
    }

    fn roll3(&mut self) -> u16 {
        let v = self.value as u16;
        let r = 3 * v + 3 - 100 * v.saturating_sub(100);
        self.value = circular(v as u8 + 3, 100);
        self.rolls += 3;
        r
    }
}

// 3 => 1: 1 1 1
// 4 => 3: 1 1 2 | 1 2 1 | 2 1 1
// 5 => 6: 1 1 3 | 1 3 1 | 3 1 1 | 1 2 2 | 2 1 2 | 2 2 1
// 6 => 7: 1 2 3 | 1 3 2 | 2 1 3 | 2 3 1 | 3 1 2 | 3 2 1 | 2 2 2
// 7 => 6: 1 3 3 | 3 1 3 | 3 3 1 | 2 2 3 | 2 3 2 | 3 2 2
// 8 => 3: 2 3 3 | 3 2 3 | 3 3 2
// 9 => 1: 3 3 3
const MULT: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn play(state: State, cache: &mut Vec<Option<(usize, usize)>>) -> (usize, usize) {
    if state.s1 >= 21 {
        (1, 0)
    } else if state.s2 >= 21 {
        (0, 1)
    } else {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        for roll in 3..=9 {
            let new_state = state.play(roll);
            let i = new_state.to_usize();
            let (sub_p1_wins, sub_p2_wins) = if let Some(r) = cache[i] {
                r
            } else {
                let r = play(new_state, cache);
                cache[i] = Some(r);
                r
            };
            let mult = MULT[roll as usize];
            p1_wins += sub_p1_wins * mult;
            p2_wins += sub_p2_wins * mult;
        }
        (p1_wins, p2_wins)
    }
}

impl Day<'_> for Day21 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut positions = input
            .lines()
            .map(|l| l.split(": ").nth(1)?.parse().ok())
            .map(Option::unwrap);
        Self {
            p1: positions.next().unwrap(),
            p2: positions.next().unwrap(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut state = State::new(self.p1, self.p2);
        let mut die = Die::new();
        while state.s1 < 1000 && state.s2 < 1000 {
            state = state.play(die.roll3());
            if state.s1 < 1000 {
                state = state.play(die.roll3());
            }
        }
        die.rolls * min(state.s1, state.s2) as usize
    }

    fn part_2(&self) -> Self::T2 {
        let (p1_wins, p2_wins) = play(State::new(self.p1, self.p2), &mut vec![None; 2usize.pow(19)]);
        max(p1_wins, p2_wins)
    }
}
