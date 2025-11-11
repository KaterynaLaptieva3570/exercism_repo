pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;

    // keep checking numbers until we find the nth prime
    loop {
        if is_prime(num) {
            if count == n {
                return num;
            } else {
                count += 1;
            }
        }
        num += 1;
    }
}

fn is_prime(num: u32) -> bool {
    // 0 and 1 are not prime
    if num < 2 {
        return false;
    }

    // check all numbers from 2 up to num - 1
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }

    // if no divisors found, it's prime
    true
}
