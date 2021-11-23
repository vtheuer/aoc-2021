use std::time::Instant;

use crate::util::format_duration;
use colored::*;
use itertools::Itertools;
use std::fmt::Display;

fn time<T, F: Fn() -> T>(f: F) -> (T, u128) {
    let begin = Instant::now();
    (f(), begin.elapsed().as_nanos())
}

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn format_row(k: &str, v: &str, left: usize, right: usize, header: bool) -> String {
    format!(
        "│ {}{} │ {}{} │",
        if header {
            k.bold().bright_blue()
        } else {
            ColoredString::from(k)
        },
        " ".repeat(left - char_count(k)),
        " ".repeat(right - char_count(v)),
        if header {
            v.bold()
        } else {
            ColoredString::from(v)
        }
    )
}

fn print_table((hk, hv): (&str, &str), rows: &Vec<(&str, &str)>) {
    let left = 24.max(rows.iter().map(|(k, _)| char_count(k)).max().unwrap());
    let right = 7.max(rows.iter().map(|(_, v)| char_count(v)).max().unwrap());

    println!(
        "┌{}┬{}┐\n{}\n├{0}┼{1}┤\n{}\n└{0}┴{1}┘",
        "─".repeat(left + 2),
        "─".repeat(right + 2),
        format_row(&hk, &hv, left, right, true),
        rows.iter()
            .map(|(k, v)| format_row(k, v, left, right, false))
            .join("\n")
    );
}

pub trait Day<'a>: Sized {
    type T1: Display;
    type T2: Display;

    fn new(input: &'a str) -> Self;
    fn part_1(&self) -> Self::T1;
    fn part_2(&self) -> Self::T2;

    fn run(n: u8, input: &'a str) -> u128 {
        let (day, parse_duration) = time(|| Self::new(input));

        let (output_1, part_1_duration) = time(|| day.part_1());
        let (output_2, part_2_duration) = time(|| day.part_2());

        let total = parse_duration + part_1_duration + part_2_duration;

        print_table(
            (&format!("Day {}", n), &format_duration(total)),
            &vec![
                ("Parse  :", &format_duration(parse_duration)),
                (
                    &format!("Part 1 : {}", output_1),
                    &format_duration(part_1_duration),
                ),
                (
                    &format!("Part 2 : {}", output_2),
                    &format_duration(part_2_duration),
                ),
            ],
        );

        total
    }
}
