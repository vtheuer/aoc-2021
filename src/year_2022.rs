use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn get() -> Year {
    Year {
        year: 2022,
        days: days_vec!(4),
    }
}
