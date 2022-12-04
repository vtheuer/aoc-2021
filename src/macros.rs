extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn days_vec(token_stream: TokenStream) -> TokenStream {
    let tokens = token_stream.into_iter().map(|t| t.to_string()).collect::<Vec<_>>();
    assert_eq!(tokens.len(), 1, "run_day requires a day count");
    let day_count = tokens
        .get(0)
        .unwrap()
        .parse::<u8>()
        .expect("day count must be a u8 literal");

    format!(
        "vec![{}]",
        (1..=day_count)
            .map(|n| format!("|y, i| {{day{0:02}::Day{0:02}::run(y, {0}, i)}},", n))
            .collect::<String>(),
    )
    .parse()
    .unwrap()
}
