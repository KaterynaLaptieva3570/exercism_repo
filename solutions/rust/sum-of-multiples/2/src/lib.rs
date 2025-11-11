pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    for i in 0..factors.len() {
        let factor = factors[i];
        if factor == 0 {
            continue;
        }

        let mut multiple = factor;
        while multiple < limit {
            if !numbers.contains(&multiple) {
                numbers.push(multiple);
            }
            multiple = multiple + factor;
        }
    }

    let mut sum = 0;
    for i in 0..numbers.len() {
        sum = sum + numbers[i];
    }

    return sum;
}
