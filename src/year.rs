use colored::*;

use crate::client::Client;
use crate::util::NumArg::{Last, Nth};
use crate::util::{format_duration, NumArg};

type RunDay = for<'r> fn(&'r str, &'r str) -> u128;

pub struct Year {
    pub year: u16,
    pub days: [Option<RunDay>; 25],
}

impl Year {
    pub fn run(&self, client: &Client) {
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
                        .map(|(n, day)| day.map(|d| self.run_day((n + 1, d), client)).unwrap_or(0))
                        .sum()
                )
            )
            .bold()
            .cyan()
        )
    }

    pub fn run_day_by_number(&self, d: NumArg<usize>, client: &Client) -> u128 {
        self.run_day(
            match d {
                Nth(n) => (n, self.days[n - 1].unwrap_or_else(|| panic!("day {} not found", n))),
                Last => self
                    .days
                    .iter()
                    .enumerate()
                    .filter_map(|(i, day)| day.map(|d| (i + 1, d)))
                    .last()
                    .unwrap(),
            },
            client,
        )
    }

    fn run_day(&self, (n, day): (usize, RunDay), client: &Client) -> u128 {
        let input = client.get_input(self.year, n as u8).unwrap();
        let title = client.get_title(self.year, n as u8).unwrap();
        day(&format!("{} Day {}: {}", self.year, n, title), &input)
    }
}
