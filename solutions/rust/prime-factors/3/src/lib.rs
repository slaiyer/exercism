pub fn factors(n: u64) -> Vec<u64> {
    match n {
        0..=1 => Vec::new(),
        _ => find(n, 2),
    }
}

fn find(num: u64, div: u64) -> Vec<u64> {
    if div * div > num {
        return vec![num];
    }

    match num % div {
        0 => std::iter::once(div).chain(find(num / div, div)).collect(),
        _ => find(num, if div == 2 { 3 } else { div + 2 }),
    }
}
