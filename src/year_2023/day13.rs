use crate::day::Day;

pub struct Day13 {
    patterns: Vec<Vec<Vec<bool>>>,
}

fn mirror(len: usize, diff: usize, count_diff: impl Fn(usize, usize) -> usize) -> Option<usize> {
    (1..len).find(|&y| {
        let mut i = y as isize - 1;
        let mut j = y;
        let mut current_diff = 0;

        while i >= 0 && j < len && current_diff <= diff {
            current_diff += count_diff(i as usize, j);
            i -= 1;
            j += 1;
        }

        (i < 0 || j >= len) && current_diff == diff
    })
}

fn mirror_h(pattern: &[Vec<bool>], diff: usize) -> Option<usize> {
    mirror(pattern.len(), diff, |i, j| {
        pattern[i]
            .iter()
            .zip(pattern[j].iter())
            .filter(|&(&a, &b)| a != b)
            .count()
    })
}

fn mirror_v(pattern: &[Vec<bool>], diff: usize) -> Option<usize> {
    mirror(pattern[0].len(), diff, |i, j| {
        pattern.iter().filter(|row| row[i] != row[j]).count()
    })
}

impl Day13 {
    fn compute(&self, diff: usize) -> usize {
        self.patterns
            .iter()
            .map(|pattern| {
                mirror_h(pattern, diff)
                    .map(|y| y * 100)
                    .or_else(|| mirror_v(pattern, diff))
                    .unwrap()
            })
            .sum()
    }
}

impl Day<'_> for Day13 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            patterns: input
                .split("\n\n")
                .map(|p| p.lines().map(|l| l.bytes().map(|b| b == b'#').collect()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.compute(0)
    }

    fn part_2(&self) -> Self::T2 {
        self.compute(1)
    }
}
