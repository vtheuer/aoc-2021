#![allow(unused_imports, dead_code)]

extern crate macros;

use std::env;
use std::str::FromStr;

use colored::*;
use fnv::FnvHashMap;

use macros::days_vec;
use util::parse_arg;

use crate::day::Day;
use crate::util::format_duration;
use crate::year::Year;

mod day;
mod util;
mod year;
mod year_2020;
mod year_2021;
mod year_2022;

fn main() {
    let years: Vec<(u16, fn() -> Year)> = vec![(2020, year_2020::get), (2021, year_2021::get), (2022, year_2022::get)];
    let get_year = |y| {
        let year_number = parse_arg("year", y, || years.iter().last().unwrap().0);
        years
            .iter()
            .find(|(y, _)| *y == year_number)
            .unwrap_or_else(|| panic!("year {} not found", y))
            .1()
    };

    match &env::args().skip(1).collect::<Vec<_>>()[..] {
        [y, d] => {
            get_year(y).run_day(d);
        }
        [y] => {
            get_year(y).run();
        }
        [] => {
            for (_, year) in years {
                year().run()
            }
        }
        _ => panic!("Usage: aoc [YEAR] [DAY]"),
    };
}
