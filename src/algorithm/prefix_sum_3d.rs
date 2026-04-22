use num_traits::Zero;
use std::ops::{Add, AddAssign, Bound, Range, RangeBounds, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct PrefixSum3D<T> {
    diff: Vec<Vec<Vec<T>>>,
    x: usize,
    y: usize,
    z: usize,

    acc: Option<Vec<Vec<Vec<T>>>>,
}

impl<T> PrefixSum3D<T>
where
    T: Zero + Copy + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            diff: vec![vec![vec![T::zero(); z + 1]; y + 1]; x + 1],
            x,
            y,
            z,
            acc: None,
        }
    }

    pub fn from_vec(xs: &[Vec<Vec<T>>]) -> Self {
        let x = xs.len();
        let y = xs.first().map(|v| v.len()).unwrap_or(0);
        let z = xs
            .first()
            .and_then(|v| v.first())
            .map(|v| v.len())
            .unwrap_or(0);

        for (i, x_plane) in xs.iter().enumerate().take(x) {
            assert!(x_plane.len() == y, "y dimension mismatch at x={i}");
            for (j, y_row) in x_plane.iter().enumerate().take(y) {
                assert!(y_row.len() == z, "z dimension mismatch at x={i}, y={j}");
            }
        }

        let diff = vec![vec![vec![T::zero(); z + 1]; y + 1]; x + 1];
        let mut acc = vec![vec![vec![T::zero(); z + 1]; y + 1]; x + 1];

        for i in 0..x {
            for j in 0..y {
                for k in 0..z {
                    acc[i + 1][j + 1][k + 1] = acc[i][j + 1][k + 1]
                        + acc[i + 1][j][k + 1]
                        + acc[i + 1][j + 1][k]
                        + acc[i][j][k]
                        + xs[i][j][k]
                        - acc[i][j][k + 1]
                        - acc[i][j + 1][k]
                        - acc[i + 1][j][k];
                }
            }
        }

        Self {
            diff,
            x,
            y,
            z,
            acc: Some(acc),
        }
    }

    /// 直方体 [x0,x1)×[y0,y1)×[z0,z1) に +v
    /// build() 前だけ呼ぶ想定（build後に呼ぶと acc が古くなるので None に戻す）
    #[allow(clippy::too_many_arguments)]
    pub fn add_box(
        &mut self,
        x0: usize,
        x1: usize,
        y0: usize,
        y1: usize,
        z0: usize,
        z1: usize,
        v: T,
    ) {
        assert!(x0 <= x1 && x1 <= self.x);
        assert!(y0 <= y1 && y1 <= self.y);
        assert!(z0 <= z1 && z1 <= self.z);

        self.acc = None;

        self.diff[x0][y0][z0] += v;
        self.diff[x1][y1][z0] += v;
        self.diff[x1][y0][z1] += v;
        self.diff[x0][y1][z1] += v;
        self.diff[x1][y0][z0] -= v;
        self.diff[x0][y1][z0] -= v;
        self.diff[x0][y0][z1] -= v;
        self.diff[x1][y1][z1] -= v;
    }

    pub fn build(&mut self) {
        let mut val = vec![vec![vec![T::zero(); self.z]; self.y]; self.x];

        for x in 0..self.x {
            for y in 0..self.y {
                for z in 0..self.z {
                    let mut v = self.diff[x][y][z];
                    if x > 0 {
                        v += val[x - 1][y][z];
                    }
                    if y > 0 {
                        v += val[x][y - 1][z];
                    }
                    if z > 0 {
                        v += val[x][y][z - 1];
                    }

                    if x > 0 && y > 0 {
                        v -= val[x - 1][y - 1][z];
                    }
                    if x > 0 && z > 0 {
                        v -= val[x - 1][y][z - 1];
                    }
                    if y > 0 && z > 0 {
                        v -= val[x][y - 1][z - 1];
                    }

                    if x > 0 && y > 0 && z > 0 {
                        v += val[x - 1][y - 1][z - 1];
                    }
                    val[x][y][z] = v;
                }
            }
        }

        // val -> acc (prefix sum) : acc は +1 シフト
        let mut acc = vec![vec![vec![T::zero(); self.z + 1]; self.y + 1]; self.x + 1];
        for x in 0..self.x {
            for y in 0..self.y {
                for z in 0..self.z {
                    acc[x + 1][y + 1][z + 1] = acc[x][y + 1][z + 1]
                        + acc[x + 1][y][z + 1]
                        + acc[x + 1][y + 1][z]
                        + acc[x][y][z]
                        + val[x][y][z]
                        - acc[x][y][z + 1]
                        - acc[x][y + 1][z]
                        - acc[x + 1][y][z];
                }
            }
        }
        self.acc = Some(acc);
    }

    /// [0,x)×[0,y)×[0,z) の和（build後）
    pub fn prefix_sum(&self, x: usize, y: usize, z: usize) -> T {
        let acc = self.acc.as_ref().expect("call build() first");
        acc[x][y][z]
    }

    /// 直方体領域の和（build後）
    pub fn range_sum(
        &self,
        x_range: impl RangeBounds<usize>,
        y_range: impl RangeBounds<usize>,
        z_range: impl RangeBounds<usize>,
    ) -> T {
        let acc = self.acc.as_ref().expect("call build() first");

        let xr = get_range(x_range, self.x);
        let yr = get_range(y_range, self.y);
        let zr = get_range(z_range, self.z);

        let (x0, x1) = (xr.start, xr.end);
        let (y0, y1) = (yr.start, yr.end);
        let (z0, z1) = (zr.start, zr.end);

        let a = acc[x1][y1][z1];
        let b = acc[x0][y1][z1];
        let c = acc[x1][y0][z1];
        let d = acc[x1][y1][z0];
        let e = acc[x0][y0][z1];
        let f = acc[x0][y1][z0];
        let g = acc[x1][y0][z0];
        let h = acc[x0][y0][z0];

        a + e + f + g - h - b - c - d
    }

    pub fn total_sum(&self) -> T {
        self.prefix_sum(self.x, self.y, self.z)
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
    assert!(begin <= end, "invalid range: begin > end");
    assert!(
        end <= n,
        "range end out of bounds: end={end} > n={n} (use ..n, not ..=n)"
    );
    begin..end
}

