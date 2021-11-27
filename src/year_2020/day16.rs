use crate::day::Day;
use crate::util::split_pair;
use fnv::FnvHashSet;
use std::cell::Cell;

pub struct Day16<'a> {
    constraints: Vec<(&'a str, (usize, usize), (usize, usize))>,
    my_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
    valid_tickets: Cell<Vec<usize>>,
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse(input: &str) -> Option<Day16> {
    let mut parts = input.split("\n\n");
    Some(Day16 {
        constraints: parts
            .next()?
            .lines()
            .map(|l| {
                let (field, constraints) = split_pair(l, ": ")?;
                let (range1, range2) = split_pair(constraints, " or ")?;
                let (min1, max1) = split_pair(range1, "-")?;
                let (min2, max2) = split_pair(range2, "-")?;
                Some((
                    field,
                    (min1.parse().ok()?, max1.parse().ok()?),
                    (min2.parse().ok()?, max2.parse().ok()?),
                ))
            })
            .map(Option::unwrap)
            .collect(),
        my_ticket: parse_ticket(parts.next()?.lines().last()?),
        other_tickets: parts.next()?.lines().skip(1).map(parse_ticket).collect(),
        valid_tickets: Cell::new(vec![]),
    })
}

fn is_valid(n: usize, (min1, max1): (usize, usize), (min2, max2): (usize, usize)) -> bool {
    n >= min1 && n <= max1 || n >= min2 && n <= max2
}

impl<'a> Day<'a> for Day16<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        parse(input).unwrap()
    }

    fn part_1(&self) -> Self::T1 {
        let valid_tickets = self
            .other_tickets
            .iter()
            .enumerate()
            .map(|(i, ticket)| {
                (
                    i,
                    ticket.iter().find(|&&n| {
                        self.constraints
                            .iter()
                            .all(|&(_, range1, range2)| !is_valid(n, range1, range2))
                    }),
                )
            })
            .collect::<Vec<_>>();
        self.valid_tickets.set(
            valid_tickets
                .iter()
                .filter(|&&(_, n)| n.is_none())
                .map(|&(ticket, _)| ticket)
                .collect(),
        );
        valid_tickets.iter().filter_map(|(_, n)| *n).sum::<usize>()
    }

    fn part_2(&self) -> Self::T2 {
        let valid_tickets = self.valid_tickets.take();
        let ticket_length = self.my_ticket.len();
        let mut possible_columns = self
            .constraints
            .iter()
            .map(|&(field, range1, range2)| {
                (
                    field,
                    (0..ticket_length)
                        .filter(|&i| {
                            valid_tickets.iter().all(|&ticket| {
                                is_valid(self.other_tickets[ticket][i], range1, range2)
                            })
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<(_, _)>>();

        possible_columns.sort_by_key(|(_, columns)| columns.len());

        possible_columns
            .into_iter()
            .scan(FnvHashSet::default(), |used, (field, columns)| {
                let column = *columns.iter().find(|&c| !used.contains(c)).unwrap();
                used.insert(column);
                Some((field, self.my_ticket[column]))
            })
            .filter(|&(field, _)| field.starts_with("departure"))
            .map(|(_, value)| value)
            .product::<usize>()
    }
}
