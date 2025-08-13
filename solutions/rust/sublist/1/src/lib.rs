#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if contains_sequence(first_list, second_list) {
        return Comparison::Sublist;
    }

    if contains_sequence(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn contains_sequence(a: &[i32], b: &[i32]) -> bool {
    if b.is_empty() {
        return true;
    }

    if b.len() >= a.len() {
        return false;
    }

    for start in 0..=a.len() - b.len() {
        let Some(pos) = a.iter().skip(start).position(|x| x == &b[0]) else {
            continue;
        };
        if a[pos..pos + b.len()] == *b {
            return true;
        }
    }

    false
}
