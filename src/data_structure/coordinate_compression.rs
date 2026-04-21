use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Bound, RangeBounds},
};

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
    fn lower_bound(&self, x: T) -> usize {
        self.xs.partition_point(|&v| v < x)
    }

    /// xs の中で x より大きい最小の index
    fn upper_bound(&self, x: T) -> usize {
        self.xs.partition_point(|&v| v <= x)
    }

    fn bounds_to_lr<R: RangeBounds<T>>(&self, range: R) -> (usize, usize) {
        let n = self.xs.len();

        let l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => self.lower_bound(x), // >= x
            Bound::Excluded(&x) => self.upper_bound(x), // > x
        };

        let r = match range.end_bound() {
            Bound::Unbounded => n,
            Bound::Included(&x) => self.upper_bound(x), // <= x の次
            Bound::Excluded(&x) => self.lower_bound(x), // < x の次
        };

        (l.min(n), r.min(n))
    }

    /// - 範囲内の最小要素を返す(id, value)
    pub fn next<R: RangeBounds<T>>(&self, range: R) -> Option<(usize, T)> {
        let (l, r) = self.bounds_to_lr(range);
        if l < r {
            Some((l, self.xs[l]))
        } else {
            None
        }
    }

    /// - 範囲内の最大要素を返す(id, value)
    pub fn next_back<R: RangeBounds<T>>(&self, range: R) -> Option<(usize, T)> {
        let (l, r) = self.bounds_to_lr(range);
        if l < r {
            let i = r - 1;
            Some((i, self.xs[i]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CoordinateCompression;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::BTreeSet;
    use std::ops::{Bound, RangeBounds};

    #[derive(Clone, Debug)]
    struct SpecRange {
        start: Bound<i32>,
        end: Bound<i32>,
    }

    impl RangeBounds<i32> for SpecRange {
        fn start_bound(&self) -> Bound<&i32> {
            match &self.start {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(v) => Bound::Included(v),
                Bound::Excluded(v) => Bound::Excluded(v),
            }
        }
        fn end_bound(&self) -> Bound<&i32> {
            match &self.end {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(v) => Bound::Included(v),
                Bound::Excluded(v) => Bound::Excluded(v),
            }
        }
    }

    fn rand_bound(rng: &mut StdRng, lo: i32, hi: i32) -> Bound<i32> {
        match rng.random_range(0..3) {
            0 => Bound::Unbounded,
            1 => Bound::Included(rng.random_range(lo..=hi)),
            _ => Bound::Excluded(rng.random_range(lo..=hi)),
        }
    }

    fn make_random_spec(rng: &mut StdRng, lo: i32, hi: i32) -> SpecRange {
        SpecRange {
            start: rand_bound(rng, lo, hi),
            end: rand_bound(rng, lo, hi),
        }
    }

    fn start_key(b: &Bound<i32>) -> (i64, u8) {
        match b {
            Bound::Unbounded => (i64::MIN, 0),
            Bound::Included(x) => (*x as i64, 0),
            Bound::Excluded(x) => (*x as i64, 1),
        }
    }

    fn end_key(b: &Bound<i32>) -> (i64, u8) {
        match b {
            Bound::Unbounded => (i64::MAX, 1),
            Bound::Included(x) => (*x as i64, 1),
            Bound::Excluded(x) => (*x as i64, 0),
        }
    }

    fn spec_is_valid(spec: &SpecRange) -> bool {
        start_key(&spec.start) <= end_key(&spec.end)
    }

    fn make_valid_random_spec(rng: &mut StdRng, lo: i32, hi: i32) -> SpecRange {
        loop {
            let s = make_random_spec(rng, lo, hi);
            if spec_is_valid(&s) {
                return s;
            }
        }
    }

    fn assert_consistent(cc: &CoordinateCompression<i32>, set: &BTreeSet<i32>, spec: SpecRange) {
        let exp_fwd = set.range(spec.clone()).next().copied();
        let exp_bwd = set.range(spec.clone()).next_back().copied();

        let got_fwd = cc.next(spec.clone()).map(|(_id, v)| v);
        let got_bwd = cc.next_back(spec.clone()).map(|(_id, v)| v);

        assert_eq!(
            got_fwd,
            exp_fwd,
            "next mismatch: spec={:?}, xs={:?}",
            spec,
            cc.values()
        );
        assert_eq!(
            got_bwd,
            exp_bwd,
            "next_back mismatch: spec={:?}, xs={:?}",
            spec,
            cc.values()
        );

        if let Some((id, v)) = cc.next(spec.clone()) {
            assert!(id < cc.len(), "id out of bounds (next): id={}", id);
            assert_eq!(cc.value(id), v, "id->value inconsistent (next)");
            assert_eq!(cc.get(v), Some(id), "value->id inconsistent (next)");
        }
        if let Some((id, v)) = cc.next_back(spec.clone()) {
            assert!(id < cc.len(), "id out of bounds (next_back): id={}", id);
            assert_eq!(cc.value(id), v, "id->value inconsistent (next_back)");
            assert_eq!(cc.get(v), Some(id), "value->id inconsistent (next_back)");
        }
    }

    #[test]
    fn random_next_and_next_back_strong() {
        let mut rng = StdRng::seed_from_u64(0xC0FFEE_u64);

        for _case in 0..500 {
            let n = rng.random_range(0..200);

            let mut a = Vec::with_capacity(n);
            for _ in 0..n {
                let dice = rng.random_range(0..20);
                let v = if dice == 0 {
                    i32::MIN + rng.random_range(0..1000)
                } else if dice == 1 {
                    i32::MAX - rng.random_range(0..1000)
                } else {
                    rng.random_range(-50..=50)
                };
                a.push(v);
            }

            let cc = CoordinateCompression::new(&a);
            let set: BTreeSet<i32> = cc.values().iter().copied().collect();

            for _q in 0..800 {
                let spec = make_valid_random_spec(&mut rng, -80, 80);
                assert_consistent(&cc, &set, spec);
            }

            let specials = [
                SpecRange {
                    start: Bound::Unbounded,
                    end: Bound::Unbounded,
                }, // ..
                SpecRange {
                    start: Bound::Included(-10),
                    end: Bound::Unbounded,
                }, // [-10, +inf)
                SpecRange {
                    start: Bound::Excluded(-10),
                    end: Bound::Unbounded,
                }, // (-10, +inf)
                SpecRange {
                    start: Bound::Unbounded,
                    end: Bound::Included(10),
                }, // (-inf, 10]
                SpecRange {
                    start: Bound::Unbounded,
                    end: Bound::Excluded(10),
                }, // (-inf, 10)
                SpecRange {
                    start: Bound::Included(-10),
                    end: Bound::Included(10),
                }, // [-10, 10]
            ];
            for spec in specials {
                assert!(spec_is_valid(&spec));
                assert_consistent(&cc, &set, spec);
            }
        }
    }

    #[test]
    fn targeted_floor_ceil_behavior() {
        let a = vec![5, 1, 7, 7, -3, 10];
        let cc = CoordinateCompression::new(&a);
        let set: BTreeSet<i32> = cc.values().iter().copied().collect();

        for x in -20..=20 {
            let exp = set
                .range(SpecRange {
                    start: Bound::Included(x),
                    end: Bound::Unbounded,
                })
                .next()
                .copied();
            let got = cc.next(x..).map(|(_, v)| v);
            assert_eq!(got, exp, "ceil mismatch at x={}", x);

            let exp = set
                .range(SpecRange {
                    start: Bound::Unbounded,
                    end: Bound::Included(x),
                })
                .next_back()
                .copied();
            let got = cc.next_back(..=x).map(|(_, v)| v);
            assert_eq!(got, exp, "floor mismatch at x={}", x);
        }
    }
}
