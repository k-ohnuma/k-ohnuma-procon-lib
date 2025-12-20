use num_traits::{FromPrimitive, PrimInt};

pub fn factorization<T: PrimInt + FromPrimitive>(mut x: T) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    if x <= T::one() {
        return res;
    }
    for i in 2usize.. {
        let it = T::from_usize(i).unwrap();
        if it > x / it {
            break;
        }
        if x % it != T::zero() {
            continue;
        }
        let mut c = 0;
        while x % it == T::zero() {
            c += 1usize;
            x = x / it;
        }
        res.push((it, c));
    }

    if x != T::one() {
        res.push((x, 1));
    }

    res
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use crate::math::factorization::factorization;

    #[test]
    fn small_cases() {
        assert_eq!(factorization::<u64>(1), vec![]);
        assert_eq!(factorization::<u64>(2), vec![(2, 1)]);
        assert_eq!(factorization::<u64>(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factorization::<u64>(999983), vec![(999983, 1)]);
    }

    fn ref_factorization(mut x: u64) -> Vec<(u64, usize)> {
        let mut res = vec![];
        if x <= 1 {
            return res;
        }
        let mut p = 2u64;
        while p * p <= x {
            if x % p != 0 {
                p += 1;
                continue;
            }
            let mut e = 0usize;
            while x % p == 0 {
                x /= p;
                e += 1;
            }
            res.push((p, e));
            p += 1;
        }
        if x != 1 {
            res.push((x, 1));
        }
        res
    }

    #[test]
    fn random_match_reference() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..500 {
            let x: u64 = rng.random_range(0..=2_000_000_000_000u64);
            assert_eq!(factorization::<u64>(x), ref_factorization(x), "x={x}");
        }
    }
}
