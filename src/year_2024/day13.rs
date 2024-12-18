use crate::day::Day;

#[derive(Copy, Clone, Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn parse_line(line: &str) -> Option<(isize, isize)> {
        let (x, y) = line.split_once(": ")?.1.split_once(", ")?;
        Some((x[2..].parse().ok()?, y[2..].parse().ok()?))
    }

    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        Some(Machine {
            a: lines.next().and_then(Self::parse_line)?,
            b: lines.next().and_then(Self::parse_line)?,
            prize: lines.next().and_then(Self::parse_line)?,
        })
    }

    fn lowest_cost(&self) -> Option<isize> {
        let &Machine {
            a: (ax, ay),
            b: (bx, by),
            prize: (px, py),
        } = self;
        let na = (by * px - bx * py) / (ax * by - bx * ay);
        let nb = (ay * px - ax * py) / (bx * ay - ax * by);

        if (na * ax + nb * bx, na * ay + nb * by) == (px, py) {
            Some(na * 3 + nb)
        } else {
            None
        }
    }
}

pub struct Day13 {
    machines: Vec<Machine>,
}

impl Day<'_> for Day13 {
    type T1 = isize;
    type T2 = isize;

    fn new(input: &str) -> Self {
        Self {
            machines: input.split("\n\n").filter_map(Machine::parse).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.machines.iter().filter_map(Machine::lowest_cost).sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.machines
            .iter()
            .map(|&Machine { a, b, prize: (x, y) }| Machine {
                a,
                b,
                prize: (10000000000000 + x, 10000000000000 + y),
            })
            .filter_map(|m| m.lowest_cost())
            .sum()
    }
}
