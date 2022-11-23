use std::cmp::{max, min, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::thread::sleep;
use std::time::Duration;

use fnv::FnvHashSet;

use crate::day::Day;
use crate::util::Joinable;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Position {
    // 0 = top, n = nth room
    space: u8,
    // top: left to right, room: top to bottom
    position: u8,
}

fn room_entry(room: u8) -> u8 {
    room * 2
}

fn distance(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn steps(space: u8, from: u8, to: u8) -> impl Iterator<Item = Position> {
    let it: Box<dyn Iterator<Item = u8> + '_> = if to > from {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    };
    it.map(move |position| Position::new(space, position))
}

impl Position {
    fn new(space: u8, position: u8) -> Self {
        Self { space, position }
    }

    fn path(&self, to: &Self) -> impl Iterator<Item = Self> {
        let path: Box<dyn Iterator<Item = Self> + '_> = if self.space == 0 {
            if to.space == 0 {
                // top -> top
                unreachable!()
            }
            // top -> room
            Box::new(steps(0, self.position, room_entry(to.space)).chain(steps(to.space, 0, to.position)))
        } else if to.space == 0 {
            // room -> top
            Box::new(steps(self.space, self.position, 0).chain(steps(0, room_entry(self.space), to.position)))
        } else {
            // room -> room
            unreachable!()
        };
        path.skip(1)
    }

    fn distance(&self, to: &Self) -> usize {
        (if self.space == 0 {
            if to.space == 0 {
                // top -> top
                unreachable!()
            }
            // top -> room
            distance(self.position, room_entry(to.space)) + 1 + to.position
        } else if to.space == 0 {
            // room -> top
            self.position + 1 + distance(room_entry(self.space), to.position)
        } else {
            // room -> room
            unreachable!()
        }) as usize
    }

    fn cost(&self, to: &Position, amphipod: usize) -> usize {
        self.distance(to) * [1, 10, 100, 1000][amphipod]
    }

    fn h(&self) -> u128 {
        (1 << self.position) << if self.space == 0 { 0 } else { 9 + 2 * self.space }
    }
}

trait Stated {
    fn next_states(&self) -> dyn Iterator<Item = (Self, usize)> + '_
    where
        Self: Sized;
}

impl Stated for u128 {
    fn next_states(&self) -> impl Iterator<Item = (Self, usize)> + '_ {}
}

#[derive(Clone)]
struct State {
    cost: usize,
    positions: Vec<Position>,
}

fn target(i: usize) -> u8 {
    1 + i as u8 / 2
}

fn path_is_free(occupied: u128, from: &Position, to: &Position) -> bool {
    from.path(to).all(|position| position.h() & occupied == 0)
}

impl State {
    fn new(cost: usize, positions: Vec<Position>) -> Self {
        State { cost, positions }
    }

    fn parse(input: &str) -> Self {
        let lines = input.lines().skip(2).take(2).map(|l| &l[3..]).collect::<Vec<_>>();
        Self::new(
            0,
            (0..4)
                .map(|i| (0..2).map(|j| lines[j].as_bytes()[i * 2]).collect::<Vec<_>>())
                .enumerate()
                .fold(vec![vec![]; 4], |mut positions: Vec<Vec<Position>>, (space, room)| {
                    room.into_iter().enumerate().for_each(|(position, amphipod)| {
                        positions[(amphipod - b'A') as usize].push(Position::new(space as u8 + 1, position as u8))
                    });
                    positions
                })
                .into_iter()
                .flat_map(|v| v.into_iter())
                .collect(),
        )
    }

    fn is_free(&self, position: &Position) -> bool {
        !self.positions.iter().any(|&p| p == *position)
    }

    fn path_is_free(&self, from: &Position, to: &Position) -> bool {
        from.path(to).all(|position| self.is_free(&position))
    }

    fn position_is_final(&self, space: u8, position: u8) -> bool {
        let x = 2 * (space as usize - 1);
        self.positions[x..=x + 1]
            .iter()
            .any(|&p| p.space == space && p.position == position)
    }

    fn next_positions(&self, i: usize) -> impl Iterator<Item = Position> + '_ {
        let position = self.positions[i];
        let target = target(i);
        let occupied = self
            .positions
            .chunks(2)
            .map(|c| c[0].h() | c[1].h())
            .fold(0, |r, h| r | h);
        let next_positions: Box<dyn Iterator<Item = Position> + '_> = match position.space {
            0 => Box::new(
                (0..2)
                    .rev()
                    .map(move |i| Position::new(target, i))
                    .find(move |next| path_is_free(occupied, &position, next))
                    .into_iter(),
            ),
            room if room != target || position.position == 0 && !self.position_is_final(room, 1) => Box::new(
                [0, 1, 3, 5, 7, 9, 10]
                    .into_iter()
                    .map(|i| Position::new(0, i))
                    .filter(move |next| path_is_free(occupied, &position, next)),
            ),
            _ => Box::new([].into_iter()),
        };
        next_positions
    }

    fn move_to(&self, i: usize, to: Position, base_cost: usize) -> Self {
        let mut positions = self.positions.clone();
        let cost = self.positions[i].cost(&to, i / 2);
        positions[i] = to;
        Self {
            cost: base_cost + cost,
            positions,
        }
    }

    fn next_states(&self, base_cost: usize) -> impl Iterator<Item = Self> + '_ {
        (0..self.positions.len())
            .flat_map(move |i| self.next_positions(i).map(move |next| self.move_to(i, next, base_cost)))
    }

    fn is_done(&self) -> bool {
        self.positions
            .iter()
            .enumerate()
            .all(|(i, &Position { space, position: _ })| space == target(i))
    }

    fn h(&self) -> u128 {
        // 1122334400000000000
        self.positions
            .chunks(2)
            .map(|c| c[0].h() | c[1].h())
            .fold(0, |r, h| (r << 19) | h)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid = [
            "#############",
            "#...........#",
            "###.#.#.#.###",
            "  #.#.#.#.#",
            "  #########",
        ]
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        for (i, &Position { space, position }) in self.positions.iter().enumerate() {
            *match space as usize {
                0 => &mut grid[1][1 + position as usize],
                room => &mut grid[2 + position as usize][1 + room * 2],
            } = ['A', 'B', 'C', 'D'][i / 2];
        }

        write!(
            f,
            "{}",
            grid.into_iter().map(|l| l.into_iter().collect::<String>()).join("\n")
        )
    }
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.h() == other.h()
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.h().hash(state)
    }
}

pub struct Day23 {
    initial_state: State,
}

impl Day<'_> for Day23 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            initial_state: State::parse(input),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut heap = BinaryHeap::from([Reverse(self.initial_state.clone())]);
        let mut seen = FnvHashSet::default();
        let final_h = State::new(
            0,
            vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(3, 0),
                Position::new(3, 1),
                Position::new(4, 0),
                Position::new(4, 1),
            ],
        )
        .h();

        while let Some(Reverse(state)) = heap.pop() {
            // println!("{}:\n{}\n", cost, state);
            // sleep(Duration::from_secs(1));
            // println!("{} {}", heap.len(), seen.len());
            let h = state.h();
            let cost = state.cost;
            if h == final_h {
                return cost;
            } else if !seen.contains(&state) {
                seen.insert(state.clone());
                state
                    .next_states(cost)
                    .map(Reverse)
                    .filter(|Reverse(new_state)| !seen.contains(new_state))
                    .for_each(|e| heap.push(e));
            }
        }

        unreachable!()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
