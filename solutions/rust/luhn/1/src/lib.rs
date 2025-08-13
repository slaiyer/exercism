use std::collections::HashMap;

/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");

    if code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    if code.len() == 1 && code.starts_with('0') {
        return false;
    }

    let subst: HashMap<usize, usize> = HashMap::from_iter(vec![
        (0, 0),
        (1, 2),
        (2, 4),
        (3, 6),
        (4, 8),
        (5, 1),
        (6, 3),
        (7, 5),
        (8, 7),
        (9, 9),
    ]);

    let sum = code
        .bytes()
        .rev()
        .enumerate()
        .map(|(i, b)| {
            let n = (b - b'0') as usize;

            match i % 2 {
                0 => n,
                _ => subst[&n],
            }
        })
        .sum::<usize>();

    sum % 10 == 0
}
