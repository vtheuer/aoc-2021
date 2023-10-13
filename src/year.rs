use std::env;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

use colored::*;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};

use crate::day::Day;
use crate::util::NumArg::{Last, Nth};
use crate::util::{format_duration, NumArg};

const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap()
}

type RunDay = for<'r> fn(u16, &'r str) -> u128;

pub struct Year {
    pub year: u16,
    pub days: [Option<RunDay>; 25],
}

impl Year {
    pub fn run(&self) {
        println!(
            "\n{}",
            &format!(
                "Total run time for {} ({}/25): {}",
                self.year,
                self.days.iter().filter(|day| day.is_some()).count(),
                format_duration(
                    self.days
                        .iter()
                        .enumerate()
                        .map(|(n, day)| day.map(|d| self.run_day((1 + n as u8, d))).unwrap_or(0))
                        .sum()
                )
            )
            .bold()
            .cyan()
        )
    }

    pub fn run_day_by_number(&self, d: NumArg<u8>) -> u128 {
        self.run_day(match d {
            Nth(n) => (
                n,
                self.days[n as usize].unwrap_or_else(|| panic!("day {} not found", n)),
            ),
            Last => self
                .days
                .iter()
                .enumerate()
                .filter_map(|(n, day)| day.map(|d| (1 + n as u8, d)))
                .last()
                .unwrap(),
        })
    }

    fn run_day(&self, (n, day): (u8, RunDay)) -> u128 {
        day(self.year, &self.get_input(n).unwrap())
    }

    fn get_input(&self, n: u8) -> anyhow::Result<String> {
        let dir = format!("inputs/{}", self.year);
        create_dir_all(&dir).unwrap_or_else(|_| panic!("could not create directory {}", &dir));

        let input_file = format!("{}/{:02}.txt", dir, n);

        if Path::new(&input_file).exists() {
            Ok(read_to_string(input_file)?)
        } else {
            println!("Fetching input for day {}...", n);
            let input = Client::new()
                .get(format!("https://adventofcode.com/{}/day/{}/input", self.year, n))
                .header(
                    COOKIE,
                    format!(
                        "session={}",
                        first_line(
                            &read_to_string(".session")
                                .expect("please provide a session token in a file named .session")
                        )
                    ),
                )
                .header(USER_AGENT, format!("my own rust runner by {}", AUTHOR))
                .send()?
                .text()?;
            assert_ne!(
                first_line(&input),
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.",
                "session has expired"
            );
            write(input_file, &input)?;
            Ok(input)
        }
    }
}
