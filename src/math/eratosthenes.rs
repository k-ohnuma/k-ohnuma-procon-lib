use ac_library::modint::ModIntBase;
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Eratosthenes<T> {
    /// 素数一覧
    pub sosus: Vec<T>,

    /// smallest prime factor
    /// first_soinsu[x] = x を割り切る最小の素数
    /// 0, 1 は番兵として 1
    pub first_soinsu: Vec<T>,
}

impl<T> Eratosthenes<T>
where
    T: ToPrimitive + FromPrimitive + Copy + Clone,
{
    pub fn new(max_size: T) -> Self {
        let leng = max_size.to_usize().unwrap();

        let one = T::from_usize(1).unwrap();

        let mut first = vec![one; leng + 1];
        let mut primes = vec![];

        for i in 2..=leng {
            if first[i].to_usize().unwrap() != 1 {
                continue;
            }

            let p = T::from_usize(i).unwrap();
            primes.push(p);
            first[i] = p;

            if i > leng / i {
                continue;
            }

            let mut now = i * i;
            while now <= leng {
                if first[now].to_usize().unwrap() == 1 {
                    first[now] = p;
                }
                now += i;
            }
        }

        Self {
            sosus: primes,
            first_soinsu: first,
        }
    }

    pub fn primes(&self) -> &[T] {
        &self.sosus
    }

    pub fn is_prime(&self, num: T) -> bool {
        let n = num.to_usize().unwrap();

        if n < 2 {
            return false;
        }

        assert!(n < self.first_soinsu.len(), "高望みするな");

        self.first_soinsu[n].to_usize().unwrap() == n
    }

    /// 重複込み素因数分解
    ///
    /// 例:
    /// 360 -> [2, 2, 2, 3, 3, 5]
    pub fn factorization(&self, num: T) -> Vec<T> {
        let mut ans = vec![];
        let mut now = num.to_usize().unwrap();

        if now == 0 {
            panic!("0 は素因数分解できません");
        }
        if now <= 1 {
            return ans;
        }

        assert!(now < self.first_soinsu.len(), "高望みするな");

        while now > 1 {
            let p = self.first_soinsu[now];
            ans.push(p);
            now /= p.to_usize().unwrap();
        }

        ans
    }

    /// 指数付き素因数分解
    ///
    /// 例:
    /// 360 -> [(2, 3), (3, 2), (5, 1)]
    pub fn factorization_with_count(&self, num: T) -> Vec<(T, usize)> {
        let mut ans = vec![];
        let mut now = num.to_usize().unwrap();

        if now == 0 {
            panic!("0 は素因数分解できません");
        }
        if now <= 1 {
            return ans;
        }

        assert!(now < self.first_soinsu.len(), "高望みするな");

        while now > 1 {
            let p = self.first_soinsu[now];
            let pu = p.to_usize().unwrap();

            let mut count = 0;
            while now % pu == 0 {
                now /= pu;
                count += 1;
            }

            ans.push((p, count));
        }

        ans
    }

    /// factorization_with_count の短い別名
    pub fn factorize(&self, num: T) -> Vec<(T, usize)> {
        self.factorization_with_count(num)
    }

    /// 約数列挙
    ///
    /// 例:
    /// 12 -> [1, 2, 3, 4, 6, 12]
    pub fn divisors(&self, num: T) -> Vec<T> {
        let n = num.to_usize().unwrap();

        assert!(n < self.first_soinsu.len(), "高望みするな");

        if n == 0 {
            panic!("0 の約数列挙はできません");
        }

        let factors = self.factorization_with_count(num);

        let mut divisors = vec![1usize];

        for (p, count) in factors {
            let p = p.to_usize().unwrap();

            let mut next = vec![];
            let mut mul = 1usize;

            for i in 0..=count {
                for &d in divisors.iter() {
                    next.push(d * mul);
                }
                if i != count {
                    mul *= p;
                }
            }

            divisors = next;
        }

        divisors.sort_unstable();

        divisors
            .into_iter()
            .map(|x| T::from_usize(x).unwrap())
            .collect()
    }

    /// 約数の個数
    ///
    /// 例:
    /// 12 = 2^2 * 3^1
    /// 個数 = (2 + 1) * (1 + 1) = 6
    pub fn num_divisors(&self, num: T) -> usize {
        let n = num.to_usize().unwrap();

        assert!(n < self.first_soinsu.len(), "高望みするな");

        if n == 0 {
            panic!("0 の約数個数は定義しません");
        }

        self.factorization_with_count(num)
            .into_iter()
            .map(|(_, count)| count + 1)
            .product()
    }

    /// 約数の総和
    ///
    /// 例:
    /// 12 の約数は 1, 2, 3, 4, 6, 12
    /// sum = 28
    ///
    /// 戻り値は overflow 避けで u128 にしています。
    pub fn sum_divisors(&self, num: T) -> u128 {
        let n = num.to_usize().unwrap();

        assert!(n < self.first_soinsu.len(), "高望みするな");

        if n == 0 {
            panic!("0 の約数和は定義しません");
        }

        let mut ans = 1u128;

        for (p, count) in self.factorization_with_count(num) {
            let p = p.to_u128().unwrap();

            let mut now = 1u128;
            let mut term = 1u128;

            for _ in 0..count {
                term *= p;
                now += term;
            }

            ans *= now;
        }

        ans
    }
}

