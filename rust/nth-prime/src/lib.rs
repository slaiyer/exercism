pub fn nth(n: u32) -> u32 {
    let n = (n + 1) as usize;
    let mut primes = Vec::with_capacity(n);

    let mut x = 2;
    while primes.len() < n {
        if is_prime(&primes, x) {
            primes.push(x);
        }

        x += 1;
    }

    *primes.last().unwrap()
}

fn is_prime(primes: &[u32], x: u32) -> bool {
    let p_max = (x as f32).sqrt() as u32;
    !primes.iter().filter(|p| **p <= p_max).any(|p| x % p == 0)
}
