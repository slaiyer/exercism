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

    if contains_list(first_list, second_list) {
        return Comparison::Sublist;
    }

    if contains_list(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn contains_list(a: &[i32], b: &[i32]) -> bool {
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
        if a[start + pos..start + pos + b.len()] == *b {
            return true;
        }
    }

    false
}
