use crate::day::Day;
use crate::util::split_pair;
use fnv::FnvHashSet;

pub struct Day04<'a> {
    passports: Vec<Vec<(&'a str, &'a str)>>,
}

impl<'a> Day<'a> for Day04<'a> {
    type T1 = usize;
    type T2 = usize;

    fn new(input: &'a str) -> Self {
        Day04 {
            passports: input
                .split("\n\n")
                .map(|group| {
                    group
                        .split(char::is_whitespace)
                        .filter_map(|pair| split_pair(pair, ":"))
                        .collect::<Vec<_>>()
                })
                .filter(|pairs| pairs.len() >= pairs.iter().find(|(k, _)| *k == "cid").and(Some(8)).unwrap_or(7))
                .collect(),
        }
    }

    fn part_1(&self) -> Self::T1 {
        self.passports.len()
    }

    fn part_2(&self) -> Self::T2 {
        let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .collect::<FnvHashSet<_>>();
        self.passports
            .iter()
            .filter(|pairs| {
                pairs.iter().all(|(k, v)| match *k {
                    "byr" => v.parse::<u32>().map(|y| (1920..=2002).contains(&y)).unwrap_or(false),
                    "iyr" => v.parse::<u32>().map(|y| (2010..=2020).contains(&y)).unwrap_or(false),
                    "eyr" => v.parse::<u32>().map(|y| (2020..=2030).contains(&y)).unwrap_or(false),
                    "hgt" => Some(v)
                        .map(|s| (&s[..s.len() - 2], &s[s.len() - 2..]))
                        .and_then(|(h, u)| Some((h.parse::<u32>().ok()?, u)))
                        .filter(|&(h, u)| match u {
                            "cm" => (150..=193).contains(&h),
                            "in" => (59..=76).contains(&h),
                            _ => false,
                        })
                        .is_some(),
                    "hcl" => v.len() == 7 && v.starts_with('#') && v.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
                    "ecl" => eye_colors.contains(v),
                    "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
                    "cid" => true,
                    _ => unreachable!("unknown key {}", k),
                })
            })
            .count()
    }
}
