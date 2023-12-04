use crate::day::Day;

pub struct Day04 {
    points: Vec<usize>,
}

impl Day<'_> for Day04 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            points: input
                .lines()
                .map_while(|l| {
                    let (left, right) = l.split_once(" | ")?;
                    let winning = left
                        .split_once(": ")?
                        .1
                        .split(' ')
                        .filter(|n| !n.is_empty())
                        .map_while(|n| n.parse::<usize>().ok())
                        .fold(vec![false; 100], |mut r, n| {
                            r[n] = true;
                            r
                        });
                    Some(
                        right
                            .split(' ')
                            .filter(|n| !n.is_empty())
                            .map_while(|n| n.parse::<usize>().ok())
                            .filter(|&n| winning[n])
                            .count(),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.points
            .iter()
            .filter(|&&points| points > 0)
            .map(|&points| 1 << (points - 1))
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.points
            .iter()
            .enumerate()
            .fold(vec![1; self.points.len()], |mut card_counts, (i, &points)| {
                for j in 1..=points {
                    card_counts[i + j] += card_counts[i];
                }
                card_counts
            })
            .into_iter()
            .sum()
    }
}
