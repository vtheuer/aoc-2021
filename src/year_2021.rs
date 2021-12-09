mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

pub fn get() -> Year {
    Year {
        year: 2021,
        days: days_vec!(9),
    }
}
