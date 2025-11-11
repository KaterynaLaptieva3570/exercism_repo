pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let n = self.row_count as usize;
        if n == 0 {
            return Vec::new();
        }

        let mut triangle: Vec<Vec<u32>> = Vec::with_capacity(n);
        triangle.push(vec![1]); // перший рядок

        for i in 1..n {
            let prev = &triangle[i - 1];
            let mut row = Vec::with_capacity(i + 1);

            for j in 0..=i {
                let left = if j > 0 { prev[j - 1] } else { 0 };
                let right = if j < prev.len() { prev[j] } else { 0 };
                row.push(left + right);
            }

            triangle.push(row);
        }

        triangle
    }
}
