use std::collections::{HashMap, HashSet};

pub fn brackets_are_balanced(string: &str) -> bool {
    let pairs = vec![(']', '['), ('}', '{'), (')', '(')];
    let map: HashMap<char, char> = pairs.clone().into_iter().collect();
    let mut stack: Vec<char> = Vec::new();
    let brackets: HashSet<char> = pairs.into_iter().flat_map(|(a, b)| [a, b]).collect();

    for c in string.chars().filter(|c| brackets.contains(c)) {
        if map.contains_key(&c) {
            match stack.pop() {
                None => return false,
                Some(b) => {
                    if b != map[&c] {
                        return false;
                    }
                }
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}
