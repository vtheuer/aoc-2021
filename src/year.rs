use std::env;
use std::error::Error;
use std::fs::{create_dir_all, read_dir, read_to_string, write};
use std::path::Path;

use crate::day::Day;
use crate::util::format_duration;
use colored::*;
use macros::days_vec;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;

fn first_line(s: &String) -> &str {
    s.lines().next().unwrap()
}

pub struct Year {
    pub year: u16,
    pub days: Vec<for<'r> fn(&'r str) -> u128>,
}

impl Year {
    pub fn run(&self) {
        println!(
            "\n{}",
            &format!(
                "Total run time for {}: {}",
                self.year,
                format_duration(
                    (1..=self.days.len() as u8)
                        .map(|n| self.run_day_by_number(n as usize))
                        .sum()
                )
            )
            .bold()
            .cyan()
        )
    }

    pub fn run_day(&self, d: &str) -> u128 {
        let i = match d {
            "last" => self.days.len(),
            n => {
                let i = n
                    .parse()
                    .unwrap_or_else(|_| panic!("day : expected either a number or \"last\", got {}", n));
                assert!(i <= self.days.len(), "day {} not found", n);
                i
            }
        };
        self.run_day_by_number(i)
    }

    fn run_day_by_number(&self, n: usize) -> u128 {
        self.days[n - 1](&self.get_input(n).unwrap())
    }

    fn get_input(&self, n: usize) -> anyhow::Result<String> {
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
