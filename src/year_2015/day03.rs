use fnv::FnvHashSet;

use crate::day::Day;

pub struct Day03<'a> {
    directions: &'a str,
}

fn visit(directions: impl Iterator<Item = u8>, seen: &mut FnvHashSet<(i32, i32)>) -> usize {
    directions.fold((0, 0), |(x, y), direction| {
        seen.insert((x, y));
        match direction {
            b'>' => (x + 1, y),
            b'v' => (x, y + 1),
            b'<' => (x - 1, y),
            b'^' => (x, y - 1),
            _ => unreachable!(),
        }
    });
    seen.len()
}

impl<'a> Day<'a> for Day03<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(directions: &'a str) -> Self {
        Self { directions }
    }

    fn part_1(&self) -> Self::T1 {
        visit(self.directions.bytes(), &mut FnvHashSet::default())
    }

    fn part_2(&self) -> Self::T2 {
        let (santa, robot): (Vec<_>, Vec<_>) = self.directions.bytes().enumerate().partition(|(i, _)| i % 2 == 0);
        let mut seen = FnvHashSet::default();
        visit(santa.into_iter().map(|(_, d)| d), &mut seen);
        visit(robot.into_iter().map(|(_, d)| d), &mut seen);
        seen.len()
    }
}
