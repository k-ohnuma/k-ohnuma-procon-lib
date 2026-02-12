pub fn is_prime_miller(n: u64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }
    let pow = |r: u64, mut m: u64| -> u64 {
        let mut t = 1u128;
        let mut s = (r % n) as u128;
        let n = n as u128;
        while m > 0 {
            if m & 1 == 1 {
                t = t * s % n;
            }
            s = s * s % n;
            m >>= 1;
        }
        t as u64
    };
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    const B: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    for &b in B.iter() {
        let mut a = pow(b, d);
        if a <= 1 {
            continue;
        }
        let mut i = 0;
        while i < s && a != n - 1 {
            i += 1;
            a = (a as u128 * a as u128 % n as u128) as u64;
        }
        if i >= s {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_prime_miller;
    use rand::Rng;

    fn is_prime_naive(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n % 2 == 0 {
            return n == 2;
        }
        let mut d = 3u64;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }

    #[test]
    fn small_compare_naive() {
        for n in 0..200_000u64 {
            assert_eq!(is_prime_miller(n), is_prime_naive(n), "n={n}");
        }
    }

    #[test]
    fn random_composites() {
        let mut rng = rand::rng();
        for _ in 0..50_000 {
            let a: u64 = rng.random_range(3..1_000_000) | 1;
            let b: u64 = rng.random_range(3..1_000_000) | 1;
            let n = a * b;
            if n < 2 {
                continue;
            }
            assert!(!is_prime_miller(n), "composite slipped: {a}*{b}={n}");
        }
    }
}
