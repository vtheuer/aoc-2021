use crate::day::Day;
use crate::util::{rsplit_pair, split_pair};
use fnv::{FnvHashMap, FnvHashSet};

pub struct Day07<'a> {
    rules: FnvHashMap<&'a str, FnvHashMap<&'a str, u32>>,
}

impl Day07<'_> {
    fn count_bags_in(&self, container: &str) -> u32 {
        self.rules
            .get(container)
            .unwrap()
            .iter()
            .map(|(bag, count)| *count * (1 + self.count_bags_in(bag)))
            .sum()
    }
}

fn bags_containing<'a>(containers_by_bag: &FnvHashMap<&str, Vec<&'a str>>, bag: &str) -> FnvHashSet<&'a str> {
    containers_by_bag
        .get(bag)
        .map(|bags| {
            bags.iter().fold(FnvHashSet::default(), |mut containers, container| {
                containers.insert(*container);
                containers.extend(bags_containing(containers_by_bag, container));
                containers
            })
        })
        .unwrap_or_else(FnvHashSet::default)
}

impl<'a> Day<'a> for Day07<'a> {
    type T1 = usize;
    type T2 = u32;

    fn new(input: &'a str) -> Self {
        Day07::<'a> {
            rules: input
                .lines()
                .map(|l| {
                    let (bag, content) = split_pair(l, " bags contain ")?;
                    Some((
                        bag,
                        content
                            .split(", ")
                            .filter_map(|content| {
                                let (count_and_bag, _) = rsplit_pair(content, " ")?;
                                let (count, bag) = split_pair(count_and_bag, " ")?;

                                Some((bag, count.parse().ok()?))
                            })
                            .collect(),
                    ))
                })
                .map(Option::unwrap)
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        bags_containing(
            &self
                .rules
                .iter()
                .flat_map(|(container, content)| content.iter().map(move |(bag, _)| (*bag, *container)))
                .fold(FnvHashMap::default(), |mut containers_by_bag, (bag, container)| {
                    containers_by_bag.entry(bag).or_insert_with(Vec::new).push(container);
                    containers_by_bag
                }),
            "shiny gold",
        )
        .len()
    }

    fn part_2(&self) -> Self::T2 {
        self.count_bags_in("shiny gold")
    }
}
