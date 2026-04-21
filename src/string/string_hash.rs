use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug)]
pub struct StringHash {
    len: usize,

    mod1: u128,
    mod2: u128,

    h1: Vec<u128>,
    h2: Vec<u128>,

    pow1: Vec<u128>,
    pow2: Vec<u128>,
}

impl StringHash {
    const DEFAULT_MOD1: u128 = 1_000_000_007;
    const DEFAULT_MOD2: u128 = 1_000_000_009;

    const DEFAULT_BASE1: u128 = 911382323;
    const DEFAULT_BASE2: u128 = 972663749;

    fn alpha_to_u128(c: u8) -> u128 {
        match c {
            b'A'..=b'Z' => (c - b'A') as u128 + 1,
            b'a'..=b'z' => (c - b'a') as u128 + 27,
            _ => panic!("英字のみ対応"),
        }
    }

    pub fn new(s: &str) -> Self {
        let len = s.len();

        let mod1 = Self::DEFAULT_MOD1;
        let mod2 = Self::DEFAULT_MOD2;

        let base1 = Self::DEFAULT_BASE1 % mod1;
        let base2 = Self::DEFAULT_BASE2 % mod2;

        assert!(base1 != 0 && base1 != 1);
        assert!(base2 != 0 && base2 != 1);

        let mut h1 = vec![0; len + 1];
        let mut h2 = vec![0; len + 1];
        let mut pow1 = vec![1; len + 1];
        let mut pow2 = vec![1; len + 1];

        for i in 1..=len {
            pow1[i] = pow1[i - 1] * base1 % mod1;
            pow2[i] = pow2[i - 1] * base2 % mod2;
        }

        for (i, &b) in s.as_bytes().iter().enumerate() {
            let x = Self::alpha_to_u128(b);
            h1[i + 1] = (h1[i] * base1 + x) % mod1;
            h2[i + 1] = (h2[i] * base2 + x) % mod2;
        }

        Self {
            len,
            mod1,
            mod2,
            h1,
            h2,
            pow1,
            pow2,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn hash_range(&self, range: impl RangeBounds<usize>) -> (u128, u128) {
        let (l, r) = self.normalize_range(range);

        let x1 = (self.h1[r] + self.mod1 - self.h1[l] * self.pow1[r - l] % self.mod1) % self.mod1;
        let x2 = (self.h2[r] + self.mod2 - self.h2[l] * self.pow2[r - l] % self.mod2) % self.mod2;

        (x1, x2)
    }

    /// ハッシュ結合
    /// hash(A+B) = (hA * base^{lenB} + hB)
    pub fn concat_hash(&self, h1: (u128, u128), len2: usize, h2: (u128, u128)) -> (u128, u128) {
        let x1 = (h1.0 * self.pow1[len2] + h2.0) % self.mod1;
        let x2 = (h1.1 * self.pow2[len2] + h2.1) % self.mod2;
        (x1, x2)
    }

    /// LCP: s[i..] と s[j..] の最長共通接頭辞長
    /// O(log N)
    pub fn lcp(&self, i: usize, j: usize) -> usize {
        assert!(i <= self.len && j <= self.len);

        let max_len = (self.len - i).min(self.len - j);

        let mut lo = 0;
        let mut hi = max_len + 1;

        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.hash_range(i..i + mid) == self.hash_range(j..j + mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }
    fn normalize_range(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let start = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l + 1,
            Bound::Unbounded => 0,
        };
        let end_excl = match range.end_bound() {
            Bound::Included(&r) => r + 1,
            Bound::Excluded(&r) => r,
            Bound::Unbounded => self.len,
        };

        assert!(start <= end_excl, "range start > end");
        assert!(end_excl <= self.len, "range end out of bounds");

        (start, end_excl)
    }
}