impl<T> Eratosthenes<T>
where
    T: ToPrimitive + FromPrimitive + Copy + Clone + Eq + Hash,
{
    /// 複数の数の LCM を素因数分解形式で返す
    ///
    /// 例:
    /// [12, 18] = [2^2 * 3, 2 * 3^2]
    /// LCM = 2^2 * 3^2
    ///
    /// 戻り値:
    /// {2: 2, 3: 2}
    pub fn lcm_factorization(&self, nums: &[T]) -> HashMap<T, usize> {
        let mut map = HashMap::new();

        for &num in nums {
            for (p, count) in self.factorization_with_count(num) {
                let entry = map.entry(p).or_insert(0);
                *entry = (*entry).max(count);
            }
        }

        map
    }

    /// 複数の数の LCM を Mint で返す
    ///
    /// 例:
    /// let lcm = era.lcm_mod::<ModInt1000000007>(&a);
    pub fn lcm_mod<Mint>(&self, nums: &[T]) -> Mint
    where
        Mint: ModIntBase,
    {
        let factors = self.lcm_factorization(nums);

        let mut ans = Mint::new(1);

        for (p, count) in factors {
            let p = p.to_u64().unwrap();
            ans *= Mint::new(p).pow(count as u64);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ac_library::ModInt1000000007;
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

    #[test]
    fn test_new_small_sizes() {
        let e0 = Eratosthenes::<usize>::new(0);
        assert!(e0.sosus.is_empty());
        assert_eq!(e0.first_soinsu.len(), 1);

        let e1 = Eratosthenes::<usize>::new(1);
        assert!(e1.sosus.is_empty());
        assert_eq!(e1.first_soinsu.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_factorization_zero_panics() {
        let e = Eratosthenes::<usize>::new(10);
        let _ = e.factorization(0);
    }

    #[test]
    fn test_one() {
        let e = Eratosthenes::<usize>::new(10);
        let e = e.factorization(1);
        assert!(e.is_empty());
    }

    #[test]
    fn test_primes_method() {
        let e = Eratosthenes::<usize>::new(30);
        assert_eq!(e.primes(), &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_is_prime_basic() {
        let e = Eratosthenes::<usize>::new(30);

        assert!(!e.is_prime(0));
        assert!(!e.is_prime(1));
        assert!(e.is_prime(2));
        assert!(e.is_prime(3));
        assert!(!e.is_prime(4));
        assert!(e.is_prime(5));
        assert!(!e.is_prime(9));
        assert!(e.is_prime(29));
        assert!(!e.is_prime(30));
    }

    #[test]
    fn test_is_prime_matches_naive() {
        let n = 10000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 0..=n {
            assert_eq!(e.is_prime(x), is_prime_naive(x), "mismatch at {x}");
        }
    }

    #[test]
    fn test_factorization_with_count_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        assert_eq!(e.factorization_with_count(1), vec![]);
        assert_eq!(e.factorization_with_count(2), vec![(2, 1)]);
        assert_eq!(e.factorization_with_count(12), vec![(2, 2), (3, 1)]);
        assert_eq!(
            e.factorization_with_count(360),
            vec![(2, 3), (3, 2), (5, 1)]
        );
        assert_eq!(e.factorization_with_count(997), vec![(997, 1)]);
    }

    #[test]
    fn test_factorize_alias() {
        let e = Eratosthenes::<usize>::new(1000);

        for x in 1..=1000 {
            assert_eq!(e.factorize(x), e.factorization_with_count(x));
        }
    }

    #[test]
    fn test_factorization_with_count_matches_factorization() {
        let n = 20000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 1..=n {
            let expanded = e
                .factorization_with_count(x)
                .into_iter()
                .flat_map(|(p, c)| std::iter::repeat(p).take(c))
                .collect::<Vec<_>>();

            assert_eq!(expanded, e.factorization(x), "mismatch at {x}");
        }
    }

    fn divisors_naive(x: usize) -> Vec<usize> {
        let mut res = vec![];

        for d in 1..=x {
            if x % d == 0 {
                res.push(d);
            }
        }

        res
    }

    #[test]
    fn test_divisors_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        assert_eq!(e.divisors(1), vec![1]);
        assert_eq!(e.divisors(2), vec![1, 2]);
        assert_eq!(e.divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(e.divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(e.divisors(36), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
    }

    #[test]
    fn test_divisors_matches_naive() {
        let n = 5000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 1..=n {
            assert_eq!(e.divisors(x), divisors_naive(x), "mismatch at {x}");
        }
    }

    #[test]
    fn test_num_divisors_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        assert_eq!(e.num_divisors(1), 1);
        assert_eq!(e.num_divisors(2), 2);
        assert_eq!(e.num_divisors(6), 4);
        assert_eq!(e.num_divisors(12), 6);
        assert_eq!(e.num_divisors(36), 9);
        assert_eq!(e.num_divisors(997), 2);
    }

    #[test]
    fn test_num_divisors_matches_divisors_len() {
        let n = 10000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 1..=n {
            assert_eq!(e.num_divisors(x), e.divisors(x).len(), "mismatch at {x}");
        }
    }

    #[test]
    fn test_sum_divisors_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        assert_eq!(e.sum_divisors(1), 1);
        assert_eq!(e.sum_divisors(2), 3);
        assert_eq!(e.sum_divisors(6), 12);
        assert_eq!(e.sum_divisors(12), 28);
        assert_eq!(e.sum_divisors(36), 91);
        assert_eq!(e.sum_divisors(997), 998);
    }

    #[test]
    fn test_sum_divisors_matches_divisors_sum() {
        let n = 10000usize;
        let e = Eratosthenes::<usize>::new(n);

        for x in 1..=n {
            let expected = e.divisors(x).into_iter().map(|d| d as u128).sum::<u128>();
            assert_eq!(e.sum_divisors(x), expected, "mismatch at {x}");
        }
    }

    #[test]
    #[should_panic]
    fn test_factorization_with_count_zero_panics() {
        let e = Eratosthenes::<usize>::new(10);
        let _ = e.factorization_with_count(0);
    }

    #[test]
    #[should_panic]
    fn test_divisors_zero_panics() {
        let e = Eratosthenes::<usize>::new(10);
        let _ = e.divisors(0);
    }

    #[test]
    #[should_panic]
    fn test_num_divisors_zero_panics() {
        let e = Eratosthenes::<usize>::new(10);
        let _ = e.num_divisors(0);
    }

    #[test]
    #[should_panic]
    fn test_sum_divisors_zero_panics() {
        let e = Eratosthenes::<usize>::new(10);
        let _ = e.sum_divisors(0);
    }

    fn gcd_naive(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn lcm_naive(a: usize, b: usize) -> usize {
        a / gcd_naive(a, b) * b
    }

    fn lcm_all_naive(nums: &[usize]) -> usize {
        nums.iter().fold(1usize, |acc, &x| lcm_naive(acc, x))
    }

    #[test]
    fn test_lcm_factorization_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        let map = e.lcm_factorization(&[12, 18]);

        assert_eq!(map.get(&2), Some(&2));
        assert_eq!(map.get(&3), Some(&2));
        assert_eq!(map.len(), 2);

        let map = e.lcm_factorization(&[1, 1, 1]);
        assert!(map.is_empty());

        let map = e.lcm_factorization(&[8, 9, 25]);
        assert_eq!(map.get(&2), Some(&3));
        assert_eq!(map.get(&3), Some(&2));
        assert_eq!(map.get(&5), Some(&2));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_lcm_factorization_reconstructs_lcm() {
        let n = 200usize;
        let e = Eratosthenes::<usize>::new(n);

        let cases = vec![
            vec![1],
            vec![1, 1, 1],
            vec![2, 3, 5],
            vec![12, 18],
            vec![8, 9, 25],
            vec![6, 10, 15],
            vec![24, 36, 48],
        ];

        for nums in cases {
            let map = e.lcm_factorization(&nums);

            let mut reconstructed = 1usize;
            for (p, c) in map {
                reconstructed *= p.pow(c as u32);
            }

            assert_eq!(
                reconstructed,
                lcm_all_naive(&nums),
                "mismatch nums={nums:?}"
            );
        }
    }


    #[test]
    fn test_lcm_mod_basic() {
        let e = Eratosthenes::<usize>::new(1000);

        let lcm = e.lcm_mod::<ModInt1000000007>(&[12, 18]);
        assert_eq!(lcm.val(), 36);

        let lcm = e.lcm_mod::<ModInt1000000007>(&[1, 1, 1]);
        assert_eq!(lcm.val(), 1);

        let lcm = e.lcm_mod::<ModInt1000000007>(&[8, 9, 25]);
        assert_eq!(lcm.val(), 1800);
    }

    #[test]
    fn test_lcm_mod_matches_naive_small() {
        let n = 100usize;
        let e = Eratosthenes::<usize>::new(n);

        let mut rng = StdRng::seed_from_u64(67890);

        for _ in 0..10_000 {
            let len = rng.random_range(1..=8);
            let nums = (0..len)
                .map(|_| rng.random_range(1..=n))
                .collect::<Vec<_>>();

            let expected = lcm_all_naive(&nums) % 1_000_000_007;
            let actual = e.lcm_mod::<ModInt1000000007>(&nums).val() as usize;

            assert_eq!(actual, expected, "nums={nums:?}");
        }
    }
}
