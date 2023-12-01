use crate::day::Day;

pub struct Day01<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Day<'a> for Day01<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.lines
            .iter()
            .map_while(|l| {
                let digits = l
                    .bytes()
                    .filter(u8::is_ascii_digit)
                    .map(|b| b - b'0')
                    .collect::<Vec<_>>();
                Some(*digits.first()? as usize * 10 + *digits.last()? as usize)
            })
            .sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        let digits: Vec<&[u8]> = vec![
            b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
        ];

        self.lines
            .iter()
            .map(|l| find(&digits, l, false) * 10 + find(&digits, l, true))
            .sum::<usize>()
    }
}

fn find(digits: &[&[u8]], line: &str, rev: bool) -> usize {
    let l = line.as_bytes();
    let mut it: Box<dyn DoubleEndedIterator<Item = (usize, u8)>> = Box::new(l.iter().copied().enumerate());

    if rev {
        it = Box::new(it.rev());
    }

    it.find_map(|(i, b)| match b {
        b'0'..=b'9' => Some((b - b'0') as usize),
        _ => digits.iter().position(|d| l[i..].starts_with(d)),
    })
    .unwrap()
}
