use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let word_len = word.len();
    let word_norm = word.to_uppercase();
    let word_freq = letter_frequencies(&word_norm);

    candidates
        .iter()
        .filter(|candidate| word_len == candidate.len())
        .filter(|candidate| {
            let candidate_norm = candidate.to_uppercase();
            word_norm != candidate_norm && word_freq == letter_frequencies(&candidate_norm)
        })
        .cloned()
        .collect()
}

fn letter_frequencies(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut freq, c| {
        freq.entry(c).and_modify(|n| *n += 1).or_insert(1);
        freq
    })
}
