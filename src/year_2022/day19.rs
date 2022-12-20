use regex::Regex;
use std::collections::VecDeque;

use crate::day::Day;
use crate::year_2022::day19::Material::Ore;

#[derive(Debug, Clone)]
struct Blueprint {
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

#[derive(Debug)]
struct State {
    time: usize,
    ore: (usize, usize),
    clay: (usize, usize),
    obsidian: (usize, usize),
    geode: (usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use Material::*;

impl State {
    fn can_build_bot(&self, b: &Blueprint, m: Material) -> bool {
        match m {
            Ore => self.ore.1 >= b.ore_cost,
            Clay => self.ore.1 >= b.clay_cost,
            Obsidian => self.ore.1 >= b.obsidian_cost.0 && self.clay.1 >= b.obsidian_cost.1,
            Geode => self.ore.1 >= b.geode_cost.0 && self.obsidian.1 >= b.geode_cost.1,
        }
    }

    fn build_bot(&mut self, b: &Blueprint, m: Material) {
        match m {
            Ore => {
                self.ore.0 += 1;
                self.ore.1 -= b.ore_cost;
            }
            Clay => {
                self.clay.0 += 1;
                self.ore.1 -= b.clay_cost;
            }
            Obsidian => {
                self.obsidian.0 += 1;
                self.ore.1 -= b.obsidian_cost.0;
                self.clay.1 -= b.obsidian_cost.1;
            }
            Geode => {
                self.geode.0 += 1;
                self.ore.1 -= b.geode_cost.0;
                self.obsidian.1 -= b.geode_cost.1;
            }
        }
    }

    fn advance(&mut self) {
        self.ore.1 += self.ore.0;
        self.clay.1 += self.clay.0;
        self.obsidian.1 += self.obsidian.0;
        self.geode.1 += self.geode.0;
        self.time += 1;
    }
}

impl Blueprint {
    fn score(&self) -> usize {
        let obsidian_bot_goal = self.geode_cost.1;
        let clay_bot_goal = self.obsidian_cost.1;
        let ore_bot_goal =
            self.geode_cost.0 + self.obsidian_cost.0 * obsidian_bot_goal + self.clay_cost * clay_bot_goal;
        let mut next_bot_to_build = Clay;
        let mut state = State {
            time: 0,
            ore: (1, 0),
            clay: (0, 0),
            obsidian: (0, 0),
            geode: (0, 0),
        };

        while state.time < 24 {
            if state.can_build_bot(self, next_bot_to_build) {
                state.build_bot(self, next_bot_to_build);
                next_bot_to_build = if ore_bot_goal > state.ore.0 {
                    Ore
                } else if clay_bot_goal > state.clay.0 {
                    Clay
                } else if obsidian_bot_goal > state.obsidian.0 {
                    Obsidian
                } else {
                    Geode
                }
            }
            state.advance();
            dbg!(&state);
            dbg!(next_bot_to_build);
        }

        state.geode.1
    }

    fn score2(&self) -> usize {
        let mut max = 0;
        let mut queue = VecDeque::from([State {
            time: 0,
            ore: (1, 0),
            clay: (0, 0),
            obsidian: (0, 0),
            geode: (0, 0),
        }]);
        let mut visited = 0;

        while let Some(State {
            time,
            ore,
            clay,
            obsidian,
            geode,
        }) = queue.pop_front()
        {
            visited += 1;
            max = max.max(geode.1);
            if visited % 100 == 0 {
                dbg!(visited);
                dbg!(max);
                dbg!(queue.len());
                dbg!(time);
            }

            if time <= 24 {
                if ore.1 >= self.geode_cost.0 && obsidian.1 >= self.geode_cost.1 {
                    queue.push_back(State {
                        time: time + 1,
                        ore: (ore.0, ore.1 - self.geode_cost.0 + ore.0),
                        clay: (clay.0, clay.1 + clay.0),
                        obsidian: (obsidian.0, obsidian.1 - self.geode_cost.1 + obsidian.0),
                        geode: (geode.0 + 1, geode.1 + geode.0),
                    });
                }

                if ore.1 >= self.obsidian_cost.0 && clay.1 >= self.obsidian_cost.1 {
                    queue.push_back(State {
                        time: time + 1,
                        ore: (ore.0, ore.1 - self.obsidian_cost.0 + ore.0),
                        clay: (clay.0, clay.1 - self.obsidian_cost.1 + clay.0),
                        obsidian: (obsidian.0 + 1, obsidian.1 + obsidian.0),
                        geode: (geode.0, geode.1 + geode.0),
                    });
                }

                if ore.1 >= self.clay_cost {
                    queue.push_back(State {
                        time: time + 1,
                        ore: (ore.0, ore.1 - self.clay_cost + ore.0),
                        clay: (clay.0 + 1, clay.1 + clay.0),
                        obsidian: (obsidian.0, obsidian.1 + obsidian.0),
                        geode: (geode.0, geode.1 + geode.0),
                    });
                }

                if ore.1 >= self.ore_cost {
                    queue.push_back(State {
                        time: time + 1,
                        ore: (ore.0 + 1, ore.1 - self.ore_cost + ore.0),
                        clay: (clay.0, clay.1 + clay.0),
                        obsidian: (obsidian.0, obsidian.1 + obsidian.0),
                        geode: (geode.0, geode.1 + geode.0),
                    });
                }

                queue.push_back(State {
                    time: time + 1,
                    ore: (ore.0, ore.1 + ore.0),
                    clay: (clay.0, clay.1 + clay.0),
                    obsidian: (obsidian.0, obsidian.1 + obsidian.0),
                    geode: (geode.0, geode.1 + geode.0),
                });
            }
        }
        dbg!(visited);

        max
    }
}

pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

impl Day<'_> for Day19 {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &str) -> Self {
        let re = Regex::new("Blueprint \\d+: Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.").unwrap();
        Self {
            blueprints: input
                .lines()
                .map_while(|l| {
                    let captures = re.captures(l)?;
                    let get = |i| captures.get(i)?.as_str().parse().ok();
                    Some(Blueprint {
                        ore_cost: get(1)?,
                        clay_cost: get(2)?,
                        obsidian_cost: (get(3)?, get(4)?),
                        geode_cost: (get(5)?, get(6)?),
                    })
                })
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.blueprints
            .iter()
            .enumerate()
            .take(1)
            .map(|(i, b)| (i + 1) * b.score())
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        0
    }
}
