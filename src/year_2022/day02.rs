use crate::day::Day;

pub struct Day02 {
    rounds: Vec<(u8, u8)>,
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            rounds: input
                .lines()
                .map(&str::as_bytes)
                .map(|l| (l[0] - b'A', l[2] - b'X'))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.rounds
            .iter()
            .map(|&(opponent_move, player_move)| {
                player_move
                    + 1
                    + if player_move == opponent_move {
                        3
                    } else if player_move == (opponent_move + 1) % 3 {
                        6
                    } else {
                        0
                    }
            })
            .map(|score| score as usize)
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.rounds
            .iter()
            .map(|&(opponent_move, outcome)| {
                1 + outcome * 3
                    + match outcome {
                        0 => {
                            if opponent_move == 0 {
                                2
                            } else {
                                opponent_move - 1
                            }
                        }
                        1 => opponent_move,
                        _ => (opponent_move + 1) % 3,
                    }
            })
            .map(|score| score as usize)
            .sum()
    }
}
