use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (score, letters) in h {
        for &ch in letters {
            result.insert(ch.to_ascii_lowercase(), *score);
        }
    }

    result
}
