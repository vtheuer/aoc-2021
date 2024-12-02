use crate::day::Day;

pub struct Day02 {
    reports: Vec<Vec<u8>>,
}

fn is_safe(report: &[u8]) -> bool {
    let ordering = report[1].cmp(&report[0]);
    let mut prev = report[0];
    for &v in report.iter().skip(1) {
        if ordering != v.cmp(&prev) || !(1..=3).contains(&v.abs_diff(prev)) {
            return false;
        }
        prev = v;
    }

    true
}

impl Day<'_> for Day02 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            reports: input
                .lines()
                .map(|l| l.split(' ').map(|v| v.parse().unwrap()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.reports.iter().filter(|report| is_safe(report)).count()
    }

    fn part_2(&self) -> Self::T2 {
        self.reports
            .iter()
            .filter(|report| {
                is_safe(report)
                    || (0..report.len()).any(|i| {
                        is_safe(
                            &report[0..i]
                                .iter()
                                .chain(report[i + 1..].iter())
                                .copied()
                                .collect::<Vec<_>>(),
                        )
                    })
            })
            .count()
    }
}
