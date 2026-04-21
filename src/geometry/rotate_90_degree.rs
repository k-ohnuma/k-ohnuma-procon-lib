pub fn rotate_90_degrees<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - i - 1] = matrix[i][j].clone();
        }
    }

    rotated
}
