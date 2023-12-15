use crate::day::Day;
use crate::util::FindIndex;

pub struct Day20 {
    numbers: Vec<isize>,
}

fn shift(numbers: &mut Vec<(usize, isize)>, index: usize, shift: isize) {
    let len = numbers.len();
    let next: Box<dyn Fn(usize) -> usize> = if shift >= 0 {
        Box::new(|i| if i == len - 1 { 0 } else { i + 1 })
    } else {
        Box::new(|i| if i == 0 { len - 1 } else { i - 1 })
    };
    let n = numbers[index];
    let mut i = index;
    for _ in 0..(shift.abs() % (len as isize - 1)) {
        let j = next(i);
        numbers[i] = numbers[j];
        i = j;
    }
    numbers[i] = n;
}

fn shift_all(nums: &[isize], times: usize) -> isize {
    let mut numbers = nums.iter().copied().enumerate().collect::<Vec<_>>();
    let len = numbers.len();

    for _ in 0..times {
        for i in 0..len {
            let (current_index, _) = numbers
                .iter()
                .find_index_by(|&&(initial_index, _)| initial_index == i)
                .unwrap();
            let n = numbers[current_index].1;
            shift(&mut numbers, current_index, n);
        }
    }

    let index_of_0 = numbers.iter().find_index_by(|&&(_, v)| v == 0).map(|(i, _)| i).unwrap();

    numbers[(index_of_0 + 1000) % len].1 + numbers[(index_of_0 + 2000) % len].1 + numbers[(index_of_0 + 3000) % len].1
}

impl Day<'_> for Day20 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            numbers: input.lines().map_while(|l| l.parse().ok()).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        shift_all(&self.numbers, 1)
    }

    fn part_2(&self) -> Self::T2 {
        shift_all(
            &self.numbers.iter().copied().map(|n| n * 811589153).collect::<Vec<_>>(),
            10,
        )
    }
}
