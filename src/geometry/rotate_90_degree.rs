pub fn rotate_90_degrees<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    if cols == 0 {
        return Vec::new();
    }

    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];

    for (i, row) in matrix.iter().enumerate().take(rows) {
        for (j, out_row) in rotated.iter_mut().enumerate().take(cols) {
            out_row[rows - i - 1] = row[j].clone();
        }
    }

    rotated
}

#[cfg(test)]
mod tests {
    use super::rotate_90_degrees;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn test_empty() {
        let m: Vec<Vec<i32>> = vec![];
        assert!(rotate_90_degrees(m).is_empty());
    }

    #[test]
    fn test_zero_cols_nonempty_rows() {
        let m: Vec<Vec<i32>> = vec![vec![], vec![]];
        assert!(rotate_90_degrees(m).is_empty());
    }

    #[test]
    fn test_rectangular_manual() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let got = rotate_90_degrees(m);
        assert_eq!(got, vec![vec![4, 1], vec![5, 2], vec![6, 3]]);
    }

    #[test]
    fn random_four_rotations_is_identity_for_cols_positive() {
        let mut rng = StdRng::seed_from_u64(20260421);
        for _ in 0..5000 {
            let rows = rng.random_range(1..=20);
            let cols = rng.random_range(1..=20);
            let mut m = vec![vec![0i64; cols]; rows];
            for row in m.iter_mut().take(rows) {
                for x in row.iter_mut().take(cols) {
                    *x = rng.random_range(-1_000_000..=1_000_000);
                }
            }

            let r1 = rotate_90_degrees(m.clone());
            let r2 = rotate_90_degrees(r1);
            let r3 = rotate_90_degrees(r2);
            let r4 = rotate_90_degrees(r3);
            assert_eq!(r4, m);
        }
    }
}
