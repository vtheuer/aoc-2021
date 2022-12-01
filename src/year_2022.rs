use macros::days_vec;

use crate::day::Day;
use crate::year::Year;

mod day01;

pub fn get() -> Year {
    Year {
        year: 2022,
        days: days_vec!(1),
    }
}
