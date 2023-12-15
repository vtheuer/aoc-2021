extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

#[proc_macro]
pub fn year(token_stream: TokenStream) -> TokenStream {
    let tokens = token_stream.into_iter().map(|t| t.to_string()).collect::<Vec<_>>();
    assert_eq!(tokens.len(), 1, "run_day requires a year number");
    let year = tokens
        .get(0)
        .unwrap()
        .parse::<usize>()
        .expect("day count must be a number literal");

    let year_path = format!("./src/year_{}.rs", year);
    let implemented_days = read_to_string(Path::new(&env::current_dir().unwrap()).join(&year_path))
        .unwrap_or_else(|_| panic!("{} not found", year_path))
        .lines()
        .filter(|l| l.starts_with("mod day"))
        .map(|l| {
            l.chars()
                .skip("mod day".len())
                .take_while(|&c| c != ';')
                .collect::<String>()
                .parse::<u8>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    format!(
        "Year {{ year: {}, days: [{}] }}",
        year,
        (1..=25u8)
            .map(|n| Some(n).filter(|_| implemented_days.contains(&n)))
            .map(|day| day
                .map(|n| format!("Some(|title, input| {{day{0:02}::Day{0:02}::run(title, input)}}),", n))
                .unwrap_or_else(|| "None,".to_string()))
            .collect::<String>()
    )
    .parse()
    .unwrap()
}
