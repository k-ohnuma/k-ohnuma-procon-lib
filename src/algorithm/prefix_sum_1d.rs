use num_traits::Zero;
use std::ops::{Add, Bound, Range, RangeBounds, Sub};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixSum1D<T> {
    acc: Vec<T>,
}

impl<T> PrefixSum1D<T>
where
    T: Zero + Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(xs: &[T]) -> Self {
        let mut acc = vec![T::zero()];
        for &x in xs.iter() {
            let &la = acc.last().unwrap();
            acc.push(la + x);
        }
        Self { acc }
    }

    pub fn acc(&self) -> &Vec<T> {
        &self.acc
    }

    // rangeの和
    pub fn range_sum(&self, range: impl RangeBounds<usize>) -> T {
        let range = self.get_range(range);
        let end = self.acc[range.end];
        let start = self.acc[range.start];
        end - start
    }

    /// 区間 [0, end) の和
    pub fn prefix_sum(&self, end: usize) -> T {
        self.acc[end]
    }

    /// 区間 [begin, n) の和
    pub fn suffix_sum(&self, begin: usize) -> T {
        let n = self.acc.len() - 1;
        let total = self.acc[n];
        total - self.acc[begin]
    }

    /// 全要素の和
    pub fn total_sum(&self) -> T {
        let &la = self.acc.last().unwrap_or(&T::zero());
        la
    }

    /// f(sum(l..r)) が true となる最大の r in [l, n] を二分探索で探す。
    ///
    /// 条件:
    /// - f(0) は true
    /// - r が増えるといつか false になり、その後はずっと false（単調）
    ///
    /// O(log n)
    pub fn max_right<F>(&self, l: usize, mut f: F) -> usize
    where
        F: FnMut(T) -> bool,
    {
        let n = self.acc.len() - 1;
        assert!(l <= n);
        assert!(f(T::zero()), "f(0) must be true");

        if f(self.range_sum(l..n)) {
            // r=n はOK（[l, n)）
            return n;
        }

        let mut ok = l;
        let mut ng = n + 1;
        while ng - ok > 1 {
            let mid = ok + (ng - ok) / 2;
            if f(self.range_sum(l..mid)) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    /// f(sum(l..r)) が true となる最小の l in [0, r] を二分探索で探す。
    ///
    /// 条件:
    /// - f(0) は true
    /// - l を右に動かしていくと、ある地点からずっと true（単調）
    ///
    /// O(log r)
    pub fn min_left<F>(&self, r: usize, mut f: F) -> usize
    where
        F: FnMut(T) -> bool,
    {
        let n = self.acc.len() - 1;
        assert!(r <= n);
        assert!(f(T::zero()), "f(0) must be true");

        if f(self.range_sum(0..r)) {
            return 0;
        }

        let mut ok = r;
        let mut ng = 0;
        while ok - ng > 1 {
            let mid = ng + (ok - ng) / 2;
            if f(self.range_sum(mid..r)) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    fn get_range(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        let begin = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let end = match range.end_bound() {
            Bound::Excluded(&x) => x,
            Bound::Included(&x) => x + 1,
            Bound::Unbounded => self.acc.len() - 1,
        };
        begin..end
    }
}
