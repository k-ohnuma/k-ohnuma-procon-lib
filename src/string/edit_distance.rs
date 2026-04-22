// Complexity:
// - edit_distance: O(nm) time, O(m) memory
// - is_edit_distance_leq_one: O(n + m)
// - is_edit_distance_leq_k: O(k * min(n, m)) ~ O(k * max(n, m)) time, O(m) memory
//
// Notes:
// - For k == 1, prefer `is_edit_distance_leq_one`, which is simpler and faster.
// - `is_edit_distance_leq_k` uses banded DP.

/// Full edit distance.
/// O(nm) time, O(m) memory.
pub fn edit_distance<T: Eq>(s: &[T], t: &[T]) -> usize {
    let n = s.len();
    let m = t.len();

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    let mut prev = (0..=m).collect::<Vec<_>>();
    let mut curr = vec![0usize; m + 1];

    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[m]
}

/// Whether edit distance is <= 1.
/// O(n + m).
pub fn is_edit_distance_leq_one<T: Eq>(s: &[T], t: &[T]) -> bool {
    let n = s.len();
    let m = t.len();

    if n.abs_diff(m) > 1 {
        return false;
    }

    if n == m {
        let mut diff = 0usize;
        for i in 0..n {
            if s[i] != t[i] {
                diff += 1;
                if diff > 1 {
                    return false;
                }
            }
        }
        return true;
    }

    let (short, long) = if n < m { (s, t) } else { (t, s) };

    let mut i = 0usize;
    let mut j = 0usize;
    let mut used_skip = false;

    while i < short.len() && j < long.len() {
        if short[i] == long[j] {
            i += 1;
            j += 1;
        } else {
            if used_skip {
                return false;
            }
            used_skip = true;
            j += 1;
        }
    }

    true
}

