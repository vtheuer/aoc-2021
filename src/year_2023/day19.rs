use ahash::AHashMap;

use Category::*;
use RuleResult::*;

use crate::day::Day;

#[derive(Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(b: u8) -> Self {
        match b {
            b'x' => X,
            b'm' => M,
            b'a' => A,
            b's' => S,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Condition {
    category: Category,
    greater: bool,
    value: usize,
}

impl Condition {
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        Condition {
            category: Category::parse(bytes[0]),
            greater: bytes[1] == b'>',
            value: input[2..].parse().unwrap(),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let v = match self.category {
            X => part.x,
            M => part.m,
            A => part.a,
            S => part.s,
        };

        if self.greater {
            v > self.value
        } else {
            v < self.value
        }
    }
}
#[derive(Copy, Clone)]
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

struct Rule<'a> {
    condition: Option<Condition>,
    result: RuleResult<'a>,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Self {
        let (condition, result) = input
            .split_once(':')
            .map(|(condition, result)| (Some(Condition::parse(condition)), result))
            .unwrap_or((None, input));
        Rule {
            condition,
            result: RuleResult::parse(result),
        }
    }

    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if self.condition.as_ref().map(|c| c.matches(part)).unwrap_or(true) {
            Some(self.result)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Ranges {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

impl Ranges {
    fn value(&self) -> usize {
        (self.x_max + 1).saturating_sub(self.x_min)
            * (self.m_max + 1).saturating_sub(self.m_min)
            * (self.a_max + 1).saturating_sub(self.a_min)
            * (self.s_max + 1).saturating_sub(self.s_min)
    }

    fn set_min(&mut self, category: Category, v: usize) {
        *(match category {
            X => &mut self.x_min,
            M => &mut self.m_min,
            A => &mut self.a_min,
            S => &mut self.s_min,
        }) = v;
    }

    fn set_max(&mut self, category: Category, v: usize) {
        *(match category {
            X => &mut self.x_max,
            M => &mut self.m_max,
            A => &mut self.a_max,
            S => &mut self.s_max,
        }) = v;
    }

    fn split(
        &self,
        Condition {
            category,
            value,
            greater,
        }: Condition,
    ) -> (Ranges, Ranges) {
        let mut matched = self.clone();
        let mut rest = self.clone();
        if greater {
            matched.set_min(category, value + 1);
            rest.set_max(category, value);
        } else {
            matched.set_max(category, value - 1);
            rest.set_min(category, value);
        }
        (matched, rest)
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl PartialEq<Self> for Workflow<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
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
        self.rules.iter().find_map(|r| r.apply(part)).unwrap()
    }

    fn count_accepted(&self, ranges: Ranges, workflows: &AHashMap<&str, Workflow>) -> usize {
        self.rules
            .iter()
            .scan(ranges.clone(), |current_ranges, &Rule { condition, result }| {
                let (matched, rest) = if let Some(c) = condition {
                    current_ranges.split(c)
                } else {
                    (current_ranges.clone(), current_ranges.clone())
                };
                *current_ranges = rest;
                Some(match result {
                    Accepted => matched.value(),
                    Rejected => 0,
                    ToWorkflow(w) => workflows[w].count_accepted(matched, workflows),
                })
            })
            .sum()
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
}

pub struct Day19<'a> {
    workflows: AHashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
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
            .filter(|&part| {
                let mut name = "in";
                loop {
                    match self.workflows[name].apply(part) {
                        Accepted => return true,
                        Rejected => return false,
                        ToWorkflow(n) => name = n,
                    }
                }
            })
            .map(|&Part { x, m, a, s }| x + m + a + s)
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.workflows["in"].count_accepted(
            Ranges {
                x_min: 1,
                x_max: 4000,
                m_min: 1,
                m_max: 4000,
                a_min: 1,
                a_max: 4000,
                s_min: 1,
                s_max: 4000,
            },
            &self.workflows,
        )
    }
}
