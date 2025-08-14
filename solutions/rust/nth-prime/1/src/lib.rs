pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    let mut x = 2;
    while primes.len() <= n as usize {
        if is_prime(&primes, x) {
            primes.push(x);
        }

        x += 1;
    }

    *primes.last().unwrap()
}

fn is_prime(primes: &[u32], n: u32) -> bool {
    !primes.iter().any(|p| n % p == 0)
}
