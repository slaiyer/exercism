/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");

    if code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    if code.len() == 1 && code.starts_with('0') {
        return false;
    }

    let subst = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

    let sum = code
        .bytes()
        .rev()
        .enumerate()
        .map(|(i, b)| {
            let n = (b - b'0') as usize;

            match i % 2 {
                0 => n,
                _ => subst[n],
            }
        })
        .sum::<usize>();

    sum % 10 == 0
}
