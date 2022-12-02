use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

mod day01;
mod day02;

pub fn get() -> Year {
    Year {
        year: 2022,
        days: days_vec!(2),
    }
}
