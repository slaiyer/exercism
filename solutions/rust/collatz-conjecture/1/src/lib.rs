pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut num_steps = 0_u64;
    let mut n = n;

    while n != 1 {
        n = match n % 2 {
            0 => n / 2,
            _ => (3 * n) + 1,
        };

        num_steps += 1
    }

    Some(num_steps)
}
