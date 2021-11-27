use crate::day::Day;

pub struct Day15 {
    numbers: Vec<u32>,
}

impl Day15 {
    fn play(&self, end: u32) -> usize {
        let start = self.numbers.len() as u32 - 1;
        let mut last_seen = vec![u32::MAX; end as usize];
        for (i, n) in self.numbers.iter().enumerate().take(start as usize) {
            last_seen[*n as usize] = i as u32;
        }

        (start..end - 1).fold(self.numbers[start as usize] as usize, |previous, i| {
            let next = match last_seen[previous] {
                u32::MAX => 0,
                j => i - j,
            } as usize;
            last_seen[previous] = i;

            next
        })
    }
}

impl Day<'_> for Day15 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day15 {
            numbers: input
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.play(2020)
    }

    fn part_2(&self) -> Self::T2 {
        self.play(30_000_000)
    }
}
