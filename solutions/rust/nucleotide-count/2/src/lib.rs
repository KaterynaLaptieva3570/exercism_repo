use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }

    let mut counts = nucleotide_counts(dna)?;
    Ok(*counts.get(&nucleotide).unwrap())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // лічильник з усіма 0
    let mut map = HashMap::from([
        ('A', 0),
        ('C', 0),
        ('G', 0),
        ('T', 0),
    ]);

    for ch in dna.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => *map.get_mut(&ch).unwrap() += 1,
            invalid => return Err(invalid),
        }
    }

    Ok(map)
}
