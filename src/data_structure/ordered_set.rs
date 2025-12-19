use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    hash::Hash,
};

use std::ops::{Bound, RangeBounds};

use num_traits::Bounded;

use crate::data_structure::fenwick_tree::FenwickTree;

pub struct OrderedSet<T: Ord + Hash + Copy + Bounded> {
    id_map: HashMap<T, usize>,
    inv: Vec<T>,
    id_max: usize,
    fen: FenwickTree<isize>,
    set: BTreeSet<T>,
}

impl<T: Ord + Hash + Copy + Bounded + Debug> OrderedSet<T> {
    /// オフラインでの利用を想定
    pub fn from(v: &Vec<T>) -> Self {
        let mut v = v.to_owned();
        v.sort();
        v.dedup();
        let mut id = 0;
        let mut id_map = HashMap::new();
        for &num in v.iter() {
            id_map.insert(num, id);
            id += 1usize;
        }
        let mut inv = vec![T::max_value(); id];
        let fen = FenwickTree::new(id + 5, 0);
        for &num in v.iter() {
            let &id = id_map.get(&num).unwrap();
            inv[id] = num;
        }
        Self {
            id_map,
            inv,
            id_max: id,
            fen,
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, num: T) -> bool {
        let id = self.get_id(num);
        if !self.set.insert(num) {
            return false;
        }
        self.fen.add(id, 1);
        true
    }

    pub fn remove(&mut self, num: T) -> bool {
        let id = self.get_id(num);
        if !self.set.remove(&num) {
            return false;
        }
        self.fen.add(id, -1);
        true
    }

    pub fn range_count<R>(&self, range: R) -> usize
    where
        R: RangeBounds<T> + Debug,
    {
        let set = self.set();
        let low: Option<T> = match range.start_bound() {
            Bound::Included(&l) => set.range(l..).next().copied(),
            Bound::Excluded(&l) => set
                .range((Bound::Excluded(l), Bound::Unbounded))
                .next()
                .copied(),
            Bound::Unbounded => set.iter().next().copied(),
        };

        let high: Option<T> = match range.end_bound() {
            Bound::Included(&r) => set.range(..=r).next_back().copied(),
            Bound::Excluded(&r) => set.range(..r).next_back().copied(),
            Bound::Unbounded => set.iter().next_back().copied(),
        };

        let (low, high) = match (low, high) {
            (Some(a), Some(b)) => (a, b),
            _ => return 0,
        };

        if low > high {
            return 0;
        }

        let l = self.get_id(low);
        let r = self.get_id(high);

        self.fen.sum(l..=r) as usize
    }

    /// 昇順n番目の要素を取得する。0-indexed
    pub fn get_nth(&self, n: usize) -> Option<T> {
        let c = n + 1;
        let id = self.fen.lower_bound(c as isize)?;
        Some(self.inv[id])
    }

    pub fn set(&self) -> &BTreeSet<T> {
        &self.set
    }

    pub fn get_id(&self, num: T) -> usize {
        let &id = self.id_map.get(&num).unwrap();
        id
    }
    pub fn max_id(&self) -> usize {
        self.id_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::BTreeSet;
    use std::ops::Bound;

    fn nth_in_set(set: &BTreeSet<i64>, n: usize) -> Option<i64> {
        set.iter().nth(n).copied()
    }

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(20251219);

        let m = 300usize;
        let mut universe = Vec::with_capacity(m);
        for _ in 0..m {
            universe.push(rng.random_range(-1000i64..=1000));
        }

        let mut os = OrderedSet::<i64>::from(&universe);
        let mut set = BTreeSet::<i64>::new();

        for _step in 0..30000 {
            let op = rng.random_range(0..100);

            if op < 45 {
                // insert
                let x = universe[rng.random_range(0..m)];
                let a = os.insert(x);
                let b = set.insert(x);
                assert_eq!(a, b);
            } else if op < 75 {
                // remove
                let x = universe[rng.random_range(0..m)];
                let a = os.remove(x);
                let b = set.remove(&x);
                assert_eq!(a, b);
            } else if op < 93 {
                // range_count
                let a = universe[rng.random_range(0..m)];
                let b = universe[rng.random_range(0..m)];
                let (lo, hi) = if a <= b { (a, b) } else { (b, a) };

                let mode = rng.random_range(0..=4);
                let (lb, rb) = match mode {
                    0 => (Bound::Included(lo), Bound::Included(hi)), // [lo, hi]
                    1 => (Bound::Included(lo), Bound::Excluded(hi)), // [lo, hi)
                    2 => (Bound::Unbounded, Bound::Included(hi)),    // (-inf, hi]
                    3 => (Bound::Unbounded, Bound::Excluded(hi)),    // (-inf, hi)
                    _ => (Bound::Included(lo), Bound::Unbounded),    // [lo, +inf)
                };

                let run = || -> usize {
                    match (lb, rb) {
                        (Bound::Included(l), Bound::Included(r)) => os.range_count(l..=r),
                        (Bound::Included(l), Bound::Excluded(r)) => os.range_count(l..r),
                        (Bound::Excluded(l), Bound::Included(r)) => {
                            os.range_count((Bound::Excluded(l), Bound::Included(r)))
                        }
                        (Bound::Excluded(l), Bound::Excluded(r)) => {
                            os.range_count((Bound::Excluded(l), Bound::Excluded(r)))
                        }
                        (Bound::Unbounded, Bound::Included(r)) => os.range_count(..=r),
                        (Bound::Unbounded, Bound::Excluded(r)) => os.range_count(..r),
                        (Bound::Included(l), Bound::Unbounded) => os.range_count(l..),
                        (Bound::Excluded(l), Bound::Unbounded) => {
                            os.range_count((Bound::Excluded(l), Bound::Unbounded))
                        }
                        (Bound::Unbounded, Bound::Unbounded) => os.range_count(..),
                    }
                };

                let exp = set.range((lb, rb)).count();
                let res = run();
                assert_eq!(res, exp, "count mismatch mode={mode} lo={lo} hi={hi}");
            } else {
                // get_nth
                let sz = set.len();
                let n = if rng.random_bool(0.8) {
                    if sz == 0 {
                        0
                    } else {
                        rng.random_range(0..sz)
                    }
                } else {
                    rng.random_range(0..(sz + 10))
                };
                let got = os.get_nth(n);
                let exp = nth_in_set(&set, n);
                assert_eq!(got, exp, "get_nth mismatch n={n} sz={sz}");
            }

            let vec_os: Vec<i64> = os.set().iter().copied().collect();
            let vec_set: Vec<i64> = set.iter().copied().collect();
            assert_eq!(vec_os, vec_set);
        }
    }
}
