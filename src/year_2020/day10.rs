use crate::day::Day;

pub struct Day10 {
    numbers: Vec<u32>,
}

impl Day<'_> for Day10 {
    type T1 = u32;
    type T2 = u64;

    fn new(input: &str) -> Self {
        let mut numbers = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>();
        numbers.sort_unstable();
        Day10 { numbers }
    }

    fn part_1(&self) -> Self::T1 {
        let (n1, n3) = self.numbers.windows(2).fold((1, 1), |(n1, n3), w| match w[1] - w[0] {
            1 => (n1 + 1, n3),
            3 => (n1, n3 + 1),
            n => unreachable!("unexpected difference {}", n),
        });
        n1 * n3
    }

    fn part_2(&self) -> Self::T2 {
        self.numbers
            .iter()
            .fold((0, 1, 1u64), |(prev, run, total), &n| {
                if n == prev + 1 {
                    (n, run + 1, total)
                } else if run > 2 {
                    (
                        n,
                        1,
                        total
                            * match run {
                                3 => 2,
                                4 => 4,
                                5 => 7,
                                n => unreachable!("unexpected run length {}", n),
                            },
                    )
                } else {
                    (n, 1, total)
                }
            })
            .2
    }
}
