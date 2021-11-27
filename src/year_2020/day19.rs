use crate::day::Day;
use crate::util::split_pair;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Rule {
    Char(u8),
    Composite((Vec<usize>, Option<Vec<usize>>)),
}

use Rule::*;

pub struct Day19<'a> {
    rules: Vec<Rule>,
    messages: Vec<&'a str>,
}

fn matches_branch(rules: &[Rule], sub: &Vec<usize>, message: &str, n: usize) -> (bool, usize) {
    let mut matches = true;
    let mut i = 0;
    let mut it = sub.iter().peekable();

    while matches && it.peek().is_some() {
        let (m, j) = matches_rule(rules, *it.next().unwrap(), &message[i..], n + 1);
        matches = m;
        i += if matches { j } else { 0 };
    }

    (matches, i)
}

fn format_rule(rules: &[Rule], i: usize) -> String {
    format!(
        "{}: {}",
        i,
        match &rules[i] {
            Char(c) => String::from(*c as char),
            Composite((a, b)) => format!(
                "{}{}",
                a.iter().join(" "),
                b.as_ref()
                    .map(|b| format!(" | {}", b.iter().join(" ")))
                    .unwrap_or(String::new())
            ),
        }
    )
}

fn matches_rule(rules: &[Rule], rule_index: usize, message: &str, n: usize) -> (bool, usize) {
    let (m, i) = match &rules[rule_index] {
        Char(expected) => message
            .bytes()
            .next()
            .map(|c| (c == *expected, 1))
            .unwrap_or((false, 0)),
        Composite((a, b)) => {
            let (matches, i) = matches_branch(rules, &a, message, n);
            if matches {
                (true, i)
            } else {
                b.as_ref()
                    .map(|br| matches_branch(rules, &br, message, n))
                    .unwrap_or((false, 0))
            }
        }
    };
    println!(
        "{}{} - {} {}",
        "└".repeat(n),
        format_rule(&rules, rule_index),
        message,
        if m { '✔' } else { '✗' }
    );
    (m, i)
}

fn matches(rules: &[Rule], message: &str) -> bool {
    println!("### {}", message);
    let (matches, i) = matches_rule(rules, 0, message, 0);
    matches && i == message.len()
}

fn parse_branch(input: &str) -> Vec<usize> {
    input.split(' ').map(|n| n.parse().unwrap()).collect()
}

fn parse_rule(input: &str) -> Rule {
    let mut bytes = input.bytes();
    if bytes.next().unwrap() == b'"' {
        Char(bytes.next().unwrap())
    } else {
        let mut branches = input.splitn(2, " | ");
        Composite((
            branches.next().map(parse_branch).unwrap(),
            branches.next().map(parse_branch),
        ))
    }
}

impl<'a> Day<'a> for Day19<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let (all_rules, messages) = split_pair(input, "\n\n").unwrap();
        let mut rules = all_rules
            .lines()
            .map(|l| split_pair(l, ": "))
            .map(Option::unwrap)
            .map(|(i, r)| (i.parse::<usize>().unwrap(), parse_rule(r)))
            .collect::<Vec<_>>();
        rules.sort_by_key(|&(i, _)| i);

        Day19 {
            rules: rules.into_iter().map(|(_, r)| r).collect(),
            messages: messages.lines().collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        0
        // self.messages
        //     .iter()
        //     .filter(|&message| matches(&self.rules, message))
        //     .count()
    }

    fn part_2(&self) -> Self::T2 {
        let mut rules = self.rules.clone();
        // 8: 42 | 42 8
        // rules[8] = Composite((vec![42], Some(vec![42, 8])));
        rules[8] = Composite((vec![30], Some(vec![30, 8])));
        // 11: 42 31 | 42 11 31
        // rules[11] = Composite((vec![42, 31], Some(vec![42, 11, 31])));
        rules[11] = Composite((vec![30, 29], Some(vec![30, 11, 29])));
        self.messages
            .iter()
            .filter(|&message| matches(&rules, message))
            .inspect(|&message| println!("{}", message))
            .count()
    }
}
