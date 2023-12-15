#![allow(unused_imports, dead_code)]

extern crate macros;

use std::env;
use std::str::FromStr;

use colored::*;
use fnv::FnvHashMap;

use util::parse_arg;
use year_2020::YEAR_2020;
use year_2021::YEAR_2021;
use year_2022::YEAR_2022;
use year_2023::YEAR_2023;

use crate::client::Client;
use crate::day::Day;
use crate::util::format_duration;
use crate::util::NumArg::{Last, Nth};
use crate::year::Year;

mod client;
mod day;
mod util;
mod year;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;

fn main() {
    let years = vec![YEAR_2020, YEAR_2021, YEAR_2022, YEAR_2023];
    let get_year = |y| match parse_arg::<u16>("year", y) {
        Nth(nth) => years
            .iter()
            .find(|y| y.year == nth)
            .unwrap_or_else(|| panic!("year {} not found", y)),
        Last => years.iter().last().unwrap(),
    };
    let client = Client::new();

    match &env::args().skip(1).collect::<Vec<_>>()[..] {
        [y, d] => {
            get_year(y).run_day_by_number(parse_arg("day", d), &client);
        }
        [y] => {
            get_year(y).run(&client);
        }
        [] => {
            let duration = years.into_iter().map(|year| year.run(&client)).sum::<u128>();
            println!(
                "\n{}",
                &format!("Total run time for all years: {}", format_duration(duration))
                    .bold()
                    .cyan()
            );
        }
        _ => panic!("Usage: aoc [YEAR] [DAY]"),
    };
}
