use fnv::FnvHashMap;

use crate::day::Day;

struct Record {
    springs: Vec<Option<bool>>,
    lengths: Vec<usize>,
}

impl Record {
    fn count(&self) -> usize {
        let mut cache: FnvHashMap<(usize, usize, usize, bool), usize> = FnvHashMap::default();
        self.count_rec(&mut cache, 0, 0, 0, false)
    }

    fn count_rec(
        &self,
        cache: &mut FnvHashMap<(usize, usize, usize, bool), usize>,
        spring_index: usize,
        current_length: usize,
        length_index: usize,
        previous_is_damaged: bool,
    ) -> usize {
        let lengths = &self.lengths;
        if spring_index >= self.springs.len() {
            return if current_length == 0 && length_index >= lengths.len() {
                1
            } else {
                0
            };
        }

        let mut get = |new_length, new_length_index, damaged| {
            let new_spring_index = spring_index + 1;
            let k = (new_spring_index, new_length, new_length_index, damaged);
            if let Some(&v) = cache.get(&k) {
                v
            } else {
                let v = self.count_rec(cache, new_spring_index, new_length, new_length_index, damaged);
                cache.insert(k, v);
                v
            }
        };

        match self.springs[spring_index] {
            Some(true) => {
                if current_length == 0 {
                    if previous_is_damaged || length_index >= lengths.len() {
                        0
                    } else {
                        get(lengths[length_index] - 1, length_index + 1, true)
                    }
                } else {
                    get(current_length - 1, length_index, true)
                }
            }
            Some(false) => {
                if current_length > 0 {
                    0
                } else {
                    get(0, length_index, false)
                }
            }
            None => {
                if current_length > 0 {
                    get(current_length - 1, length_index, true)
                } else if previous_is_damaged {
                    get(0, length_index, false)
                } else {
                    get(0, length_index, false)
                        + if length_index < lengths.len() {
                            get(lengths[length_index] - 1, length_index + 1, true)
                        } else {
                            0
                        }
                }
            }
        }
    }
}

pub struct Day12 {
    records: Vec<Record>,
}

impl Day<'_> for Day12 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            records: input
                .lines()
                .map(|l| {
                    let (springs, lengths) = l.split_once(' ').unwrap();
                    Record {
                        springs: springs
                            .bytes()
                            .map(|b| match b {
                                b'.' => Some(false),
                                b'#' => Some(true),
                                b'?' => None,
                                _ => unreachable!(),
                            })
                            .collect(),
                        lengths: lengths.split(',').map(str::parse).map(Result::unwrap).collect(),
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.records.iter().map(|record| record.count()).sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.records
            .iter()
            .map(|Record { springs, lengths }| Record {
                springs: (0..5).fold(Vec::new(), |mut r, _| {
                    if !r.is_empty() {
                        r.push(None);
                    }
                    r.append(&mut springs.clone());
                    r
                }),
                lengths: lengths.repeat(5),
            })
            .map(|record| record.count())
            .sum()
    }
}