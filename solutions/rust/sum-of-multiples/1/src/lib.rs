use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&factor| {
            (1..)
                .map(move |i| i * factor)
                .take_while(move |&m| m < limit)
        })
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
