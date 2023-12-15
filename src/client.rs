use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;
use std::time::Duration;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

use crate::util::first_line;

const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const TITLES_FILE: &str = ".titles.json";

pub struct Client {
    client: reqwest::blocking::Client,
    title_regex: Regex,
    titles: RefCell<HashMap<u16, HashMap<u8, String>>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .tcp_keepalive(Duration::from_secs(10))
                .user_agent(format!("my own rust runner by {}", AUTHOR))
                .default_headers(
                    read_to_string(".session")
                        .map(|session| format!("session={}", session.trim_end()))
                        .map(|c| HeaderValue::from_str(&c).unwrap())
                        .map(|header_value| HeaderMap::from_iter([(COOKIE, header_value)]))
                        .expect("please provide a session token in a file named .session"),
                )
                .build()
                .unwrap(),
            title_regex: Regex::new("<h2>--- Day \\d+: (.+) ---</h2>").unwrap(),
            titles: RefCell::new(
                Some(TITLES_FILE)
                    .map(Path::new)
                    .filter(|path| path.exists())
                    .map(|path| read_to_string(path).expect("Could not read .titles.json"))
                    .map(|json| serde_json::from_str(&json).expect(".titles.json does not contain valid json"))
                    .unwrap_or_default(),
            ),
        }
    }

    fn fetch(&self, url: &str) -> anyhow::Result<String> {
        Ok(self.client.get(url).send()?.text()?)
    }

    pub fn get_input(&self, year: u16, day: u8) -> anyhow::Result<String> {
        let dir = format!("inputs/{}", year);
        create_dir_all(&dir).unwrap_or_else(|_| panic!("could not create directory {}", &dir));

        let input_file = format!("{}/{:02}.txt", dir, day);

        if Path::new(&input_file).exists() {
            Ok(read_to_string(input_file)?)
        } else {
            println!("Fetching input for day {}...", day);
            let input = self.fetch(&format!("https://adventofcode.com/{}/day/{}/input", year, day))?;
            assert_ne!(
                first_line(&input),
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.",
                "session has expired"
            );
            write(input_file, &input)?;
            Ok(input)
        }
    }

    pub fn get_title(&self, year: u16, day: u8) -> anyhow::Result<String> {
        let mut titles = self.titles.borrow_mut();
        if let Some(title) = titles.get(&year).and_then(|days| days.get(&day)) {
            Ok(title.clone())
        } else {
            println!("Fetching title for day {}...", day);
            let html = self.fetch(&format!("https://adventofcode.com/{}/day/{}", year, day))?;
            let title = self
                .title_regex
                .captures(&html)
                .expect("couldn't find day title in html")
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            (*titles).entry(year).or_default().insert(day, title.clone());
            write(TITLES_FILE, serde_json::to_string(&(*titles))?)?;
            Ok(title)
        }
    }
}
