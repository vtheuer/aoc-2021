use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

pub fn get() -> Year {
    Year {
        year: 2022,
        days: days_vec!(17),
    }
}
