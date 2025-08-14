pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    find(n, 2)
}
fn find(num: u64, div: u64) -> Vec<u64> {
    if div * div > num {
        return vec![num];
    }

    if num % div == 0 {
        std::iter::once(div).chain(find(num / div, div)).collect()
    } else {
        let next_div = if div == 2 { 3 } else { div + 2 };
        find(num, next_div)
    }
}
