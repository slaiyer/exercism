use itertools::Itertools;

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .tuple_windows()
        .flat_map(|(a, b)| {
            if a.is_ascii_lowercase() && b.is_ascii_uppercase() {
                vec![a, ' ']
            } else {
                vec![a]
            }
        })
        .chain(phrase.chars().last())
        .collect::<String>()
        .replace('-', " ")
        .split_ascii_whitespace()
        .filter_map(|w| w.chars().find(|c| c.is_ascii_alphabetic()))
        .collect::<String>()
        .to_ascii_uppercase()
}
