use std::convert::identity;

use crate::day::Day;
use crate::util::SortableByKey;

pub struct Day15 {
    sensors_and_beacons: Vec<((isize, isize), (isize, isize))>,
}

impl Day<'_> for Day15 {
    type T1 = usize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            sensors_and_beacons: input
                .lines()
                .map(|l| {
                    let (sensor, beacon) = l["Sensor at x=".len()..]
                        .split_once(": closest beacon is at x=")
                        .unwrap();
                    let (sx, sy) = sensor.split_once(", y=").unwrap();
                    let (bx, by) = beacon.split_once(", y=").unwrap();
                    (
                        (sx.parse().unwrap(), sy.parse().unwrap()),
                        (bx.parse().unwrap(), by.parse().unwrap()),
                    )
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let row = 2000000;
        self.sensors_and_beacons
            .iter()
            .map(|&((sx, sy), (bx, by))| ((sx, sy), (bx, by), sx.abs_diff(bx) + sy.abs_diff(by), sy.abs_diff(row)))
            .filter(|&(_, _, distance_to_beacon, distance_to_row)| distance_to_row <= distance_to_beacon)
            .map(|((sx, _), _, distance_to_beacon, distance_to_row)| {
                let half_width = (distance_to_beacon - distance_to_row) as isize;
                (sx - half_width, sx + half_width)
            })
            .sorted_unstable_by_key(|&i| i)
            .fold(vec![], |mut intervals: Vec<(isize, isize)>, (f, t)| {
                if let Some(last) = intervals.last_mut() {
                    if f > last.1 {
                        intervals.push((f, t));
                    } else if f <= last.1 && t > last.1 {
                        last.1 = t;
                    }
                } else {
                    intervals.push((f, t));
                }
                intervals
            })
            .into_iter()
            .map(|(f, t)| f.abs_diff(t))
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        (0..4000000)
            .find_map(|row| {
                let mut intervals = self
                    .sensors_and_beacons
                    .iter()
                    .map(|&((sx, sy), (bx, by))| {
                        ((sx, sy), (bx, by), sx.abs_diff(bx) + sy.abs_diff(by), sy.abs_diff(row))
                    })
                    .filter(|&(_, _, distance_to_beacon, distance_to_row)| distance_to_row <= distance_to_beacon)
                    .map(|((sx, _), _, distance_to_beacon, distance_to_row)| {
                        let half_width = (distance_to_beacon - distance_to_row) as isize;
                        (sx - half_width, sx + half_width)
                    })
                    .collect::<Vec<_>>();
                intervals.sort();

                let mut last_to = intervals[0].1;
                for &(f, t) in &intervals[1..] {
                    if f > last_to {
                        return Some((f - 1) * 4000000 + row);
                    } else if f <= last_to && t > last_to {
                        last_to = t;
                    }
                }
                None
            })
            .unwrap()
    }
}
