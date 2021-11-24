#![allow(unused_imports, dead_code)]

extern crate macros;

use std::env;
use std::error::Error;
use std::fs::{read_dir, read_to_string, write};
use std::path::Path;

use crate::day::Day;
use crate::util::format_duration;
use colored::*;
use macros::days_vec;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;

mod day;
mod util;

fn first_line(s: &String) -> &str {
    s.lines().next().unwrap()
}

fn get_input(n: u8) -> anyhow::Result<String> {
    let input_file = format!("inputs/{:02}.txt", n);
    if Path::new(&input_file).exists() {
        Ok(read_to_string(input_file)?)
    } else {
        println!("Fetching input for day {}...", n);
        let input = Client::new()
            .get(format!("https://adventofcode.com/2021/day/{}/input", n))
            .header(
                COOKIE,
                format!(
                    "session={}",
                    first_line(
                        &read_to_string(".session").expect("please provide a session token in a file named .session")
                    )
                ),
            )
            .send()?
            .text()?;
        assert_ne!(
            first_line(&input),
            "Puzzle inputs differ by user.  Please log in to get your puzzle input.",
            "session has expired"
        );
        write(input_file, &input)?;
        Ok(input)
    }
}

fn run_day(days: &[fn(&str) -> u128], n: u8) -> u128 {
    assert!(n <= days.len() as u8, "day {} not found", n);
    days[n as usize - 1](&get_input(n).unwrap())
}

fn day_from_input() -> Option<u8> {
    read_dir("inputs")
        .ok()?
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter_map(|f| f[..f.find('.')?].parse().ok())
        .max()
}

fn main() {
    let arg = env::args().skip(1).next();

    let days: Vec<for<'r> fn(&'r str) -> u128> = days_vec!(0);

    if arg == Some(String::from("-a")) {
        println!(
            "\n{}",
            &format!(
                "Total run time: {}",
                format_duration((1..=days.len() as u8).map(|n| run_day(&days, n)).sum::<u128>())
            )
            .bold()
            .cyan()
        );
    } else {
        run_day(
            &days,
            arg.map(|n| n.parse().expect("Could not read day number"))
                .or_else(day_from_input)
                .expect("No input file found"),
        );
    }
}