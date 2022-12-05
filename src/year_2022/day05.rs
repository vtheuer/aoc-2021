use crate::day::Day;

pub struct Day05 {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Day05 {
    fn run<F>(&self, mut move_crates: F) -> String
    where
        F: FnMut(&mut Vec<Vec<u8>>, &(usize, usize, usize)),
    {
        self.instructions
            .iter()
            .fold(self.stacks.clone(), |mut stacks, instruction| {
                move_crates(&mut stacks, instruction);
                stacks
            })
            .into_iter()
            .map(|stack| *stack.last().unwrap() as char)
            .collect()
    }
}

impl Day<'_> for Day05 {
    type T1 = String;
    type T2 = String;

    fn new(input: &str) -> Self {
        let (stacks, instructions) = input.split_once("\n\n").unwrap();
        Self {
            stacks: stacks.lines().rev().skip(1).fold(vec![], |mut s, l| {
                let row = l.as_bytes().iter().skip(1).step_by(4);
                if s.is_empty() {
                    row.map(|&c| vec![c]).collect()
                } else {
                    for (i, c) in row.enumerate().filter(|(_, &c)| c != b' ') {
                        s[i].push(*c);
                    }
                    s
                }
            }),
            instructions: instructions
                .lines()
                .map_while(|l| {
                    let mut i = l
                        .split(|c: char| !c.is_digit(10))
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse().unwrap());
                    Some((i.next()?, i.next()? - 1, i.next()? - 1))
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.run(|stacks, &(m, f, t)| {
            for _ in 0..m {
                let x = stacks[f].pop().unwrap();
                stacks[t].push(x);
            }
        })
    }

    fn part_2(&self) -> Self::T2 {
        self.run(|stacks, &(m, f, t)| {
            let from = &mut stacks[f];
            let mut to_move = from.split_off(from.len() - m);
            stacks[t].append(&mut to_move);
        })
    }
}
