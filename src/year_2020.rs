use crate::day::Day;
use crate::year::Year;
use macros::days_vec;

mod day01;

pub fn get() -> Year {
    Year {
        year: 2020,
        days: days_vec!(1),
    }
}