#[cfg(test)]
mod tests {
    use crate::algorithm::prefix_sum_3d::PrefixSum3D;

    #[test]
    fn empty_box_does_nothing() {
        let mut ps = PrefixSum3D::<i64>::new(4, 4, 4);
        ps.add_box(2, 2, 1, 3, 0, 4, 7); // x幅0
        ps.build();
        assert_eq!(ps.total_sum(), 0);
        assert_eq!(ps.range_sum(..0, ..0, ..0), 0); // 空
        assert_eq!(ps.range_sum(0..4, 0..4, 0..4), 0);
    }

    #[test]
    fn single_cell_update_and_queries() {
        let mut ps = PrefixSum3D::<i64>::new(3, 3, 3);
        ps.add_box(1, 2, 1, 2, 1, 2, 5);
        ps.build();

        assert_eq!(ps.range_sum(1..2, 1..2, 1..2), 5);
        assert_eq!(ps.range_sum(0..3, 0..3, 0..3), 5);
        assert_eq!(ps.range_sum(1..2, 1..2, 2..3), 0);
        assert_eq!(ps.range_sum(2..2, 0..3, 0..3), 0);
        assert_eq!(ps.range_sum(0..1, 0..3, 0..3), 0);
    }

    #[test]
    fn whole_box_update_total_sum() {
        let x = 4usize;
        let y = 5usize;
        let z = 3usize;
        let mut ps = PrefixSum3D::<i64>::new(x, y, z);
        ps.add_box(0, x, 0, y, 0, z, 1);
        ps.build();

        assert_eq!(ps.total_sum(), (x * y * z) as i64);
        assert_eq!(ps.range_sum(0..x, 0..y, 0..z), (x * y * z) as i64);

        assert_eq!(ps.range_sum(0..x, 0..y, 0..1), (x * y) as i64);
        assert_eq!(ps.range_sum(0..1, 0..y, 0..z), (y * z) as i64);
        assert_eq!(ps.range_sum(0..x, 0..1, 0..z), (x * z) as i64);
    }

    #[test]
    fn overlapping_boxes_add_up() {
        // 箱A: [0,3)×[0,2)×[0,2) に +2
        // 箱B: [1,4)×[1,3)×[1,2) に -1
        let mut ps = PrefixSum3D::<i64>::new(4, 3, 2);
        ps.add_box(0, 3, 0, 2, 0, 2, 2);
        ps.add_box(1, 4, 1, 3, 1, 2, -1);
        ps.build();

        // 交差領域: x=1..3(2) * y=1..2(1) * z=1..2(1) = 2
        // そこは 2 + (-1) = 1
        assert_eq!(ps.range_sum(1..3, 1..2, 1..2), 2_i64 /*cells*/);

        assert_eq!(ps.range_sum(0..1, 0..2, 0..2), 4 * 2);

        assert_eq!(ps.range_sum(3..4, 2..3, 1..2), -1);

        assert_eq!(ps.total_sum(), 18);
    }

