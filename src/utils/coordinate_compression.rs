pub mod coordinate_compression {
    use std::{collections::HashMap, hash::Hash};

    #[derive(Clone, Debug)]
    pub struct CoordinateCompression<T> {
        xs: Vec<T>,
        id_map: HashMap<T, usize>,
    }

    impl<T> CoordinateCompression<T>
    where
        T: Ord + Hash + Eq + Copy,
    {
        pub fn new(a: &[T]) -> Self {
            let mut xs = a.to_vec();
            xs.sort();
            xs.dedup();

            let mut id_map = HashMap::with_capacity(xs.len());
            for (i, &x) in xs.iter().enumerate() {
                id_map.insert(x, i);
            }

            Self { xs, id_map }
        }

        /// Option版
        pub fn get(&self, x: T) -> Option<usize> {
            self.id_map.get(&x).copied()
        }

        /// id -> 元の値
        pub fn value(&self, id: usize) -> T {
            self.xs[id]
        }

        pub fn len(&self) -> usize {
            self.xs.len()
        }

        pub fn is_empty(&self) -> bool {
            self.xs.is_empty()
        }

        pub fn values(&self) -> &[T] {
            &self.xs
        }

        /// xs の中で x 以上となる最小の index
        pub fn lower_bound(&self, x: T) -> usize {
            self.xs.partition_point(|&v| v < x)
        }

        /// xs の中で x より大きい最小の index
        pub fn upper_bound(&self, x: T) -> usize {
            self.xs.partition_point(|&v| v <= x)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::utils::coordinate_compression::coordinate_compression::CoordinateCompression;

    #[test]
    fn random_test() {
        let mut rng = rand::rng();

        for _ in 0..3000 {
            let n = rng.random_range(0..=200);
            let mut a = Vec::with_capacity(n);
            for _ in 0..n {
                a.push(rng.random_range(-50i64..=50));
            }

            let cc = CoordinateCompression::new(&a);

            let xs = cc.values().to_vec();
            let mut ys = a.clone();
            ys.sort();
            ys.dedup();
            assert_eq!(xs, ys);

            for &x in xs.iter() {
                let id = cc.get(x).unwrap();
                assert_eq!(cc.value(id), x);
            }

            for _ in 0..50 {
                let x = rng.random_range(-60i64..=60);

                let lb = cc.lower_bound(x);
                let ub = cc.upper_bound(x);

                let brute_lb = xs.iter().position(|&v| v >= x).unwrap_or(xs.len());
                let brute_ub = xs.iter().position(|&v| v > x).unwrap_or(xs.len());

                assert_eq!(lb, brute_lb);
                assert_eq!(ub, brute_ub);
            }
        }
    }
}
