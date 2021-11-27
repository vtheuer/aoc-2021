#![allow(unused_imports, dead_code)]

extern crate macros;

use std::env;

use colored::*;
use fnv::FnvHashMap;

use macros::days_vec;

use crate::day::Day;
use crate::util::format_duration;
use crate::year::Year;

mod day;
mod util;
mod year;
mod year_2020;
mod year_2021;

fn make_years() -> FnvHashMap<u16, Year> {
    let mut years = FnvHashMap::default();
    years.insert(2020, year_2020::get());
    years.insert(2021, year_2021::get());
    years
}

fn main() {
    let years = make_years();
    let get_year = |y| {
        let i = match y {
            "last" => *years.keys().max().unwrap(),
            n => n
                .parse()
                .unwrap_or_else(|_| panic!("year : expected either a number or \"last\", got {}", n)),
        };
        years.get(&i).unwrap_or_else(|| panic!("year {} not found", y))
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
                year.run()
            }
        }
        _ => panic!("Usage: aoc [YEAR] [DAY]"),
    };
}
