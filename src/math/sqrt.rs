use num_traits::int::PrimInt;
pub fn isqrt<T: PrimInt>(v: T) -> T {
    assert!(v >= T::zero());
    if v == T::zero() {
        return T::zero();
    }
    let one = T::one();
    if v == one {
        return one;
    }

    let two = one + one;

    let mut ok = T::zero();
    let mut ng = v / two + one;

    while ng - ok > one {
        let mid = ok + (ng - ok) / two;
        if mid <= v / mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::isqrt;
    use rand::Rng;

    fn assert_isqrt_ok_u<T>(v: T)
    where
        T: num_traits::int::PrimInt + std::fmt::Debug,
    {
        assert!(v >= T::zero());

        let x = isqrt(v);

        let xx = x
            .checked_mul(&x)
            .expect("x*x overflow: isqrt returned too large?");
        assert!(xx <= v, "x^2 <= v failed: v={v:?}, x={x:?}, x^2={xx:?}");

        let one = T::one();
        if let Some(x1) = x.checked_add(&one) {
            if let Some(x1x1) = x1.checked_mul(&x1) {
                assert!(
                    v < x1x1,
                    "v < (x+1)^2 failed: v={v:?}, x={x:?}, (x+1)^2={x1x1:?}"
                );
            }
        }
    }

    #[test]
    fn edge_cases() {
        // u64
        assert_eq!(isqrt(0u64), 0);
        assert_eq!(isqrt(1u64), 1);
        assert_eq!(isqrt(2u64), 1);
        assert_eq!(isqrt(3u64), 1);
        assert_eq!(isqrt(4u64), 2);
        assert_isqrt_ok_u(u64::MAX);
        assert_isqrt_ok_u(u64::MAX - 1);

        assert_eq!(isqrt(0i64), 0);
        assert_eq!(isqrt(1i64), 1);
        assert_eq!(isqrt(2i64), 1);
        assert_isqrt_ok_u(i64::MAX);
    }

    #[test]
    fn random_u32() {
        let mut rng = rand::rng();
        for _ in 0..200_000 {
            let v: u32 = rng.random();
            assert_isqrt_ok_u(v);
        }
    }

    #[test]
    fn random_u64() {
        let mut rng = rand::rng();
        for _ in 0..200_000 {
            let v: u64 = rng.random();
            assert_isqrt_ok_u(v);
        }
    }

    #[test]
    fn random_u128() {
        let mut rng = rand::rng();
        for _ in 0..200_000 {
            let v: u128 = rng.random();
            assert_isqrt_ok_u(v);
        }
    }

    #[test]
    fn random_i64_nonneg() {
        let mut rng = rand::rng();
        for _ in 0..200_000 {
            let v: i64 = rng.random_range(0..=i64::MAX);
            assert_isqrt_ok_u(v);
        }
    }

    #[test]
    #[should_panic]
    fn negative_panics() {
        let _ = isqrt(-1i64);
    }
}
