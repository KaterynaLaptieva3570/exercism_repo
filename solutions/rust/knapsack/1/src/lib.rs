#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mw = max_weight as usize;
    let mut dp = vec![0u32; mw + 1];

    for it in items {
        let w = it.weight as usize;
        let v = it.value;
        // backwards 
        for cap in (w..=mw).rev() {
            dp[cap] = dp[cap].max(dp[cap - w] + v);
        }
    }

    dp[mw]
}
