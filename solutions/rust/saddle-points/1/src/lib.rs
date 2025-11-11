pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    // Handle empty matrix
    if input.is_empty() || input[0].is_empty() {
        return saddle_points;
    }

 

    let rows = input.len();
    let cols = input[0].len();

    // Precompute max in each row and min in each column
    let row_max: Vec<u64> = input
        .iter()
        .map(|row| *row.iter().max().unwrap())
        .collect();

    let col_min: Vec<u64> = (0..cols)
        .map(|j| (0..rows).map(|i| input[i][j]).min().unwrap())
        .collect();

    // Check each cell: is it the max in its row AND min in its column?
    for i in 0..rows {
        for j in 0..cols {
            let value = input[i][j];
            if value == row_max[i] && value == col_min[j] {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}

