pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }

    let n = upper_bound as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= n {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }

    primes
}