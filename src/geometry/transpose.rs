pub fn transpose<T: Clone + Copy + Default>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![T::default(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    transposed
}

#[cfg(test)]
mod tests {
    use super::transpose;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn is_rect<T>(m: &[Vec<T>]) -> bool {
        if m.is_empty() {
            return true;
        }
        let c = m[0].len();
        m.iter().all(|row| row.len() == c)
    }

    #[test]
    fn test_empty() {
        let m: Vec<Vec<i32>> = vec![];
        let t = transpose(m);
        assert!(t.is_empty());
    }

    #[test]
    fn test_1x1() {
        let m = vec![vec![7i32]];
        let t = transpose(m);
        assert_eq!(t, vec![vec![7]]);
    }

    #[test]
    fn test_row_vector() {
        let m = vec![vec![1i32, 2, 3, 4]];
        let t = transpose(m);
        assert_eq!(t, vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_col_vector() {
        let m = vec![vec![1i32], vec![2], vec![3]];
        let t = transpose(m);
        assert_eq!(t, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_rectangular_manual() {
        let m = vec![vec![1i32, 2, 3], vec![4, 5, 6]];
        let t = transpose(m);
        assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6],]);
    }

    #[test]
    fn property_double_transpose_is_identity() {
        let mut rng = StdRng::seed_from_u64(20251220);

        for _ in 0..10_000 {
            let rows = rng.random_range(0..=30);
            let cols = if rows == 0 {
                0
            } else {
                rng.random_range(1..=30)
            };

            let mut m = vec![vec![0i64; cols]; rows];
            for i in 0..rows {
                for j in 0..cols {
                    m[i][j] = rng.random_range(-1_000_000..=1_000_000);
                }
            }
            assert!(is_rect(&m));

            let tt = transpose(transpose(m.clone()));
            assert_eq!(tt, m);
        }
    }

    #[test]
    fn property_dimensions_and_values() {
        let mut rng = StdRng::seed_from_u64(123456);

        for _ in 0..10_000 {
            let rows = rng.random_range(1..=25);
            let cols = rng.random_range(0..=25);

            let mut m = vec![vec![0u32; cols]; rows];
            for i in 0..rows {
                for j in 0..cols {
                    m[i][j] = rng.random();
                }
            }
            assert!(is_rect(&m));

            let t = transpose(m.clone());

            assert_eq!(t.len(), cols);
            if cols > 0 {
                assert!(t.iter().all(|row| row.len() == rows));
            }

            for i in 0..rows {
                for j in 0..cols {
                    assert_eq!(t[j][i], m[i][j]);
                }
            }
        }
    }

    #[test]
    fn test_zero_cols_nonempty_rows() {
        let m: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        let t = transpose(m);
        assert!(t.is_empty());
    }
}
