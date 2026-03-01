pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line|
                line.split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            )
            .collect();
        Self { rows }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no.checked_sub(1)?).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let c = col_no.checked_sub(1)?;

        if self.rows.iter().any(|r| c >= r.len()) {
            return None;
        }
        Some(self.rows.iter().map(|r| r[c]).collect())
    }
}
