use crate::day::Day;

pub struct Day08<'a> {
    instructions: Vec<(&'a str, isize)>,
}

fn run(instructions: &[(&str, isize)]) -> (bool, isize) {
    let mut pointer = 0;
    let mut accumulator = 0;
    let mut visited = (0..instructions.len()).map(|_| false).collect::<Vec<_>>();

    while pointer < instructions.len() && !visited[pointer] {
        visited[pointer] = true;
        match instructions[pointer] {
            ("acc", n) => {
                accumulator += n;
                pointer += 1
            }
            ("jmp", n) => pointer = (pointer as isize + n) as usize,
            ("nop", _) => pointer += 1,
            (op, n) => unreachable!("unknown op: {} {}", op, n),
        }
    }

    (pointer >= instructions.len(), accumulator)
}

impl<'a> Day<'a> for Day08<'a> {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &'a str) -> Self {
        Day08::<'a> {
            instructions: input
                .lines()
                .map(|l| {
                    let mut split = l.split(' ');
                    Some((split.next()?, split.next()?.parse().ok()?))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        run(&self.instructions).1
    }

    fn part_2(&self) -> Self::T2 {
        self.instructions
            .iter()
            .enumerate()
            .filter_map(|(i, (op, _))| match *op {
                "jmp" => Some((i, "nop")),
                "nop" => Some((i, "jmp")),
                _ => None,
            })
            .map(|(i, op)| {
                let mut instructions = self.instructions.clone();
                instructions[i].0 = op;
                run(&instructions)
            })
            .find(|(terminates, _)| *terminates)
            .unwrap()
            .1
    }
}
