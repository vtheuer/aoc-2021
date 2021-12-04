use std::fmt::Display;
use std::str::FromStr;
use std::vec::IntoIter;

pub fn split_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, p);
    Some((s.next()?, s.next()?))
}

pub fn rsplit_pair<'a>(input: &'a str, p: &str) -> Option<(&'a str, &'a str)> {
    let mut s = input.rsplitn(2, p);
    let (a, b) = (s.next()?, s.next()?);
    Some((b, a))
}

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

pub fn parse_arg<T: FromStr, F: FnOnce() -> T>(arg_name: &str, arg: &str, get_last: F) -> T {
    match arg {
        "last" => Ok(get_last()),
        n => n.parse(),
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
