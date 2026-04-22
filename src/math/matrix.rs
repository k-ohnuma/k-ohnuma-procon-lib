/// 2x2 Matrix exponentiation library for affine / linear recurrences.
///
/// Typical use cases:
/// - Fibonacci
/// - repeated affine transform: x -> a*x + b (mod m)
/// - digit concatenation transition: x -> 10*x + c (mod m)
///
/// For the digit transition,
///
///     [x_next]   [10 c] [x]
///     [   1   ] = [ 0 1] [1]
///
/// so repeating the same digit `len` times becomes matrix exponentiation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix2 {
    pub a00: i64,
    pub a01: i64,
    pub a10: i64,
    pub a11: i64,
}

impl Matrix2 {
    pub fn new(a00: i64, a01: i64, a10: i64, a11: i64, modu: i64) -> Self {
        assert!(modu > 0);
        Self {
            a00: a00.rem_euclid(modu),
            a01: a01.rem_euclid(modu),
            a10: a10.rem_euclid(modu),
            a11: a11.rem_euclid(modu),
        }
    }

    pub fn identity() -> Self {
        Self {
            a00: 1,
            a01: 0,
            a10: 0,
            a11: 1,
        }
    }

    pub fn mul(self, rhs: Self, modu: i64) -> Self {
        assert!(modu > 0);
        let m = modu as i128;
        let a00 = ((self.a00 as i128) * (rhs.a00 as i128) + (self.a01 as i128) * (rhs.a10 as i128))
            .rem_euclid(m) as i64;
        let a01 = ((self.a00 as i128) * (rhs.a01 as i128) + (self.a01 as i128) * (rhs.a11 as i128))
            .rem_euclid(m) as i64;
        let a10 = ((self.a10 as i128) * (rhs.a00 as i128) + (self.a11 as i128) * (rhs.a10 as i128))
            .rem_euclid(m) as i64;
        let a11 = ((self.a10 as i128) * (rhs.a01 as i128) + (self.a11 as i128) * (rhs.a11 as i128))
            .rem_euclid(m) as i64;
        Self { a00, a01, a10, a11 }
    }

    pub fn pow(mut self, mut exp: u64, modu: i64) -> Self {
        let mut res = Matrix2::identity();
        while exp > 0 {
            if exp & 1 == 1 {
                res = res.mul(self, modu);
            }
            self = self.mul(self, modu);
            exp >>= 1;
        }
        res
    }

    /// Applies the matrix to a column vector [x, y]^T.
    pub fn apply_vec2(self, x: i64, y: i64, modu: i64) -> (i64, i64) {
        assert!(modu > 0);
        let m = modu as i128;
        (
            ((self.a00 as i128) * (x as i128) + (self.a01 as i128) * (y as i128)).rem_euclid(m)
                as i64,
            ((self.a10 as i128) * (x as i128) + (self.a11 as i128) * (y as i128)).rem_euclid(m)
                as i64,
        )
    }
}

/// Repeats the digit transition
///
///     x -> 10*x + digit (mod modu)
///
/// exactly `len` times using matrix exponentiation.
pub fn repeat_digit_mod_matrix(x: i64, digit: i64, len: u64, modu: i64) -> i64 {
    let mat = Matrix2::new(10, digit, 0, 1, modu).pow(len, modu);
    let (nx, _) = mat.apply_vec2(x, 1, modu);
    nx
}

