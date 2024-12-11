pub mod direction;
pub mod grid;

use std::fmt::Display;
use std::str::FromStr;
use std::vec::IntoIter;

use num::Num;

use crate::util::NumArg::{Last, Nth};

pub fn format_duration(time: u128) -> String {
    let ftime = time as f64;
    if ftime <= 1e3 {
        format!("{:.0}ns", ftime)
    } else if ftime <= 1e6 {
        format!("{:.1}µs", ftime / 1e3)
    } else if ftime <= 1e9 {
        format!("{:.1}ms", ftime / 1e6)
    } else {
        format!("{:.1} s", ftime / 1e9)
    }
}

pub enum NumArg<T: Num> {
    Nth(T),
    Last,
}

pub fn parse_arg<T>(arg_name: &str, arg: &str) -> NumArg<T>
where
    T: Num + FromStr,
{
    match arg {
        "last" => Ok(Last),
        nth => nth.parse().map(|n| Nth(n)),
    }
    .unwrap_or_else(|_| panic!("{} : expected either a number or \"last\", got {}", arg_name, arg))
}

pub trait SortableByKey<T, I> {
    fn sorted_unstable_by_key<K, F>(self, f: F) -> IntoIter<T>
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T, I> SortableByKey<T, I> for I
where
    T: Sized,
    I: Iterator<Item = T>,
{
    fn sorted_unstable_by_key<K, F>(self, f: F) -> IntoIter<T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut v = Vec::from_iter(self);
        v.sort_unstable_by_key(f);
        v.into_iter()
    }
}

pub trait Joinable {
    fn join(self, sep: &str) -> String;
}

impl<T, I> Joinable for I
where
    T: Display,
    I: Iterator<Item = T>,
{
    fn join(self, sep: &str) -> String {
        self.map(|e| e.to_string()).collect::<Vec<_>>().join(sep)
    }
}

pub trait FindIndex<I> {
    fn find_index_by<P>(self, predicate: P) -> Option<(usize, I)>
    where
        P: FnMut(&I) -> bool;

    fn rfind_index_by<P>(self, predicate: P) -> Option<(usize, I)>
    where
        P: FnMut(&I) -> bool,
        Self: DoubleEndedIterator + ExactSizeIterator;

    fn find_index(self, value: I) -> Option<usize>
    where
        Self: Sized,
        I: PartialEq,
    {
        self.find_index_by(|v| *v == value).map(|(i, _)| i)
    }
}
impl<I, S> FindIndex<I> for S
where
    S: Iterator<Item = I>,
{
    fn find_index_by<P>(self, mut predicate: P) -> Option<(usize, I)>
    where
        P: FnMut(&I) -> bool,
    {
        self.enumerate().find(|(_, e)| predicate(e))
    }

    fn rfind_index_by<P>(self, mut predicate: P) -> Option<(usize, I)>
    where
        P: FnMut(&I) -> bool,
        Self: DoubleEndedIterator + ExactSizeIterator,
    {
        self.enumerate().rev().find(|(_, e)| predicate(e))
    }
}

pub fn first_line(s: &str) -> &str {
    s.lines().next().unwrap()
}

pub fn count_digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
