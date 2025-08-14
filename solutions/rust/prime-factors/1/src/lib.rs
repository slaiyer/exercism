#[allow(clippy::mut_range_bound)]
pub fn factors(n: u64) -> Vec<u64> {
    let mut n_div = n;
    let (mut primes, mut prime_factors) = (Vec::new(), Vec::new());
    let (mut p_min, mut p_max) = (2, (n_div as f64).sqrt() as u64);

    while p_min <= p_max {
        for x in p_min..=p_max {
            if !is_prime(&primes, x) {
                continue;
            }
            primes.push(x);

            while n_div % x == 0 {
                n_div /= x;
                prime_factors.push(x);
            }

            (p_min, p_max) = (x + 1, (n_div as f64).sqrt() as u64);
        }
    }

    if n_div > 1 {
        prime_factors.push(n_div);
    }

    prime_factors
}

fn is_prime(primes: &[u64], x: u64) -> bool {
    let p_max = (x as f64).sqrt() as u64;
    !primes.iter().filter(|p| **p <= p_max).any(|p| x % p == 0)
}
