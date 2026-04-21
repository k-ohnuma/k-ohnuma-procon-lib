// 10^14くらいまでの素数に関して[l, r]の範囲で列挙する
// 区間篩
use num_traits::{FromPrimitive, PrimInt, ToPrimitive, Unsigned};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegSieveError<T> {
    OutOfRange { x: T, l: T, r: T },
}

fn isqrt_u128(x: u128) -> u128 {
    if x <= 1 {
        return x;
    }
    let mut r = (x as f64).sqrt() as u128;
    while (r + 1) * (r + 1) <= x {
        r += 1;
    }
    while r * r > x {
        r -= 1;
    }
    r
}

fn sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            if i <= n / i {
                let mut j = i * i;
                while j <= n {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
    }
    primes
}

#[derive(Clone, Debug)]
pub struct EratosSieve<T>
where
    T: PrimInt + Unsigned + ToPrimitive + FromPrimitive,
{
    l: T,
    r: T,
    low: T,
    low_u: u128,
    flags: Vec<bool>,
}

impl<T> EratosSieve<T>
where
    T: PrimInt + Unsigned + ToPrimitive + FromPrimitive,
{
    pub fn new(l: T, r: T) -> Self {
        assert!(l <= r);

        let two = T::from_u8(2).unwrap();
        let low = if l < two { two } else { l };

        let (low_u, flags) = if r < two || low > r {
            (low.to_u128().unwrap_or(0), Vec::new())
        } else {
            let low_u = low.to_u128().unwrap();
            let r_u = r.to_u128().unwrap();

            let len = (r_u - low_u + 1).to_usize().unwrap();
            let mut flags = vec![true; len];

            let lim = isqrt_u128(r_u).to_usize().unwrap();
            let base_primes = sieve(lim);

            for &p32 in &base_primes {
                let p = p32 as u128;
                let pp = p * p;
                if pp > r_u {
                    break;
                }

                let mut start = ((low_u + p - 1) / p) * p;
                if start == p {
                    start += p;
                }

                let mut x = start;
                while x <= r_u {
                    flags[(x - low_u) as usize] = false;
                    x += p;
                }
            }

            (low_u, flags)
        };

        Self {
            l,
            r,
            low,
            low_u,
            flags,
        }
    }

    /// 素数判定
    /// 範囲外を叩かれたらErr。素数だったらtrueを返す。
    pub fn is_prime(&self, x: T) -> Result<bool, SegSieveError<T>> {
        if x < self.l || x > self.r {
            return Err(SegSieveError::OutOfRange {
                x,
                l: self.l,
                r: self.r,
            });
        }

        if self.flags.is_empty() {
            return Ok(false);
        }

        if x < self.low {
            return Ok(false);
        }

        let xu = x.to_u128().unwrap();
        let idx = (xu - self.low_u) as usize;

        Ok(self.flags[idx])
    }

    // 範囲内の素数列挙
    pub fn primes(&self) -> Vec<T> {
        let mut res = Vec::new();
        for (i, ok) in self.flags.iter().enumerate() {
            if *ok {
                res.push(self.low + T::from_usize(i).unwrap());
            }
        }
        res
    }

    // 範囲内に何個素数があるかをカウントする
    pub fn count(&self) -> usize {
        self.flags.iter().filter(|&&b| b).count()
    }
}
