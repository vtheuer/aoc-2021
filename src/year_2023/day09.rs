use crate::day::Day;

pub struct Day09 {
    histories: Vec<Vec<isize>>,
}

fn get_variations(vec: &[isize]) -> Vec<isize> {
    vec.windows(2).fold(Vec::new(), |mut variations, w| {
        variations.push(w[1] - w[0]);
        variations
    })
}

impl Day<'_> for Day09 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            histories: input
                .lines()
                .map(|l| l.split(' ').map(|h| h.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.histories
            .iter()
            .map_while(|history| {
                let mut variations = history.clone();
                let mut r = *history.last()?;

                while variations.iter().any(|&v| v != 0) {
                    variations = get_variations(&variations);
                    r += *variations.last()?;
                }

                Some(r)
            })
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.histories
            .iter()
            .map_while(|history| {
                let mut variations = history.clone();
                let mut firsts = vec![*history.first()?];

                while variations.iter().any(|&v| v != 0) {
                    variations = get_variations(&variations);
                    firsts.push(*variations.first()?);
                }

                Some(firsts.into_iter().rev().fold(0, |r, v| v - r))
            })
            .sum()
    }
}
