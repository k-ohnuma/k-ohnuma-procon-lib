use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_integer::Integer;
use num_traits::Signed;

#[derive(Debug, Clone, Copy)]
pub struct Frac<T> {
    a: T,
    b: T,
}

impl<T> Frac<T>
where
    T: Integer + Signed + Copy,
{
    pub fn new(a: T, b: T) -> Self {
        let mut f = Self { a, b };
        f.normalize();
        f
    }

    pub fn from_int(x: T) -> Self {
        Self::new(x, T::one())
    }

    pub fn get(&self) -> (T, T) {
        (self.a, self.b)
    }

    pub fn zero() -> Self {
        Self::new(T::zero(), T::one())
    }

    pub fn pinf() -> Self {
        Self::new(T::one(), T::zero())
    }

    pub fn minf() -> Self {
        Self::new(-T::one(), T::zero())
    }

    pub fn inv(&self) -> Self {
        if self.a.is_zero() {
            return Self::new(T::one(), T::zero());
        }
        Self::new(self.b, self.a)
    }

    fn normalize(&mut self) {
        if self.b.is_zero() {
            if self.a.is_zero() {
                self.a = T::one();
            } else {
                self.a = self.a.signum();
            }
            return;
        }
        let g = self.a.gcd(&self.b);
        self.a = self.a / g;
        self.b = self.b / g;

        if self.b.is_negative() {
            self.a = -self.a;
            self.b = -self.b;
        }
    }
    pub fn is_inf(&self) -> bool {
        self.b.is_zero() && !self.a.is_zero()
    }
    pub fn is_pinf(&self) -> bool {
        self.b.is_zero() && self.a.is_positive()
    }
    pub fn is_minf(&self) -> bool {
        self.b.is_zero() && self.a.is_negative()
    }

    pub fn is_zero(&self) -> bool {
        let a = self.a;
        let b = self.b;
        a.is_zero() && b.is_one()
    }
    pub fn is_positive(&self) -> bool {
        if self.is_pinf() {
            return true;
        }
        if self.is_minf() {
            return false;
        }
        self.a.is_positive()
    }
    pub fn is_negative(&self) -> bool {
        if self.is_minf() {
            return true;
        }
        if self.is_pinf() {
            return false;
        }
        self.a.is_negative()
    }
}

impl<T> From<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn from(value: T) -> Self {
        Self::from_int(value)
    }
}

impl<T> Neg for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let a = self.a;
        let b = self.b;
        Self::new(-a, b)
    }
}

// ── Frac<T> と Frac<T>の計算 ────────────────────────────────────────
impl<T> Add for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_inf() || rhs.is_inf() {
            if self.is_pinf() && rhs.is_minf() || self.is_minf() && rhs.is_pinf() {
                panic!("indeterminate form: +inf + -inf");
            }
            return if self.is_inf() { self } else { rhs };
        }
        let (a, b) = (self.a, self.b);
        let (c, d) = (rhs.a, rhs.b);

        let g = b.gcd(&d);
        let b1 = b / g; // b'
        let d1 = d / g; // d'

        let num = a * d1 + c * b1;
        let den = b1 * d;

        let g2 = num.gcd(&g);
        Self::new(num / g2, den / g2)
    }
}

impl<T> Sub for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T> Mul for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_inf() || rhs.is_inf() {
            if (!self.is_inf() && self.a.is_zero()) || (!rhs.is_inf() && rhs.a.is_zero()) {
                panic!("indeterminate form: 0 * inf");
            }
            let s = self.a.signum() * rhs.a.signum();
            return Self::new(s, T::zero());
        }

        let (mut a, mut b) = (self.a, self.b);
        let (mut c, mut d) = (rhs.a, rhs.b);

        let g1 = a.gcd(&d);
        a = a / g1;
        d = d / g1;

        let g2 = c.gcd(&b);
        c = c / g2;
        b = b / g2;

        Self::new(a * c, b * d)
    }
}

impl<T> Div for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<T> PartialEq for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
impl<T> Eq for Frac<T> where T: Integer + Signed + Copy {}

impl<T> PartialOrd for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.b.is_zero() && other.b.is_zero() {
            return Some(self.a.cmp(&other.a));
        }
        if self.b.is_zero() {
            return Some(if self.a.is_positive() {
                Ordering::Greater
            } else {
                Ordering::Less
            });
        }
        if other.b.is_zero() {
            return Some(if other.a.is_positive() {
                Ordering::Less
            } else {
                Ordering::Greater
            });
        }
        Some((self.a * other.b).cmp(&(other.a * self.b)))
    }
}

impl<T> Ord for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// ── Frac<T>とTの計算 ────────────────────────────────────────────────
impl<T> Add<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = Self::from_int(rhs);
        self + rhs
    }
}

impl<T> Sub<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Self::from_int(rhs);
        self - rhs
    }
}

impl<T> Mul<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Self::from_int(rhs);
        self * rhs
    }
}

impl<T> Div<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = Self::from_int(rhs);
        let inv = rhs.inv();
        self * inv
    }
}

impl<T> PartialEq<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn eq(&self, other: &T) -> bool {
        let other = Self::from_int(*other);
        self.a == other.a && self.b == other.b
    }
}

