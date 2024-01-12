use ahash::AHashMap;

use crate::day::Day;
use crate::year_2023::day19::RuleResult::{Accepted, Rejected, ToWorkflow};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Condition {
    category: u8,
    greater: bool,
    value: usize,
}

impl Condition {
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        Condition {
            category: bytes[0],
            greater: bytes[1] == b'>',
            value: input[2..].parse().unwrap(),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let v = match self.category {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => unreachable!(),
        };

        if self.greater {
            v > self.value
        } else {
            v < self.value
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum RuleResult<'a> {
    Accepted,
    Rejected,
    ToWorkflow(&'a str),
}

impl<'a> RuleResult<'a> {
    fn parse(input: &'a str) -> Self {
        match input {
            "A" => Accepted,
            "R" => Rejected,
            _ => ToWorkflow(input),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    result: RuleResult<'a>,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        if let Some((condition, result)) = input.split_once(':') {
            Rule {
                condition: Some(Condition::parse(condition)),
                result: RuleResult::parse(result),
            }
        } else {
            Rule {
                condition: None,
                result: RuleResult::parse(input),
            }
        }
    }

    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if self.condition.map(|c| c.matches(part)).unwrap_or(true) {
            Some(self.result)
        } else {
            None
        }
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn parse(input: &'a str) -> Self {
        let (name, rules) = input.split_once('{').unwrap();
        Workflow {
            name,
            rules: rules[..rules.len() - 1]
                .split(',')
                .map(|rule| Rule::parse(rule))
                .collect(),
        }
    }

    fn apply(&self, part: &Part) -> RuleResult {
        self.rules
            .iter()
            .find_map(|r| r.apply(part))
            .unwrap_or_else(|| panic!("{part:?} did not match any rule in {self:?}"))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(input: &str) -> Self {
        let mut categories = input[1..input.len() - 1]
            .split(',')
            .map(|c| c.split_once('=').unwrap().1)
            .map(|v| v.parse().unwrap());
        Self {
            x: categories.next().unwrap(),
            m: categories.next().unwrap(),
            a: categories.next().unwrap(),
            s: categories.next().unwrap(),
        }
    }

    fn rating(&self) -> usize {
        let &Part { x, m, a, s } = self;
        x + m + a + s
    }
}

pub struct Day19<'a> {
    workflows: AHashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

impl<'a> Day19<'a> {
    fn accepts(&self, part: &Part) -> bool {
        let mut name = "in";

        loop {
            match self.workflows[name].apply(part) {
                Accepted => return true,
                Rejected => return false,
                ToWorkflow(n) => name = n,
            }
        }
    }
}

impl<'a> Day<'a> for Day19<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
        Self {
            workflows: AHashMap::from_iter(workflows_str.lines().map(|l| Workflow::parse(l)).map(|w| (w.name, w))),
            parts: parts_str.lines().map(|l| Part::parse(l)).collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.parts
            .iter()
            .filter(|&part| self.accepts(part))
            .map(|p| p.rating())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
