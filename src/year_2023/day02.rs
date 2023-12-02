use crate::day::Day;

pub struct Day02 {
    games: Vec<Vec<(u8, u8, u8)>>,
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            games: input
                .lines()
                .map_while(|l| Some(l.split_once(": ")?.1))
                .map(|games| {
                    games
                        .split("; ")
                        .map(|game| {
                            game.split(", ")
                                .map_while(|color| color.split_once(' '))
                                .map(|(count, color)| (count.parse::<u8>().unwrap(), color))
                                .fold((0, 0, 0), |(r, g, b), (count, color)| match color {
                                    "red" => (r + count, g, b),
                                    "green" => (r, g + count, b),
                                    "blue" => (r, g, b + count),
                                    _ => unreachable!(),
                                })
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.games
            .iter()
            .enumerate()
            .filter(|(_, game)| game.iter().all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14))
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.games
            .iter()
            .map(|game| {
                let (mr, mg, mb) = game
                    .iter()
                    .fold((0, 0, 0), |(mr, mg, mb), &(r, g, b)| (mr.max(r), mg.max(g), mb.max(b)));
                mr as usize * mg as usize * mb as usize
            })
            .sum()
    }
}
