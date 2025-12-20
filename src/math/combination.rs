use ac_library::modint::ModIntBase;
use num_traits::{PrimInt, Unsigned};

pub fn nck<T>(n: T, k: T) -> T
where
    T: PrimInt + Unsigned,
{
    if k > n {
        return T::zero();
    }

    let k = {
        let nk = n - k;
        if k > nk {
            nk
        } else {
            k
        }
    };

    let mut result = T::one();
    let mut i = T::zero();

    while i < k {
        // result = result * (n - i) / (i + 1)
        result = result * (n - i) / (i + T::one());
        i = i + T::one();
    }

    result
}

pub struct Comb<Mint> {
    pub fact: Vec<Mint>,
    pub ifact: Vec<Mint>,
}

impl<Mint: ModIntBase> Comb<Mint> {
    pub fn new(nmax: usize) -> Self {
        let mut fact = vec![Mint::new(1); nmax + 1];
        let mut ifact = vec![Mint::new(0); nmax + 1];
        for i in 1..=nmax {
            fact[i] = fact[i - 1] * i.into();
        }
        ifact[nmax] = fact[nmax].inv();
        for i in (1..=nmax).rev() {
            ifact[i - 1] = ifact[i] * i.into();
        }
        Self { fact, ifact }
    }

    pub fn nck(&self, n: usize, k: usize) -> Mint {
        if n < k {
            return Mint::new(0);
        }
        return self.fact[n] * self.ifact[k] * self.ifact[n - k];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::modint::{ModInt1000000007, ModInt998244353};
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn nck_ref_u64(n: u64, k: u64) -> u64 {
        if k > n {
            return 0;
        }
        let k = k.min(n - k);

        let mut numers: Vec<u128> = (0..k).map(|i| (n - i) as u128).collect();
        let mut denoms: Vec<u128> = (1..=k).map(|i| i as u128).collect();

        // 分母を素因数分解的に潰す（gcdで約分）
        for d in &mut denoms {
            if *d == 1 {
                continue;
            }
            for a in &mut numers {
                if *d == 1 {
                    break;
                }
                let g = gcd_u128(*a, *d);
                if g > 1 {
                    *a /= g;
                    *d /= g;
                }
            }
            debug_assert_eq!(*d, 1);
        }

        let mut res: u128 = 1;
        for a in numers {
            res *= a;
        }
        res as u64
    }

    fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    #[test]
    fn test_nck_small_exact_table() {
        assert_eq!(nck(0u64, 0u64), 1);
        assert_eq!(nck(5u64, 0u64), 1);
        assert_eq!(nck(5u64, 1u64), 5);
        assert_eq!(nck(5u64, 2u64), 10);
        assert_eq!(nck(5u64, 3u64), 10);
        assert_eq!(nck(5u64, 4u64), 5);
        assert_eq!(nck(5u64, 5u64), 1);
        assert_eq!(nck(5u64, 6u64), 0);
    }

    #[test]
    fn test_nck_symmetry_and_pascal_no_overflow_range() {
        for n in 0u128..=66 {
            for k in 0u128..=n {
                let a = nck(n, k);
                let b = nck(n, n - k);
                assert_eq!(a, b, "symmetry failed: n={n}, k={k}");

                if 0 < k && k < n {
                    let lhs = nck(n, k);
                    let rhs = nck(n - 1, k - 1) + nck(n - 1, k);
                    assert_eq!(lhs, rhs, "pascal failed: n={n}, k={k}");
                }
            }
        }
    }

    #[test]
    fn test_nck_against_reference_random() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..2000 {
            let n: u64 = rng.random_range(0..=60);
            let k: u64 = rng.random_range(0..=80);
            let got = nck(n, k);
            let exp = nck_ref_u64(n, k);
            assert_eq!(got, exp, "mismatch: n={n}, k={k}");
        }
    }

    fn fact_mod<M: ac_library::modint::ModIntBase>(max: usize) -> Vec<M> {
        let mut r = M::new(1);
        let mut v = vec![r];
        for i in 1..=max {
            r *= i.into();
            v.push(r);
        }
        v
    }

    #[test]
    fn test_comb_matches_naive_998244353() {
        type Mint = ModInt998244353;
        let nmax = 2000;
        let comb = Comb::<Mint>::new(nmax);
        let vs: Vec<Mint> = fact_mod(nmax);

        for n in 0..=2000 {
            for k in 0..=n {
                let got = comb.nck(n, k);

                let nf = vs[n];
                let kf = vs[k];
                let nkf = vs[n - k];
                let exp = nf * (kf * nkf).inv();

                assert_eq!(got, exp, "n={n}, k={k}");
            }
        }
        assert_eq!(comb.nck(10, 11).val(), 0);
    }

    #[test]
    fn test_comb_random_1e9p7() {
        type Mint = ModInt1000000007;
        let nmax = 200_000;
        let comb = Comb::<Mint>::new(nmax);

        let mut rng = StdRng::seed_from_u64(2025);
        for _ in 0..50_000 {
            let n = rng.random_range(0..=nmax);
            let k = rng.random_range(0..=nmax);
            let got = comb.nck(n, k);

            if k > n {
                assert_eq!(got.val(), 0);
            } else {
                let exp = comb.fact[n] * comb.ifact[n - k] * comb.ifact[k];
                assert_eq!(got, exp, "n={n}, k={k}");
            }
        }
    }
}