/// Whether edit distance is <= k.
///
/// Uses banded DP:
/// only computes cells with |i - j| <= k.
///
/// O(k * max(n, m)) time in practice, O(m) memory.
pub fn is_edit_distance_leq_k<T: Eq>(s: &[T], t: &[T], k: usize) -> bool {
    if k == 0 {
        return s == t;
    }
    if k == 1 {
        return is_edit_distance_leq_one(s, t);
    }

    let n0 = s.len();
    let m0 = t.len();

    if n0.abs_diff(m0) > k {
        return false;
    }

    let (s, t, n, m) = if n0 <= m0 {
        (s, t, n0, m0)
    } else {
        (t, s, m0, n0)
    };

    let inf = k + 1;
    let mut prev = vec![inf; m + 1];
    let mut curr = vec![inf; m + 1];

    for (j, v) in prev.iter_mut().enumerate().take(m.min(k) + 1) {
        *v = j;
    }

    for i in 1..=n {
        curr.fill(inf);

        let left = i.saturating_sub(k);
        let right = (i + k).min(m);

        if left == 0 {
            curr[0] = i;
        }

        for j in left..=right {
            if j == 0 {
                continue;
            }

            let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };

            let delete = prev[j] + 1;
            let insert = curr[j - 1] + 1;
            let replace_or_match = prev[j - 1] + cost;

            curr[j] = delete.min(insert).min(replace_or_match);
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    prev[m] <= k
}

#[cfg(test)]
mod tests {
    use crate::string::edit_distance::*;

    fn to_chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_edit_distance_basic() {
        assert_eq!(edit_distance::<char>(&[], &[]), 0);
        assert_eq!(edit_distance(&to_chars(""), &to_chars("abc")), 3);
        assert_eq!(edit_distance(&to_chars("abc"), &to_chars("")), 3);
        assert_eq!(edit_distance(&to_chars("abc"), &to_chars("abc")), 0);
        assert_eq!(edit_distance(&to_chars("abc"), &to_chars("axc")), 1);
        assert_eq!(edit_distance(&to_chars("abc"), &to_chars("ab")), 1);
        assert_eq!(edit_distance(&to_chars("ab"), &to_chars("abc")), 1);
        assert_eq!(edit_distance(&to_chars("kitten"), &to_chars("sitting")), 3);
        assert_eq!(
            edit_distance(&to_chars("intention"), &to_chars("execution")),
            5
        );
    }

    #[test]
    fn test_is_edit_distance_leq_one_basic() {
        assert!(is_edit_distance_leq_one::<char>(&[], &[]));
        assert!(is_edit_distance_leq_one(&to_chars("a"), &to_chars("")));
        assert!(is_edit_distance_leq_one(&to_chars(""), &to_chars("a")));
        assert!(is_edit_distance_leq_one(&to_chars("abc"), &to_chars("abc")));
        assert!(is_edit_distance_leq_one(&to_chars("abc"), &to_chars("axc")));
        assert!(is_edit_distance_leq_one(&to_chars("abc"), &to_chars("ab")));
        assert!(is_edit_distance_leq_one(&to_chars("ab"), &to_chars("abc")));

        assert!(!is_edit_distance_leq_one(
            &to_chars("abc"),
            &to_chars("axy")
        ));
        assert!(!is_edit_distance_leq_one(&to_chars("abc"), &to_chars("a")));
        assert!(!is_edit_distance_leq_one(&to_chars("a"), &to_chars("abc")));
        assert!(!is_edit_distance_leq_one(
            &to_chars("abc"),
            &to_chars("cab")
        ));
    }

    #[test]
    fn test_is_edit_distance_leq_k_basic() {
        let cases = vec![
            ("", "", 0, true),
            ("", "a", 0, false),
            ("", "a", 1, true),
            ("abc", "abc", 0, true),
            ("abc", "axc", 1, true),
            ("abc", "axy", 1, false),
            ("abc", "ab", 1, true),
            ("ab", "abc", 1, true),
            ("kitten", "sitting", 2, false),
            ("kitten", "sitting", 3, true),
            ("intention", "execution", 4, false),
            ("intention", "execution", 5, true),
        ];

        for (s, t, k, expected) in cases {
            assert_eq!(
                is_edit_distance_leq_k(&to_chars(s), &to_chars(t), k),
                expected,
                "failed on s={:?}, t={:?}, k={}",
                s,
                t,
                k
            );
        }
    }

    fn gen_all_strings(alphabet: &[char], max_len: usize) -> Vec<Vec<char>> {
        fn dfs(cur: &mut Vec<char>, out: &mut Vec<Vec<char>>, alphabet: &[char], max_len: usize) {
            out.push(cur.clone());
            if cur.len() == max_len {
                return;
            }
            for &c in alphabet {
                cur.push(c);
                dfs(cur, out, alphabet, max_len);
                cur.pop();
            }
        }

        let mut out = vec![];
        let mut cur = vec![];
        dfs(&mut cur, &mut out, alphabet, max_len);
        out
    }

    #[test]
    fn test_exhaustive_small_for_leq_one_and_k() {
        let alphabet = ['a', 'b', 'c'];
        let all = gen_all_strings(&alphabet, 5);

        for s in &all {
            for t in &all {
                let d = edit_distance(s, t);

                assert_eq!(
                    is_edit_distance_leq_one(s, t),
                    d <= 1,
                    "leq_one mismatch: s={:?}, t={:?}, d={}",
                    s,
                    t,
                    d
                );

                for k in 0..=4 {
                    assert_eq!(
                        is_edit_distance_leq_k(s, t, k),
                        d <= k,
                        "leq_k mismatch: s={:?}, t={:?}, d={}, k={}",
                        s,
                        t,
                        d,
                        k
                    );
                }
            }
        }
    }

    #[test]
    fn test_generic_non_char() {
        let s = vec![1, 2, 3, 4];
        let t = vec![1, 9, 3, 4];
        assert_eq!(edit_distance(&s, &t), 1);
        assert!(is_edit_distance_leq_one(&s, &t));
        assert!(is_edit_distance_leq_k(&s, &t, 1));
        assert!(!is_edit_distance_leq_k(&s, &t, 0));
    }

    #[test]
    fn test_many_prefix_suffix_patterns() {
        let cases = vec![
            ("aaaaa", "aaaaa"),
            ("aaaaa", "aaaab"),
            ("aaaaa", "baaaa"),
            ("aaaaa", "aaaa"),
            ("aaaa", "aaaaa"),
            ("aaaaab", "aaaaba"),
            ("abcde", "zabcd"),
            ("abcde", "abcdz"),
            ("xabcd", "abcd"),
            ("abcd", "xabcd"),
            ("abcdx", "abcd"),
            ("abcd", "abcdx"),
        ];

        for (s, t) in cases {
            let s = to_chars(s);
            let t = to_chars(t);
            let d = edit_distance(&s, &t);
            assert_eq!(is_edit_distance_leq_one(&s, &t), d <= 1);
            for k in 0..=3 {
                assert_eq!(is_edit_distance_leq_k(&s, &t, k), d <= k);
            }
        }
    }

    #[test]
    fn random_test_against_full_dp() {
        use rand::Rng;

        let mut rng = rand::rng();

        for _ in 0..20_000 {
            let n = rng.random_range(0..=30);
            let m = rng.random_range(0..=30);

            let s = (0..n)
                .map(|_| rng.random_range(0..4usize))
                .collect::<Vec<_>>();
            let t = (0..m)
                .map(|_| rng.random_range(0..4usize))
                .collect::<Vec<_>>();

            let d = edit_distance(&s, &t);

            assert_eq!(
                is_edit_distance_leq_one(&s, &t),
                d <= 1,
                "random leq_one mismatch: s={:?}, t={:?}, d={}",
                s,
                t,
                d
            );

            for k in 0..=8 {
                assert_eq!(
                    is_edit_distance_leq_k(&s, &t, k),
                    d <= k,
                    "random leq_k mismatch: s={:?}, t={:?}, d={}, k={}",
                    s,
                    t,
                    d,
                    k
                );
            }
        }
    }

    #[test]
    fn random_test_longer_lengths_small_k() {
        use rand::Rng;

        let mut rng = rand::rng();

        for _ in 0..5_000 {
            let n = rng.random_range(0..=200);
            let m = rng.random_range(0..=200);

            let s = (0..n)
                .map(|_| rng.random_range(0..3usize))
                .collect::<Vec<_>>();
            let t = (0..m)
                .map(|_| rng.random_range(0..3usize))
                .collect::<Vec<_>>();

            let d = edit_distance(&s, &t);

            for k in 0..=5 {
                assert_eq!(
                    is_edit_distance_leq_k(&s, &t, k),
                    d <= k,
                    "long random leq_k mismatch: len(s)={}, len(t)={}, d={}, k={}",
                    s.len(),
                    t.len(),
                    d,
                    k
                );
            }
        }
    }
}
