use crate::day::Day;

pub struct Day13 {
    time: usize,
    buses: Vec<(usize, usize)>,
}

impl Day<'_> for Day13 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        Day13 {
            time: lines.next().unwrap().parse().unwrap(),
            buses: lines
                .next()
                .unwrap()
                .split(',')
                .enumerate()
                .filter(|&(_, b)| b != "x")
                .map(|(i, b)| (b.parse().unwrap(), i))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.buses
            .iter()
            .map(move |(bus, _)| (bus, (self.time / bus + 1) * bus - self.time))
            .min_by_key(|&(_, wait)| wait)
            .map(|(bus, wait)| bus * wait)
            .unwrap()
    }

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    fn part_2(&self) -> Self::T2 {
        let product = self.buses.iter().map(|&(b, _)| b).product::<usize>();

        self.buses
            .iter()
            .map(|&(bus, i)| (bus, (i as f32 / bus as f32).ceil() as usize * bus - i))
            .map(|(bus, remainder)| {
                (1..)
                    .map(|n| n * product / bus)
                    .find(|n| n % bus == 1)
                    .map(|e| e * remainder)
                    .unwrap()
            })
            .sum::<usize>()
            % product
    }
}
