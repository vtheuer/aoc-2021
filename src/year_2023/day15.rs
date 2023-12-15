use crate::day::Day;
use crate::util::FindIndex;

pub struct Day15<'a> {
    strings: Vec<&'a [u8]>,
}

fn hash(s: &[u8]) -> usize {
    s.iter().fold(0, |h, &b| (h + b as usize) * 17 % 256)
}

impl<'a> Day<'a> for Day15<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            strings: input.lines().next().unwrap().split(',').map(|s| s.as_bytes()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.strings.iter().map(|&s| hash(s)).sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.strings
            .iter()
            .map(|s| {
                let (i, &action) = s.iter().find_index_by(|&&b| b == b'-' || b == b'=').unwrap();
                (
                    &s[0..i],
                    action,
                    Some(i + 1).filter(|&j| j < s.len()).map(|j| s[j] - b'0'),
                )
            })
            .fold(vec![Vec::new(); 256], |mut boxes, (label, action, focal)| {
                let b = &mut boxes[hash(label)];
                let lens_index = b.iter().find_index_by(|&&(l, _)| l == label).map(|(i, _)| i);
                if action == b'-' {
                    if let Some(i) = lens_index {
                        b.remove(i);
                    }
                } else {
                    let f = focal.unwrap();
                    if let Some(i) = lens_index {
                        b[i].1 = f;
                    } else {
                        b.push((label, f));
                    }
                }
                boxes
            })
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                b.into_iter()
                    .enumerate()
                    .map(|(j, (_, focal))| (i + 1) * (j + 1) * focal as usize)
                    .sum::<usize>()
            })
            .sum()
    }
}
