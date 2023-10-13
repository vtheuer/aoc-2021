use std::clone;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem::swap;
use Ordering::{Equal, Greater, Less};

use fnv::FnvHashMap;

use crate::day::Day;

pub struct Day20 {
    numbers: Vec<isize>,
}

impl Day<'_> for Day20 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        Self {
            numbers: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let mut shifts = vec![0; self.numbers.len()];

        for (i, &n) in self.numbers.iter().enumerate() {
            let new_i = (i as isize - shifts[i]) as usize;
            match n.cmp(&0) {
                Greater => {
                    let j = new_i + n as usize;
                    let s = shifts[new_i];
                    dbg!(i);
                    dbg!(n);
                    dbg!(new_i);
                    dbg!(j);
                    dbg!(s);
                    for k in new_i..j {
                        shifts[k] = shifts[k + 1] + 1;
                    }
                    shifts[j] = s - n;
                }
                Less => {
                    if new_i as isize + n >= 0 {
                        unimplemented!()
                    } else {
                        let j = (new_i as isize + shifts.len() as isize - 1 + n) as usize;
                        let s = shifts[new_i];
                        dbg!(i);
                        dbg!(n);
                        dbg!(new_i);
                        dbg!(j);
                        dbg!(s);
                        for k in new_i..j {
                            shifts[k] = shifts[k + 1] + 1;
                        }
                        shifts[j] = s - (j - new_i) as isize;
                    }
                }
                _ => {}
            }
            dbg!(&shifts);
            dbg!(self.apply(&shifts));
        }

        0
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}

impl Day20 {
    fn apply(&self, shifts: &[isize]) -> Vec<isize> {
        shifts
            .iter()
            .enumerate()
            .map(|(i, &shift)| self.numbers[(i as isize + shift) as usize])
            .collect()
    }
}

fn rotate_right(v: &mut [usize], from: usize, to: usize) {
    let tmp = v[from];

    for i in (0..(to - from)).rev() {
        v[(from + i + 1) % v.len()] = v[(from + i) % v.len()]
    }

    v[to % v.len()] = tmp;
}

fn shift(current_indices: &mut [usize], values: &[isize], i: usize) {
    let shift = values[i];
    let new_index = (i as isize + shift) as usize;
    // let tmp = current_indices[new_index];
    // dbg!(i);
    // dbg!(shift);
    // dbg!(new_index);
    // dbg!(tmp);

    // for j in (i..new_index).rev() {
    //     current_indices[j + 1] = current_indices[j];
    // }
    rotate_right(current_indices, i, new_index);

    // current_indices[i] = tmp;
}

fn shifted(values: &[isize], indices: &[usize]) -> Vec<isize> {
    let mut shifted = vec![0; values.len()];

    for &i in indices {
        shifted[indices[i]] = values[i];
    }

    shifted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let values = vec![3, 2, 1, 0, -2];
        let mut current_indices = (0..values.len()).collect::<Vec<_>>();
        // 01234
        // 30124
        // 32014
        // 32104
        // 32104
        // 43102

        shift(&mut current_indices, &values, 0);
        assert_eq!(current_indices, vec![3, 0, 1, 2, 4]);
        assert_eq!(shifted(&values, &current_indices), vec![2, 1, 0, 3, -2]);

        shift(&mut current_indices, &values, 1);
        assert_eq!(current_indices, vec![3, 2, 0, 1, 4]);
        assert_eq!(shifted(&values, &current_indices), vec![1, 0, 2, 3, -2]);

        shift(&mut current_indices, &values, 2);
        assert_eq!(current_indices, vec![3, 2, 1, 0, 4]);
        assert_eq!(shifted(&values, &current_indices), vec![0, 1, 2, 3, -2]);

        shift(&mut current_indices, &values, 3);
        assert_eq!(current_indices, vec![3, 2, 1, 0, 4]);
        assert_eq!(shifted(&values, &current_indices), vec![0, 1, 2, 3, -2]);

        shift(&mut current_indices, &values, 4);
        assert_eq!(current_indices, vec![4, 3, 1, 0, 2]);
        assert_eq!(shifted(&values, &current_indices), vec![0, 1, -2, 2, 3]);
    }
}
