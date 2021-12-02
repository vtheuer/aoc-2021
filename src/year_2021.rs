mod day01;
mod day02;

use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

pub fn get() -> Year {
    Year {
        year: 2021,
        days: days_vec!(2),
    }
}
