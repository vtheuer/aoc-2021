use crate::day::Day;

pub struct Day01 {
    depths: Vec<u16>,
}

fn count_increasing(depths: &[u16]) -> usize {
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

impl Day<'_> for Day01 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Day01 {
            depths: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        count_increasing(&self.depths)
    }

    fn part_2(&self) -> Self::T2 {
        count_increasing(&self.depths.windows(3).map(|w| w.iter().sum()).collect::<Vec<_>>())
    }
}
