use num_traits::Zero;
use std::ops::{Add, AddAssign, Bound, Range, RangeBounds, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct PrefixSum2D<T> {
    diff: Vec<Vec<T>>,
    x: usize,
    y: usize,

    acc: Option<Vec<Vec<T>>>,
}

impl<T> PrefixSum2D<T>
where
    T: Zero + Copy + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            diff: vec![vec![T::zero(); y + 1]; x + 1],
            x,
            y,
            acc: None,
        }
    }

    pub fn from_vec(xs: &[Vec<T>]) -> Self {
        let x = xs.len();
        let y = xs.first().map(|v| v.len()).unwrap_or(0);

        for (i, row) in xs.iter().enumerate().take(x) {
            assert!(row.len() == y, "y dimension mismatch at x={i}");
        }

        let diff = vec![vec![T::zero(); y + 1]; x + 1];
        let mut acc = vec![vec![T::zero(); y + 1]; x + 1];

        // acc[i][j] = sum(xs[0..i)[0..j))
        for i in 0..x {
            for j in 0..y {
                acc[i + 1][j + 1] = acc[i][j + 1] + acc[i + 1][j] - acc[i][j] + xs[i][j];
            }
        }

        Self {
            diff,
            x,
            y,
            acc: Some(acc),
        }
    }

    /// 矩形 [x0,x1)×[y0,y1) に +v（2Dいもす）
    pub fn add_rect(&mut self, x0: usize, x1: usize, y0: usize, y1: usize, v: T) {
        assert!(x0 <= x1 && x1 <= self.x);
        assert!(y0 <= y1 && y1 <= self.y);

        self.acc = None;

        self.diff[x0][y0] += v;
        self.diff[x1][y0] -= v;
        self.diff[x0][y1] -= v;
        self.diff[x1][y1] += v;
    }

    pub fn build(&mut self) {
        let mut val = vec![vec![T::zero(); self.y]; self.x];

        for i in 0..self.x {
            for j in 0..self.y {
                let mut v = self.diff[i][j];
                if i > 0 {
                    v += val[i - 1][j];
                }
                if j > 0 {
                    v += val[i][j - 1];
                }
                if i > 0 && j > 0 {
                    v -= val[i - 1][j - 1];
                }
                val[i][j] = v;
            }
        }

        let mut acc = vec![vec![T::zero(); self.y + 1]; self.x + 1];
        for i in 0..self.x {
            for j in 0..self.y {
                acc[i + 1][j + 1] = acc[i][j + 1] + acc[i + 1][j] - acc[i][j] + val[i][j];
            }
        }

        self.acc = Some(acc);
    }
    pub fn prefix_sum(&self, x: usize, y: usize) -> T {
        let acc = self.acc.as_ref().expect("call build() first");
        acc[x][y]
    }

    pub fn range_sum(
        &self,
        x_range: impl RangeBounds<usize>,
        y_range: impl RangeBounds<usize>,
    ) -> T {
        let acc = self.acc.as_ref().expect("call build() first");

        let xr = get_range(x_range, self.x);
        let yr = get_range(y_range, self.y);

        let (x0, x1) = (xr.start, xr.end);
        let (y0, y1) = (yr.start, yr.end);

        let a = acc[x1][y1];
        let b = acc[x0][y1];
        let c = acc[x1][y0];
        let d = acc[x0][y0];

        a - b - c + d
    }

    pub fn total_sum(&self) -> T {
        self.prefix_sum(self.x, self.y)
    }
}

fn get_range(range: impl RangeBounds<usize>, n: usize) -> Range<usize> {
    let begin = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Excluded(&x) => x,
        Bound::Included(&x) => x + 1,
        Bound::Unbounded => n,
    };
    assert!(begin <= end);
    assert!(end <= n);
    begin..end
}

#[cfg(test)]
mod small_tests {
    use super::PrefixSum2D;

    #[test]
    fn empty_rect_does_nothing() {
        let mut ps = PrefixSum2D::<i64>::new(4, 5);
        ps.add_rect(2, 2, 1, 4, 10); // 幅0
        ps.build();
        assert_eq!(ps.total_sum(), 0);
        assert_eq!(ps.range_sum(0..4, 0..5), 0);
    }