impl<T> PartialOrd<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other = Self::from_int(*other);
        if self.b.is_zero() && other.b.is_zero() {
            return Some(self.a.cmp(&other.a));
        }
        if self.b.is_zero() {
            return Some(if self.a.is_positive() {
                Ordering::Greater
            } else {
                Ordering::Less
            });
        }
        if other.b.is_zero() {
            return Some(if other.a.is_positive() {
                Ordering::Less
            } else {
                Ordering::Greater
            });
        }
        Some((self.a * other.b).cmp(&(other.a * self.b)))
    }
}

// Assign関連
impl<T> AddAssign for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        let t = self.to_owned() + rhs;
        *self = t;
    }
}
impl<T> SubAssign for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}
impl<T> MulAssign for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        let t = self.to_owned() * rhs;
        *self = t
    }
}
impl<T> DivAssign for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        let inv = rhs.inv();
        *self *= inv
    }
}
impl<T> AddAssign<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = Self::from_int(rhs);
        *self += rhs;
    }
}
impl<T> SubAssign<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = Self::from_int(rhs);
        *self -= rhs;
    }
}
impl<T> MulAssign<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = Self::from_int(rhs);
        *self *= rhs
    }
}
impl<T> DivAssign<T> for Frac<T>
where
    T: Integer + Signed + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = Self::from_int(rhs);
        *self /= rhs
    }
}

#[cfg(test)]
mod tests {

    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::math::fraction::Frac;

    fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn norm_i128(mut a: i128, mut b: i128) -> (i128, i128) {
        if b == 0 {
            let s = if a > 0 { 1 } else { -1 };
            return (s, 0);
        }
        if a == 0 {
            return (0, 1);
        }
        let g = gcd_i128(a, b);
        a /= g;
        b /= g;
        if b < 0 {
            a = -a;
            b = -b;
        }
        (a, b)
    }

    fn frac_to_i128(f: Frac<i64>) -> (i128, i128) {
        let (a, b) = f.get();
        (a as i128, b as i128)
    }

    fn assert_frac_eq_i128(actual: Frac<i64>, expected: (i128, i128)) {
        let expected = norm_i128(expected.0, expected.1);
        let actual = frac_to_i128(actual);
        assert_eq!(actual, expected, "actual={:?}, expected={:?}", actual, expected);
    }

    fn rand_nonzero_i64(rng: &mut StdRng, lo: i64, hi: i64) -> i64 {
        loop {
            let x = rng.random_range(lo..=hi);
            if x != 0 {
                return x;
            }
        }
    }

    #[test]
    fn normalize_basic() {
        assert_eq!(Frac::new(2_i64, 4).get(), (1, 2));
        assert_eq!(Frac::new(-2_i64, 4).get(), (-1, 2));
        assert_eq!(Frac::new(2_i64, -4).get(), (-1, 2));
        assert_eq!(Frac::new(-2_i64, -4).get(), (1, 2));

        assert_eq!(Frac::new(0_i64, 5).get(), (0, 1));
        assert_eq!(Frac::new(0_i64, -5).get(), (0, 1));

        assert_eq!(Frac::pinf().get(), (1, 0));
        assert_eq!(Frac::minf().get(), (-1, 0));
        let pinf = Frac::<i32>::pinf();
        let minf = Frac::<i32>::minf();
        assert!(pinf.is_pinf());
        assert!(minf.is_minf());
        assert!(pinf.is_inf());
        assert!(minf.is_inf());
    }

    #[test]
    fn inv_basic() {
        let f = Frac::new(3_i64, 4);
        assert_eq!(f.inv().get(), (4, 3));

        let g = Frac::new(-3_i64, 4);
        assert_eq!(g.inv().get(), (-4, 3));
        assert_eq!(g.inv().get(), Frac::new(4_i64, -3).get());

        let z = Frac::<i64>::zero();
        assert!(z.inv().is_pinf());
    }

    #[test]
    fn sign_checks() {
        let pinf = Frac::<i32>::pinf();
        let minf = Frac::<i32>::minf();
        let zero = Frac::<i32>::zero();
        assert!(Frac::new(1_i64, 2).is_positive());
        assert!(Frac::new(-1_i64, 2).is_negative());
        assert!(pinf.is_positive());
        assert!(minf.is_negative());
        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
    }

    #[test]
    fn inf_add_finite() {
        let x = Frac::new(7_i64, 3);
        assert_eq!((Frac::pinf() + x).get(), (1, 0));
        assert_eq!((x + Frac::minf()).get(), (-1, 0));
    }

    #[test]
    #[should_panic(expected = "indeterminate form")]
    fn inf_plus_minf_panics() {
        let _ = Frac::<i128>::pinf() + Frac::minf();
    }

    #[test]
    #[should_panic(expected = "indeterminate form")]
    fn zero_times_inf_panics() {
        let _ = Frac::<i128>::zero() * Frac::pinf();
    }

    #[test]
    fn inf_mul_sign() {
        let neg = Frac::new(-3_i64, 4);
        assert!( (Frac::pinf() * neg).is_minf() );
        assert!( (Frac::minf() * neg).is_pinf() );
    }

