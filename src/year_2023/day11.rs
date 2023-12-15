use crate::day::Day;
use crate::util::first_line;

pub struct Day11 {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<bool>,
    empty_columns: Vec<bool>,
}

fn distance(empty: &[bool], a: usize, b: usize, expansion: usize) -> usize {
    let (from, to) = if a < b { (a, b) } else { (b, a) };
    let empty_count = empty[from..=to].iter().filter(|&&empty| empty).count();
    let not_empty_count = to - from - empty_count;
    empty_count * expansion + not_empty_count
}

impl Day11 {
    fn distance(&self, i: usize, j: usize, expansion: usize) -> usize {
        let (ax, ay) = self.galaxies[i];
        let (bx, by) = self.galaxies[j];

        distance(&self.empty_columns, ax, bx, expansion) + distance(&self.empty_rows, ay, by, expansion)
    }

    fn sum_of_distances(&self, expansion: usize) -> usize {
        let len = self.galaxies.len();
        (0..len)
            .flat_map(move |i| (i + 1..len).map(move |j| self.distance(i, j, expansion)))
            .sum()
    }
}

impl Day<'_> for Day11 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let height = input.lines().count();
        let width = first_line(input).len();
        let (galaxies, empty_rows, empty_columns) = input
            .lines()
            .enumerate()
            .flat_map(move |(y, l)| l.bytes().enumerate().map(move |(x, c)| (x, y, c)))
            .filter(|&(_, _, c)| c == b'#')
            .fold(
                (Vec::new(), vec![true; height], vec![true; width]),
                |(mut galaxies, mut empty_rows, mut empty_columns), (x, y, _)| {
                    galaxies.push((x, y));
                    empty_rows[y] = false;
                    empty_columns[x] = false;
                    (galaxies, empty_rows, empty_columns)
                },
            );
        Self {
            galaxies,
            empty_rows,
            empty_columns,
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.sum_of_distances(2)
    }

    fn part_2(&self) -> Self::T2 {
        self.sum_of_distances(1_000_000)
    }
}
