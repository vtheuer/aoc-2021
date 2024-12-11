use crate::day::Day;
use crate::util::count_digits;

pub struct Day06 {
    races: Vec<(usize, usize)>,
}

fn number_of_better_distances(time: f64, distance: f64) -> usize {
    // https://www.wolframalpha.com/input?i=-x%C2%B2%2BTx-D%3D0
    let discriminant = f64::sqrt(time * time - 4.0 * distance);
    let x1 = (time - discriminant) / 2.0;
    let x2 = (time + discriminant) / 2.0;
    let x1_ceil = x1.ceil();
    (x2.ceil() - if x1 == x1_ceil { x1_ceil + 1.0 } else { x1_ceil }) as usize
}

impl Day<'_> for Day06 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let mut lines = input.lines().map(|l| {
            l.split_once(':')
                .unwrap()
                .1
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
        });
        let times = lines.next().unwrap();
        let distances = lines.next().unwrap();
        Self {
            races: times.zip(distances).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.races
            .iter()
            .map(|&(time, distance)| (time as f64, distance as f64))
            .map(|(time, distance)| number_of_better_distances(time, distance))
            .product()
    }

    fn part_2(&self) -> Self::T2 {
        let (time, distance) = self.races.iter().fold((0, 0), |(time, distance), &(t, d)| {
            (
                time * 10usize.pow(count_digits(t)) + t,
                distance * 10usize.pow(count_digits(d)) + d,
            )
        });
        number_of_better_distances(time as f64, distance as f64)
    }
}