/// Repeats a general affine transition
///
///     x -> mul*x + add (mod modu)
///
/// exactly `times` times using matrix exponentiation.
pub fn repeat_affine_mod_matrix(x: i64, mul: i64, add: i64, times: u64, modu: i64) -> i64 {
    let mat = Matrix2::new(mul, add, 0, 1, modu).pow(times, modu);
    let (nx, _) = mat.apply_vec2(x, 1, modu);
    nx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_repeat_digit(mut x: i64, digit: i64, len: u64, modu: i64) -> i64 {
        for _ in 0..len {
            x = (10 * x + digit).rem_euclid(modu);
        }
        x
    }

    fn naive_repeat_affine(mut x: i64, mul: i64, add: i64, times: u64, modu: i64) -> i64 {
        for _ in 0..times {
            x = (mul * x + add).rem_euclid(modu);
        }
        x
    }

    #[test]
    fn test_matrix_mul_identity() {
        let modu = 1_000_000_007;
        let m = Matrix2::new(3, 5, 7, 11, modu);
        assert_eq!(m.mul(Matrix2::identity(), modu), m);
        assert_eq!(Matrix2::identity().mul(m, modu), m);
    }

    #[test]
    fn test_repeat_digit_mod_matrix_small() {
        for modu in 1..=50 {
            for digit in 0..=9 {
                for len in 0..=100u64 {
                    for x in 0..modu {
                        let got = repeat_digit_mod_matrix(x, digit, len, modu);
                        let want = naive_repeat_digit(x, digit, len, modu);
                        assert_eq!(got, want, "modu={modu}, digit={digit}, len={len}, x={x}");
                    }
                }
            }
        }
    }

    #[test]
    fn test_repeat_affine_mod_matrix_small() {
        for modu in 1..=50 {
            for mul in 0..=15 {
                for add in 0..=15 {
                    for times in 0..=60u64 {
                        for x in 0..modu {
                            let got = repeat_affine_mod_matrix(x, mul, add, times, modu);
                            let want = naive_repeat_affine(x, mul, add, times, modu);
                            assert_eq!(
                                got, want,
                                "modu={modu}, mul={mul}, add={add}, times={times}, x={x}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_fibonacci_example() {
        // [F_{n+1}, F_n]^T = [[1,1],[1,0]]^n [F_1, F_0]^T
        let modu = 1_000_000_007;
        let fib = Matrix2::new(1, 1, 1, 0, modu);
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (n, &want) in expected.iter().enumerate() {
            if n == 0 {
                assert_eq!(want, 0);
                continue;
            }
            let p = fib.pow((n - 1) as u64, modu);
            let (fn_, _) = p.apply_vec2(1, 0, modu);
            assert_eq!(fn_, want);
        }
    }

    #[test]
    fn test_matrix_mul_large_mod_no_overflow() {
        let modu = 9_000_000_000_000_000_007i64;
        let a = Matrix2::new(modu - 2, modu - 3, modu - 5, modu - 7, modu);
        let b = Matrix2::new(modu - 11, modu - 13, modu - 17, modu - 19, modu);

        let got = a.mul(b, modu);

        let m = modu as i128;
        let exp_a00 = ((a.a00 as i128) * (b.a00 as i128) + (a.a01 as i128) * (b.a10 as i128))
            .rem_euclid(m) as i64;
        let exp_a01 = ((a.a00 as i128) * (b.a01 as i128) + (a.a01 as i128) * (b.a11 as i128))
            .rem_euclid(m) as i64;
        let exp_a10 = ((a.a10 as i128) * (b.a00 as i128) + (a.a11 as i128) * (b.a10 as i128))
            .rem_euclid(m) as i64;
        let exp_a11 = ((a.a10 as i128) * (b.a01 as i128) + (a.a11 as i128) * (b.a11 as i128))
            .rem_euclid(m) as i64;

        assert_eq!(
            got,
            Matrix2 {
                a00: exp_a00,
                a01: exp_a01,
                a10: exp_a10,
                a11: exp_a11
            }
        );
    }

    #[test]
    fn test_apply_vec2_large_mod_no_overflow() {
        let modu = 9_000_000_000_000_000_007i64;
        let a = Matrix2::new(modu - 2, modu - 3, modu - 5, modu - 7, modu);
        let x = modu - 11;
        let y = modu - 13;

        let got = a.apply_vec2(x, y, modu);
        let m = modu as i128;
        let exp = (
            ((a.a00 as i128) * (x as i128) + (a.a01 as i128) * (y as i128)).rem_euclid(m) as i64,
            ((a.a10 as i128) * (x as i128) + (a.a11 as i128) * (y as i128)).rem_euclid(m) as i64,
        );
        assert_eq!(got, exp);
    }
}