    #[test]
    fn single_cell_update() {
        let mut ps = PrefixSum2D::<i64>::new(3, 3);
        ps.add_rect(1, 2, 1, 2, 5);
        ps.build();

        assert_eq!(ps.range_sum(1..2, 1..2), 5);
        assert_eq!(ps.range_sum(0..3, 0..3), 5);
        assert_eq!(ps.range_sum(2..3, 1..2), 0);
    }

    #[test]
    fn whole_area_update() {
        let mut ps = PrefixSum2D::<i64>::new(4, 5);
        ps.add_rect(0, 4, 0, 5, 1);
        ps.build();

        assert_eq!(ps.total_sum(), 4 * 5);
        assert_eq!(ps.range_sum(0..4, 0..5), 20);
        assert_eq!(ps.range_sum(0..1, 0..5), 5);
        assert_eq!(ps.range_sum(0..4, 0..1), 4);
    }

    #[test]
    fn overlapping_rectangles() {
        let mut ps = PrefixSum2D::<i64>::new(4, 4);
        ps.add_rect(0, 3, 0, 3, 2); // 3x3 *2 = 18
        ps.add_rect(1, 4, 1, 4, -1); // 3x3 *(-1) = -9
        ps.build();

        // 交差: [1,3)x[1,3) = 2x2, 値 = 1
        assert_eq!(ps.range_sum(1..3, 1..3), 4);

        // 箱Aのみ
        assert_eq!(ps.range_sum(0..1, 0..3), 3 * 2);

        // 箱Bのみ
        assert_eq!(ps.range_sum(3..4, 3..4), -1);

        assert_eq!(ps.total_sum(), 9);
    }

    #[test]
    fn from_vec_hand_check() {
        let a = vec![vec![1i64, 2, 3], vec![4i64, 5, 6]];
        let ps = PrefixSum2D::from_vec(&a);

        assert_eq!(ps.total_sum(), 21);
        assert_eq!(ps.range_sum(0..1, 0..1), 1);
        assert_eq!(ps.range_sum(0..2, 0..3), 21);
        assert_eq!(ps.range_sum(1..2, 1..3), 11);
    }
}

#[cfg(test)]
mod random_tests {
    use super::PrefixSum2D;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256PlusPlus;

    fn naive_apply_rect(a: &mut [Vec<i64>], x0: usize, x1: usize, y0: usize, y1: usize, v: i64) {
        for row in a.iter_mut().take(x1).skip(x0) {
            for x in row.iter_mut().take(y1).skip(y0) {
                *x += v;
            }
        }
    }

    fn naive_range_sum(a: &[Vec<i64>], x0: usize, x1: usize, y0: usize, y1: usize) -> i64 {
        let mut s = 0;
        for row in a.iter().take(x1).skip(x0) {
            for &x in row.iter().take(y1).skip(y0) {
                s += x;
            }
        }
        s
    }

    #[test]
    fn random_rectangles_match_naive() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);

        for _ in 0..200 {
            let x = rng.random_range(0..6);
            let y = rng.random_range(0..6);

            let mut ps = PrefixSum2D::<i64>::new(x, y);
            let mut a = vec![vec![0i64; y]; x];

            for _ in 0..200 {
                let x0 = rng.random_range(0..=x);
                let x1 = rng.random_range(x0..=x);
                let y0 = rng.random_range(0..=y);
                let y1 = rng.random_range(y0..=y);
                let v = rng.random_range(-10..=10);

                ps.add_rect(x0, x1, y0, y1, v);
                naive_apply_rect(&mut a, x0, x1, y0, y1, v);
            }

            ps.build();

            for _ in 0..200 {
                let x0 = rng.random_range(0..=x);
                let x1 = rng.random_range(x0..=x);
                let y0 = rng.random_range(0..=y);
                let y1 = rng.random_range(y0..=y);

                let got = ps.range_sum(x0..x1, y0..y1);
                let exp = naive_range_sum(&a, x0, x1, y0, y1);
                assert_eq!(got, exp);
            }

            let exp_total = naive_range_sum(&a, 0, x, 0, y);
            assert_eq!(ps.total_sum(), exp_total);
        }
    }
}
