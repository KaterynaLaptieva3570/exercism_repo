#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !matches!(c, 'A' | 'C' | 'G' | 'T') {
                return Err(i);
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let rna_str: String = self
            .0
            .chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!("Dna::new ensures only G,C,T,A are present"),
            })
            .collect();
        Rna(rna_str)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !matches!(c, 'A' | 'C' | 'G' | 'U') {
                return Err(i);
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
