use std::collections::HashMap;
use std::{collections::HashSet, hash::Hash};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let count_word = HashMultiSet::new(word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| is_anagram(word, &count_word, candidate))
        .collect()
}

fn is_anagram(a: &str, count_a: &HashMultiSet<u8, usize>, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let count_b = HashMultiSet::new(b);

    *count_a == count_b
}

struct HashMultiSet<K: Eq + Hash, V> {
    map: HashMap<K, V>,
}

impl HashMultiSet<u8, usize> {
    fn new(s: &str) -> Self {
        let mut map = HashMap::new();

        for b in s.bytes() {
            map.entry(b).and_modify(|count| *count += 1).or_default();
        }

        Self { map }
    }
}

impl PartialEq for HashMultiSet<u8, usize> {
    fn eq(&self, other: &Self) -> bool {
        if self.map.len() != other.map.len() || !self.map.keys().all(|k| other.map.contains_key(k))
        {
            return false;
        }

        for (k, v) in self.map.iter() {
            if other.map[k] != *v {
                return false;
            }
        }

        true
    }
}
