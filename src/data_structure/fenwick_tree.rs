use std::ops::{Bound, RangeBounds};

#[derive(Clone)]
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T> + PartialOrd> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
        FenwickTree {
            n,
            ary: vec![e.clone(); n],
            e,
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum<R>(&self, range: R) -> T
    where
        T: std::ops::Sub<Output = T>,
        R: RangeBounds<usize>,
    {
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => return self.accum(r),
        };
        self.accum(r) - self.accum(l)
    }
    /// prefix sum が k 以上になる最小の idx (0-indexed)
    /// 値が全て非負であること仮定。
    pub fn lower_bound(&self, k: T) -> Option<usize> {
        // k <= 0 相当なら先頭（eが0のとき）。一般型では厳密判定しづらいので e と比較。
        if k <= self.e {
            return Some(0);
        }
        if self.accum(self.n) < k {
            return None;
        }

        let mut idx = 0usize;
        let mut cur = self.e.clone();

        let mut bit = 1usize;
        while (bit << 1) <= self.n {
            bit <<= 1;
        }

        while bit > 0 {
            let next = idx + bit;
            if next <= self.n {
                let mut cand = cur.clone();
                cand += self.ary[next - 1].clone();
                if cand < k {
                    cur = cand;
                    idx = next;
                }
            }
            bit >>= 1;
        }
        Some(idx) // 0-indexed の data の位置
    }

    /// prefix sum が k より大きくなる最小 idx (0-indexed)
    /// 値が全て非負であること仮定。
    pub fn upper_bound(&self, k: T) -> Option<usize> {
        if self.accum(self.n) <= k {
            return None;
        }

        let mut idx = 0usize;
        let mut cur = self.e.clone();

        let mut bit = 1usize;
        while (bit << 1) <= self.n {
            bit <<= 1;
        }

        while bit > 0 {
            let next = idx + bit;
            if next <= self.n {
                let mut cand = cur.clone();
                cand += self.ary[next - 1].clone();
                if cand <= k {
                    cur = cand;
                    idx = next;
                }
            }
            bit >>= 1;
        }
        Some(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn naive_prefix(a: &[i64], r: usize) -> i64 {
        a[..r].iter().sum()
    }
    fn naive_sum(a: &[i64], l: usize, r: usize) -> i64 {
        a[l..r].iter().sum()
    }

    #[test]
    fn basic_add_sum() {
        let mut fw = FenwickTree::<i64>::new(5, 0);
        fw.add(0, 3);
        fw.add(3, 10);
        fw.add(4, -2);

        assert_eq!(fw.accum(0), 0);
        assert_eq!(fw.accum(1), 3);
        assert_eq!(fw.accum(4), 13);
        assert_eq!(fw.accum(5), 11);

        assert_eq!(fw.sum(0..0), 0);
        assert_eq!(fw.sum(0..1), 3);
        assert_eq!(fw.sum(3..5), 8);
        assert_eq!(fw.sum(..), 11);
        assert_eq!(fw.sum(..3), 3);
        assert_eq!(fw.sum(3..), 8);
        assert_eq!(fw.sum(0..=4), 11);
    }

    #[test]
    fn randomized_add_sum_accum_compare_naive() {
        let mut rng = StdRng::seed_from_u64(1);

        for n in 1..80usize {
            let mut fw = FenwickTree::<i64>::new(n, 0);
            let mut a = vec![0i64; n];

            for _ in 0..8000 {
                if rng.random_bool(0.6) {
                    let idx = rng.random_range(0..n);
                    let v = rng.random_range(-100i64..=100);
                    fw.add(idx, v);
                    a[idx] += v;
                } else {
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(0..=n);
                    let (l, r) = if l <= r { (l, r) } else { (r, l) };

                    let mode = rng.random_range(0..5);
                    let got = match mode {
                        0 => fw.sum(l..r),
                        1 => fw.sum(..r),
                        2 => fw.sum(l..),
                        3 => {
                            if l == r {
                                fw.sum(l..r)
                            } else {
                                fw.sum(l..=r - 1)
                            }
                        }
                        _ => fw.sum(..),
                    };

                    let exp = match mode {
                        0 => naive_sum(&a, l, r),
                        1 => naive_sum(&a, 0, r),
                        2 => naive_sum(&a, l, n),
                        3 => naive_sum(&a, l, r),
                        _ => naive_sum(&a, 0, n),
                    };

                    assert_eq!(got, exp, "n={n} l={l} r={r} mode={mode}");
                }

                let r = rng.random_range(0..=n);
                assert_eq!(fw.accum(r), naive_prefix(&a, r), "n={n} r={r}");
            }

            for r in 0..=n {
                assert_eq!(fw.accum(r), naive_prefix(&a, r), "final n={n} r={r}");
            }
        }
    }

    #[test]
    fn randomized_lower_upper_bound_compare_linear_nonnegative() {
        let mut rng = StdRng::seed_from_u64(2025);

        for n in 1..120usize {
            let mut fw = FenwickTree::<i64>::new(n, 0);
            let mut a = vec![0i64; n];

            for (i, ai) in a.iter_mut().enumerate().take(n) {
                let v = rng.random_range(0i64..=20);
                fw.add(i, v);
                *ai = v;
            }

            let total: i64 = a.iter().sum();

            for _ in 0..5000 {
                let k = if rng.random_bool(0.2) {
                    0
                } else {
                    rng.random_range(0i64..=(total + 30))
                };

                let exp_lb = if k <= 0 {
                    Some(0)
                } else {
                    let mut s = 0i64;
                    let mut ans = None;
                    for (i, &ai) in a.iter().enumerate().take(n) {
                        s += ai;
                        if s >= k {
                            ans = Some(i);
                            break;
                        }
                    }
                    ans
                };
                assert_eq!(fw.lower_bound(k), exp_lb, "n={n} k={k} total={total}");

                let exp_ub = {
                    let mut s = 0i64;
                    let mut ans = None;
                    for (i, &ai) in a.iter().enumerate().take(n) {
                        s += ai;
                        if s > k {
                            ans = Some(i);
                            break;
                        }
                    }
                    ans
                };
                assert_eq!(fw.upper_bound(k), exp_ub, "n={n} k={k} total={total}");

                if let Some(i) = exp_lb {
                    let pre_i = a[..=i].iter().sum::<i64>();
                    let t = a[..i].iter().sum::<i64>();
                    let pre_prev = if i == 0 { 0 } else { t };
                    assert!(pre_i >= k);
                    if k > 0 {
                        assert!(pre_prev < k);
                    }
                }
                if let Some(i) = exp_ub {
                    let pre_i = a[..=i].iter().sum::<i64>();
                    let pre_prev = if i == 0 {
                        0
                    } else {
                        a[..i].iter().sum::<i64>()
                    };
                    assert!(pre_i > k);
                    assert!(pre_prev <= k);
                }
            }

            for _ in 0..2000 {
                let idx = rng.random_range(0..n);
                let addv = rng.random_range(0i64..=5);
                fw.add(idx, addv);
                a[idx] += addv;

                let total2: i64 = a.iter().sum();
                let k = rng.random_range(0i64..=(total2 + 10));

                let exp = if k <= 0 {
                    Some(0)
                } else {
                    let mut s = 0i64;
                    let mut ans = None;
                    for (i, &ai) in a.iter().enumerate().take(n) {
                        s += ai;
                        if s >= k {
                            ans = Some(i);
                            break;
                        }
                    }
                    ans
                };
                assert_eq!(fw.lower_bound(k), exp, "after update n={n} k={k}");
            }
        }
    }

    #[test]
    fn edge_all_zero_bounds() {
        let n = 50;
        let fw = FenwickTree::<i64>::new(n, 0);

        assert_eq!(fw.accum(n), 0);
        assert_eq!(fw.sum(..), 0);

        assert_eq!(fw.lower_bound(0), Some(0));
        assert_eq!(fw.lower_bound(1), None);
        assert_eq!(fw.upper_bound(0), None);
    }
}
