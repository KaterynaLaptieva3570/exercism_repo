pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let n = size as usize;
    let mut matrix = vec![vec![0u32; n]; n];
    let mut num = 1u32;
    let mut top = 0usize;
    let mut bottom = n.saturating_sub(1);
    let mut left = 0usize;
    let mut right = n.saturating_sub(1);

    while top <= bottom && left <= right {
        // Top row
        for col in left..=right {
            matrix[top][col] = num;
            num += 1;
        }
        top += 1;

        // Right column
        for row in top..=bottom {
            matrix[row][right] = num;
            num += 1;
        }
        if right > 0 { right -= 1; }

        // Bottom row (only if there's still a row left)
        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom][col] = num;
                num += 1;
            }
            if bottom > 0 { bottom -= 1; }
        }

        // Left column (only if there's still a column left)
        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}