    #[test]
    fn from_vec_small_hand_check() {
        let a = vec![
            vec![vec![1i64, 0], vec![0, 0]],
            vec![vec![0i64, 0], vec![0, 5]],
        ];
        let ps = PrefixSum3D::from_vec(&a);

        assert_eq!(ps.total_sum(), 6);
        assert_eq!(ps.range_sum(0..1, 0..1, 0..1), 1);
        assert_eq!(ps.range_sum(1..2, 1..2, 1..2), 5);
        assert_eq!(ps.range_sum(0..2, 0..2, 0..2), 6);
        assert_eq!(ps.range_sum(0..2, 0..2, 0..1), 1);
    }

    fn naive_apply_boxes(x: usize, y: usize, z: usize) -> Vec<Vec<Vec<i64>>> {
        let mut a = vec![vec![vec![0i64; z]; y]; x];

        let boxes: &[(usize, usize, usize, usize, usize, usize, i64)] = &[
            (0, x, 0, y, 0, z, 1),
            (0, x.min(2), 0, y, 0, z, 3),
            (x.saturating_sub(2), x, 0, y.min(3), 0, z, -2),
            (0, x.min(1), 0, y.min(1), 0, z.min(1), 7),
            (1.min(x), x, 1.min(y), y, 1.min(z), z, -1),
            (0, x, 0, y, 0, z.min(1), 5),
            (0, 0, 0, y, 0, z, 123),
            (0, x, 2.min(y), 2.min(y), 0, z, -999),
        ];

        for &(x0, x1, y0, y1, z0, z1, v) in boxes {
            for yz in a.iter_mut().take(x1).skip(x0) {
                for z_row in yz.iter_mut().take(y1).skip(y0) {
                    for cell in z_row.iter_mut().take(z1).skip(z0) {
                        *cell += v;
                    }
                }
            }
        }
        a
    }

    fn naive_range_sum(
        a: &[Vec<Vec<i64>>],
        x0: usize,
        x1: usize,
        y0: usize,
        y1: usize,
        z0: usize,
        z1: usize,
    ) -> i64 {
        let mut s = 0i64;
        for yz in a.iter().take(x1).skip(x0) {
            for z_row in yz.iter().take(y1).skip(y0) {
                for &cell in z_row.iter().take(z1).skip(z0) {
                    s += cell;
                }
            }
        }
        s
    }

    #[test]
    fn exhaustive_0_to_5_multi_boxes() {
        for x in 0..=5usize {
            for y in 0..=5usize {
                for z in 0..=5usize {
                    let mut ps = PrefixSum3D::<i64>::new(x, y, z);

                    ps.add_box(0, x, 0, y, 0, z, 1);
                    ps.add_box(0, x.min(2), 0, y, 0, z, 3);
                    ps.add_box(x.saturating_sub(2), x, 0, y.min(3), 0, z, -2);
                    ps.add_box(0, x.min(1), 0, y.min(1), 0, z.min(1), 7);
                    ps.add_box(1.min(x), x, 1.min(y), y, 1.min(z), z, -1);
                    ps.add_box(0, x, 0, y, 0, z.min(1), 5);

                    ps.add_box(0, 0, 0, y, 0, z, 123);
                    ps.add_box(0, x, 2.min(y), 2.min(y), 0, z, -999);

                    ps.build();

                    let a = naive_apply_boxes(x, y, z);

                    for x0 in 0..=x {
                        for x1 in x0..=x {
                            for y0 in 0..=y {
                                for y1 in y0..=y {
                                    for z0 in 0..=z {
                                        for z1 in z0..=z {
                                            let got = ps.range_sum(x0..x1, y0..y1, z0..z1);
                                            let exp = naive_range_sum(&a, x0, x1, y0, y1, z0, z1);
                                            assert_eq!(
                                                got, exp,
                                                "mismatch at size=({x},{y},{z}) range=({x0}..{x1},{y0}..{y1},{z0}..{z1})"
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let exp_total = naive_range_sum(&a, 0, x, 0, y, 0, z);
                    assert_eq!(
                        ps.total_sum(),
                        exp_total,
                        "total mismatch size=({x},{y},{z})"
                    );
                }
            }
        }
    }
}
