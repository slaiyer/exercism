use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_len = word.len();
    let word_sorted = sorted(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| is_anagram(word, word_len, &word_sorted, candidate))
        .collect()
}

fn is_anagram(word: &str, word_len: usize, word_sorted: &Vec<char>, candidate: &str) -> bool {
    if word_len != candidate.len() || word == candidate || *word_sorted != sorted(candidate) {
        return false;
    }

    true
}

fn sorted(s: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = s.to_uppercase().chars().collect();
    word_sorted.sort();
    word_sorted
}
