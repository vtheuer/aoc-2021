use Instruction::*;

use crate::day::Day;

enum Instruction {
    Addx(i8),
    Noop,
}

pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl Day<'_> for Day10 {
    type T1 = isize;
    type T2 = &'static str;

    fn new(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|l| {
                    if l.as_bytes()[0] == b'n' {
                        Noop
                    } else {
                        Addx(l.split_once(' ').unwrap().1.parse().unwrap())
                    }
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        let step = |sum: &mut isize, cycle: &mut isize, register: isize| {
            *cycle += 1;
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    *sum += *cycle * register;
                }
                _ => {}
            }
        };
        self.instructions
            .iter()
            .fold((0, 0, 1isize), |(mut sum, mut cycle, mut register), i| {
                match i {
                    Noop => {
                        step(&mut sum, &mut cycle, register);
                    }
                    Addx(v) => {
                        step(&mut sum, &mut cycle, register);
                        step(&mut sum, &mut cycle, register);
                        register += *v as isize;
                    }
                };

                (sum, cycle, register)
            })
            .0
    }

    fn part_2(&self) -> Self::T2 {
        let step = |screen: &mut String, cycle: &mut isize, register: isize| {
            *cycle += 1;
            let position = *cycle % 40;
            screen.push(if position.abs_diff(register) < 2 { '#' } else { ' ' });
            if position == 0 {
                screen.push('\n');
            }
        };

        println!(
            "{}",
            self.instructions
                .iter()
                .fold(
                    (String::new(), 0, 1isize),
                    |(mut screen, mut cycle, mut register), i| {
                        match i {
                            Noop => {
                                step(&mut screen, &mut cycle, register);
                            }
                            Addx(v) => {
                                step(&mut screen, &mut cycle, register);
                                register += *v as isize;
                                step(&mut screen, &mut cycle, register);
                            }
                        };
                        (screen, cycle, register)
                    },
                )
                .0
        );
        "see above"
    }
}