    #[test]
    fn ord_with_inf() {
        let pinf = Frac::<i32>::pinf();
        let minf = Frac::<i32>::minf();
        let x = Frac::new(123_i64, 7);
        assert!(Frac::pinf() > x);
        assert!(Frac::minf() < x);
        assert!(pinf == Frac::pinf());
        assert!(minf == Frac::minf());
        assert!(minf < Frac::pinf());
    }

    #[test]
    fn random_arithmetic_matches_i128() {
        let mut rng = StdRng::seed_from_u64(0xC0FFE);
        let lo = -1_000_000_i64;
        let hi = 1_000_000_i64;

        for _ in 0..20_000 {
            let a = rng.random_range(lo..=hi);
            let b = rand_nonzero_i64(&mut rng, lo, hi);
            let c = rng.random_range(lo..=hi);
            let d = rand_nonzero_i64(&mut rng, lo, hi);

            let f = Frac::new(a, b);
            let g = Frac::new(c, d);

            let exp_add = (a as i128) * (d as i128) + (c as i128) * (b as i128);
            let exp_add_den = (b as i128) * (d as i128);
            assert_frac_eq_i128(f + g, (exp_add, exp_add_den));

            let exp_sub = (a as i128) * (d as i128) - (c as i128) * (b as i128);
            let exp_sub_den = (b as i128) * (d as i128);
            assert_frac_eq_i128(f - g, (exp_sub, exp_sub_den));

            let exp_mul = (a as i128) * (c as i128);
            let exp_mul_den = (b as i128) * (d as i128);
            assert_frac_eq_i128(f * g, (exp_mul, exp_mul_den));

            if c != 0 {
                let exp_div = (a as i128) * (d as i128);
                let exp_div_den = (b as i128) * (c as i128);
                assert_frac_eq_i128(f / g, (exp_div, exp_div_den));
            }

            if a == 0 {
                assert!((f.inv()).is_pinf());
            } else {
                assert_frac_eq_i128(f.inv(), (b as i128, a as i128));
            }
        }
    }

    #[test]
    fn random_assign_ops_match_nonassign() {
        let mut rng = StdRng::seed_from_u64(0xBADC0DE);

        let lo = -100_000_i64;
        let hi = 100_000_i64;

        for _ in 0..10_000 {
            let a = rng.random_range(lo..=hi);
            let b = rand_nonzero_i64(&mut rng, lo, hi);
            let c = rng.random_range(lo..=hi);
            let d = rand_nonzero_i64(&mut rng, lo, hi);

            let f = Frac::new(a, b);
            let g = Frac::new(c, d);

            // +=
            let mut x = f;
            x += g;
            assert_eq!(x, f + g);

            // -=
            let mut x = f;
            x -= g;
            assert_eq!(x, f - g);

            // *=
            let mut x = f;
            x *= g;
            assert_eq!(x, f * g);

            // /=
            if c != 0 {
                let mut x = f;
                x /= g;
                assert_eq!(x, f / g);
            }

            let t = rng.random_range(lo..=hi);

            let mut x = f;
            x += t;
            assert_eq!(x, f + t);

            let mut x = f;
            x -= t;
            assert_eq!(x, f - t);

            let mut x = f;
            x *= t;
            assert_eq!(x, f * t);

            if t != 0 {
                let mut x = f;
                x /= t;
                assert_eq!(x, f / t);
            }
        }
    }

    #[test]
    fn random_order_matches_i128() {
        let mut rng = StdRng::seed_from_u64(0xDEADBEEF);

        let lo = -1_000_000_i64;
        let hi = 1_000_000_i64;

        for _ in 0..20_000 {
            let a = rng.random_range(lo..=hi);
            let b = rand_nonzero_i64(&mut rng, lo, hi);
            let c = rng.random_range(lo..=hi);
            let d = rand_nonzero_i64(&mut rng, lo, hi);

            let f = Frac::new(a, b);
            let g = Frac::new(c, d);

            let (fa, fb) = frac_to_i128(f);
            let (ga, gb) = frac_to_i128(g);

            let expected = (fa * gb).cmp(&(ga * fb));
            assert_eq!(f.cmp(&g), expected);
            assert_eq!(f.partial_cmp(&g), Some(expected));
        }
    }

    #[test]
    fn random_normal_form_invariant() {
        let mut rng = StdRng::seed_from_u64(0x12345678);

        let lo = -1_000_000_i64;
        let hi = 1_000_000_i64;

        for _ in 0..50_000 {
            let a = rng.random_range(lo..=hi);
            let b = rand_nonzero_i64(&mut rng, lo, hi);
            let f = Frac::new(a, b);
            let (na, nb) = f.get();

            assert!(nb > 0, "denominator must be positive, got {:?}", (na, nb));

            let ga = (na as i128).abs();
            let gb = (nb as i128).abs();
            let g = gcd_i128(ga, gb);
            assert_eq!(g, 1, "not reduced: {:?}", (na, nb));

            if na == 0 {
                assert_eq!(nb, 1);
            }
        }
    }
}

