#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (_m, _n) if first_list == second_list => Comparison::Equal,
        (_m, _n) if contains_list(first_list, second_list) => Comparison::Superlist,
        (_m, _n) if contains_list(second_list, first_list) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn contains_list(a: &[i32], b: &[i32]) -> bool {
    a.windows(b.len()).any(|a_win| a_win == b)
}
