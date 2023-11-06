use regex::Regex;

use Material::*;

use crate::day::Day;

#[derive(Debug, Clone)]
struct Blueprint {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: (u8, u8),
    geode_cost: (u8, u8),
}

#[derive(Debug, Clone)]
struct State {
    time: u8,
    ore: (u8, u8),
    clay: (u8, u8),
    obsidian: (u8, u8),
    geode: (u8, u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl State {
    fn bots(&self, m: Material) -> u8 {
        match m {
            Ore => self.ore.0,
            Clay => self.clay.0,
            Obsidian => self.obsidian.0,
            Geode => self.geode.0,
        }
    }

    fn stock(&self, m: Material) -> u8 {
        match m {
            Ore => self.ore.1,
            Clay => self.clay.1,
            Obsidian => self.obsidian.1,
            Geode => self.geode.1,
        }
    }

    fn should_build_bot(&self, blueprint: &Blueprint, max_ore_cost: u8, material: Material) -> bool {
        match material {
            Ore => self.bots(Ore) < max_ore_cost,
            Clay => self.bots(Clay) < blueprint.obsidian_cost.1,
            Obsidian => self.bots(Obsidian) < blueprint.geode_cost.1,
            Geode => true,
        }
    }

    fn build_bot(&self, b: &Blueprint, duration: u8, m: Material) -> Option<State> {
        let time = 1 + match m {
            Ore => self.time_to_have_enough(Ore, b.ore_cost)?,
            Clay => self.time_to_have_enough(Ore, b.clay_cost)?,
            Obsidian => self
                .time_to_have_enough(Ore, b.obsidian_cost.0)?
                .max(self.time_to_have_enough(Clay, b.obsidian_cost.1)?),
            Geode => self
                .time_to_have_enough(Ore, b.geode_cost.0)?
                .max(self.time_to_have_enough(Obsidian, b.geode_cost.1)?),
        };

        if self.time + time > duration {
            return None;
        }

        let mut s = self.advance_by(time);
        match m {
            Ore => {
                s.ore.0 += 1;
                s.ore.1 -= b.ore_cost;
            }
            Clay => {
                s.clay.0 += 1;
                s.ore.1 -= b.clay_cost;
            }
            Obsidian => {
                s.obsidian.0 += 1;
                s.ore.1 -= b.obsidian_cost.0;
                s.clay.1 -= b.obsidian_cost.1;
            }
            Geode => {
                s.geode.0 += 1;
                s.ore.1 -= b.geode_cost.0;
                s.obsidian.1 -= b.geode_cost.1;
            }
        };
        Some(s)
    }

    fn time_to_have_enough(&self, material: Material, cost: u8) -> Option<u8> {
        let stock = self.stock(material);
        if stock >= cost {
            Some(0)
        } else {
            let bots = self.bots(material);
            if bots == 0 {
                None
            } else {
                Some((cost - stock).div_ceil(bots))
            }
        }
    }

    fn advance_by(&self, time: u8) -> Self {
        let mut s = self.clone();
        s.time += time;
        s.ore.1 += time * s.bots(Ore);
        s.clay.1 += time * s.bots(Clay);
        s.obsidian.1 += time * s.bots(Obsidian);
        s.geode.1 += time * s.bots(Geode);
        s
    }

    fn score(&self, duration: u8) -> u8 {
        self.stock(Geode) + self.bots(Geode) * (duration - self.time)
    }

    fn could_do_better(&self, duration: u8, best: u8) -> bool {
        let remaining_time = (duration - self.time) as usize;

        // max possible score = stock + bots + (bots + 1) + (bots + 2) + (bots + 3)...
        //                    = stock + bots + (bots + 1) + (bots + 2) + (bots + 3)...
        //                    = stock + bots + remaining time * bots + sum(1..=remaining_time)
        self.stock(Geode) as usize
            + (remaining_time + 1) * self.bots(Geode) as usize
            + (remaining_time * (remaining_time + 1)) / 2
            > best as usize
    }
}

impl Blueprint {
    fn score(&self, duration: u8) -> usize {
        let max_ore_cost = self
            .ore_cost
            .max(self.clay_cost)
            .max(self.obsidian_cost.0)
            .max(self.geode_cost.0);
        let mut next = vec![State {
            time: 0,
            ore: (1, 0),
            clay: (0, 0),
            obsidian: (0, 0),
            geode: (0, 0),
        }];
        let mut best = 0;

        while let Some(state) = next.pop() {
            let score = state.score(duration);
            if score >= best {
                best = score;
            }

            for material in [Ore, Obsidian, Clay, Geode] {
                if state.should_build_bot(self, max_ore_cost, material) {
                    if let Some(s) = state
                        .build_bot(self, duration, material)
                        .filter(|s| s.could_do_better(duration, best))
                    {
                        next.push(s);
                    }
                }
            }
        }

        best as usize
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
            .map(|(i, b)| (i + 1) * b.score(24))
            .sum()
    }

    fn part_2(&self) -> Self::T2 {
        self.blueprints.iter().take(3).map(|b| b.score(32)).product()
    }
}
