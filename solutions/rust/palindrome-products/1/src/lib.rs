use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut map: HashMap<u64, HashSet<(u64, u64)>> = HashMap::new();

    for a in min..=max {
        for b in a..=max {
            let prod = a * b;
            if is_palindrome(prod) {
                map.entry(prod)
                    .or_insert_with(HashSet::new)
                    .insert((a, b));
            }
        }
    }

    if map.is_empty() {
        return None;
    }

    let min_val = *map.keys().min().unwrap();
    let max_val = *map.keys().max().unwrap();

    let min_pal = Palindrome {
        value: min_val,
        factors: map.remove(&min_val).unwrap(),
    };

    let max_pal = Palindrome {
        value: max_val,
        factors: map.remove(&max_val).unwrap(),
    };

    Some((min_pal, max_pal))
}
