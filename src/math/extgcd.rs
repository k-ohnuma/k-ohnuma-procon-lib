use num_traits::{PrimInt, Signed};

// returns (g, x, y): a*x + b*y = g, g >= 0
pub fn extgcd<T>(a: T, b: T) -> (T, T, T)
where
    T: Signed + PrimInt,
{
    if b == T::zero() {
        let g = a.abs();
        let x = if a >= T::zero() { T::one() } else { -T::one() };
        return (g, x, T::zero());
    }
    let (g, x1, y1) = extgcd(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (g, x, y)
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::math::extgcd::extgcd;

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }

    #[test]
    fn random_i64() {
        let mut rng = rand::rng();

        for _ in 0..200_000 {
            let a: i64 = rng.random();
            let b: i64 = rng.random();

            let (g, x, y) = extgcd(a, b);

            assert_eq!(g, gcd(a, b));
            assert_eq!(a.wrapping_mul(x).wrapping_add(b.wrapping_mul(y)), g);
        }
    }

    #[test]
    fn edge_cases() {
        let cases = [
            (0, 0),
            (0, 5),
            (5, 0),
            (-5, 0),
            (0, -5),
            (-12, 18),
            (12, -18),
            (-12, -18),
        ];

        for (a, b) in cases {
            let (g, x, y) = extgcd(a, b);
            assert_eq!(a * x + b * y, g);
        }
    }
}
