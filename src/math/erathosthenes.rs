use num_traits::{FromPrimitive, ToPrimitive};

pub struct Eratosthenes<T> {
    pub sosus: Vec<T>,
    pub first_soinsu: Vec<T>,
}

impl<T> Eratosthenes<T>
where
    T: ToPrimitive + FromPrimitive + Copy + Clone,
{
    pub fn new(max_size: T) -> Self {
        let leng = max_size.to_usize().unwrap();
        let mut furui = vec![true; leng + 1];
        let mut first = vec![T::from_i8(1).unwrap(); leng + 1];
        let mut primes = vec![];
        furui[0] = false;
        furui[1] = false;

        for i in 2..=leng {
            if !furui[i] {
                continue;
            }
            let ans = T::from_usize(i).unwrap();
            primes.push(ans);
            first[i] = ans;
            let mut now = i * i;
            if now > leng {
                continue;
            }
            loop {
                if now > leng {
                    break;
                }
                furui[now] = false;
                first[now] = ans;
                now += i;
            }
        }
        Self {
            sosus: primes,
            first_soinsu: first,
        }
    }

    pub fn factorization(&self, num: T) -> Vec<T> {
        let mut ans = vec![];
        let mut now = num.to_usize().unwrap();
        if now >= self.first_soinsu.len() {
            panic!("高望みするな")
        }
        loop {
            let ne = self.first_soinsu[now];
            ans.push(ne);
            now /= ne.to_usize().unwrap();
            if now == 1 {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn is_prime_naive(x: usize) -> bool {
        if x < 2 {
            return false;
        }
        let mut d = 2;
        while d * d <= x {
            if x % d == 0 {
                return false;
            }
            d += 1;
        }
        true
    }

    fn factorize_naive(mut x: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut p = 2;
        while p * p <= x {
            while x % p == 0 {
                res.push(p);
                x /= p;
            }
            p += 1;
        }
        if x >= 2 {
            res.push(x);
        }
        res
    }

    fn mul(v: &[usize]) -> usize {
        v.iter().product()
    }

    #[test]
    fn test_primes_list_basic() {
        let e = Eratosthenes::<usize>::new(50);
        let primes = e.sosus;
        assert_eq!(
            primes,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }

    #[test]
    fn test_furui_primality_consistency() {
        let n = 2000usize;
        let e = Eratosthenes::<usize>::new(n);

        for &p in &e.sosus {
            assert!(is_prime_naive(p), "listed non-prime: {p}");
        }

        for x in 2..=n {
            let in_list = e.sosus.binary_search(&x).is_ok();
            assert_eq!(in_list, is_prime_naive(x), "mismatch at {x}");
        }
    }

    #[test]
    fn test_first_soinsu_divides_and_is_prime() {
        let n = 5000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 2..=n {
            let f = e.first_soinsu[x];
            assert!(f >= 2, "first factor must be >=2 for x={x}, got {f}");
            assert_eq!(x % f, 0, "first factor does not divide: x={x}, f={f}");
            assert!(is_prime_naive(f), "first factor is not prime: x={x}, f={f}");
        }
    }

    #[test]
    fn test_factorization_matches_product_and_prime_factors() {
        let n = 20000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 2..=n {
            let fs = e.factorization(x);
            assert_eq!(mul(&fs), x, "product mismatch: x={x}, fs={fs:?}");
            for &p in &fs {
                assert!(is_prime_naive(p), "non-prime factor: x={x}, p={p}");
                assert_eq!(x % p, 0, "factor does not divide original: x={x}, p={p}");
            }
        }
    }

    #[test]
    fn test_factorization_random_compare_multiset() {
        let n = 200000usize;
        let e = Eratosthenes::<usize>::new(n);

        let mut rng = StdRng::seed_from_u64(12345);
        for _ in 0..50_000 {
            let x = rng.random_range(2..=n);
            let mut a = e.factorization(x);
            let mut b = factorize_naive(x);
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b, "mismatch: x={x}");
        }
    }
}
