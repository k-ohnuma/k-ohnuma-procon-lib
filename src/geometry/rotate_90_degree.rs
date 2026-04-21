pub fn rotate_90_degrees<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];

    for (i, row) in matrix.iter().enumerate().take(rows) {
        for (j, out_row) in rotated.iter_mut().enumerate().take(cols) {
            out_row[rows - i - 1] = row[j].clone();
        }
    }

    rotated
}
