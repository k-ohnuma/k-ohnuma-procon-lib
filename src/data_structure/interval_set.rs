use std::{
    cmp::{max, min, Ordering},
    collections::BTreeSet,
    fmt::Debug,
};

// use num_traits::Bounded;
pub trait Bounded {
    fn max_value() -> Self;
}
impl Bounded for usize {
    fn max_value() -> Self {
        usize::MAX
    }
}

#[derive(Clone, Debug)]
pub struct Node<T, V> {
    pub l: T,
    pub r: T,
    pub val: V,
}

impl<T: Ord, V> PartialEq for Node<T, V> {
    fn eq(&self, other: &Self) -> bool {
        self.l == other.l && self.r == other.r
    }
}
impl<T: Ord, V> Eq for Node<T, V> {}

impl<T: Ord, V> PartialOrd for Node<T, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord, V> Ord for Node<T, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.l.cmp(&other.l) {
            Ordering::Equal => self.r.cmp(&other.r),
            o => o,
        }
    }
}

pub struct IntervalSet<T, V> {
    identity: V,
    set: BTreeSet<Node<T, V>>,
}

impl<T, V> IntervalSet<T, V>
where
    T: Ord + Copy + Debug + Bounded,
    V: Clone + PartialEq + Default + Debug,
{
    pub fn new(identity: V) -> Self {
        Self {
            identity,
            set: BTreeSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node<T, V>> {
        self.set.iter()
    }

    pub fn get_node(&self, p: T) -> Option<&Node<T, V>> {
        let key = Node {
            l: p,
            r: T::max_value(),
            val: V::default(),
        };
        let mut range = self.set.range(..=key);
        if let Some(node) = range.next_back() {
            if node.l <= p && p < node.r {
                return Some(node);
            }
        }
        None
    }

    /// p を含む区間があればそれ。なければ直後の区間。
    pub fn next_p(&self, p: T) -> Option<&Node<T, V>> {
        if let Some(n) = self.get_node(p) {
            return Some(n);
        }
        let key = Node {
            l: p,
            r: p,
            val: V::default(),
        };
        self.set.range(key..).next()
    }

    pub fn covered_point(&self, p: T) -> bool {
        self.get_node(p).is_some()
    }

    pub fn covered_range(&self, l: T, r: T) -> bool {
        assert!(l <= r);
        if l == r {
            return true;
        }
        if let Some(node) = self.get_node(l) {
            r <= node.r
        } else {
            false
        }
    }

    /// [l, r) が、値に関係なく「隙間なく」被覆されているか
    pub fn covered_range_all(&self, l: T, r: T) -> bool {
        assert!(l <= r);
        if l == r {
            return true;
        }

        let mut cur = l;
        while cur < r {
            let Some(node) = self.get_node(cur) else {
                return false;
            };
            if node.r <= cur {
                return false;
            }
            cur = node.r;
        }
        true
    }

    pub fn same(&self, p: T, q: T) -> bool {
        let h1 = self.get_node(p);
        let h2 = self.get_node(q);
        match (h1, h2) {
            (Some(a), Some(b)) => a.l == b.l && a.r == b.r,
            _ => false,
        }
    }

    pub fn get_val(&self, p: T) -> &V {
        if let Some(node) = self.get_node(p) {
            &node.val
        } else {
            &self.identity
        }
    }

    /// mex(>= p)
    pub fn mex(&self, p: T) -> T {
        let key = Node {
            l: p,
            r: T::max_value(),
            val: V::default(),
        };
        let mut range = self.set.range(..=key);
        if let Some(node) = range.next_back() {
            if node.l <= p && p < node.r {
                return node.r;
            }
        }
        p
    }

    /// f(is_add, l, r, &val)
    ///   is_add = true  → [l,r) が「新しく追加された」(add)
    ///   is_add = false → [l,r) が「消えた」(del)
    pub fn update<F>(&mut self, mut l: T, mut r: T, val: V, mut f: F)
    where
        F: FnMut(bool, T, T, &V),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        let mut to_process: Vec<Node<T, V>> = Vec::new();

        if let Some(node) = self.get_node(l) {
            if node.l < r {
                to_process.push(node.clone());
            }
        }
        let key = Node {
            l,
            r: l,
            val: V::default(),
        };
        for node in self.set.range(key..) {
            if node.l >= r {
                break;
            }
            to_process.push(node.clone());
        }

        for node in to_process.iter() {
            if self.set.remove(node) {
                // del
                f(false, node.l, node.r, &node.val);

                if node.l < l {
                    let left_l = node.l;
                    let left_r = min(node.r, l);
                    if left_l < left_r {
                        let left_val = node.val.clone();
                        self.set.insert(Node {
                            l: left_l,
                            r: left_r,
                            val: left_val.clone(),
                        });
                        // add
                        f(true, left_l, left_r, &left_val);
                    }
                }
                if node.r > r {
                    let right_l = max(node.l, r);
                    let right_r = node.r;
                    if right_l < right_r {
                        let right_val = node.val.clone();
                        self.set.insert(Node {
                            l: right_l,
                            r: right_r,
                            val: right_val.clone(),
                        });
                        // add
                        f(true, right_l, right_r, &right_val);
                    }
                }
            }
        }

        let left_key = Node {
            l,
            r: l,
            val: V::default(),
        };
        if let Some(prev) = self.set.range(..left_key).next_back().cloned() {
            if prev.r == l && prev.val == val && self.set.remove(&prev) {
                // del
                f(false, prev.l, prev.r, &prev.val);
                l = prev.l;
            }
        }

        let right_key = Node {
            l: r,
            r,
            val: V::default(),
        };
        if let Some(next) = self.set.range(right_key..).next().cloned() {
            if next.l == r && next.val == val && self.set.remove(&next) {
                // del
                f(false, next.l, next.r, &next.val);
                r = next.r;
            }
        }

        if l < r {
            self.set.insert(Node {
                l,
                r,
                val: val.clone(),
            });
            f(true, l, r, &val);
        }
    }

    pub fn erase<F>(&mut self, l: T, r: T, mut f: F)
    where
        F: FnMut(bool, T, T, &V),
    {
        assert!(l <= r);
        if l == r {
            return;
        }

        let mut to_process: Vec<Node<T, V>> = Vec::new();

        if let Some(node) = self.get_node(l) {
            if node.l < r {
                to_process.push(node.clone());
            }
        }
        let key = Node {
            l,
            r: l,
            val: V::default(),
        };
        for node in self.set.range(key..) {
            if node.l >= r {
                break;
            }
            to_process.push(node.clone());
        }

        for node in to_process.iter() {
            if self.set.remove(node) {
                // del
                f(false, node.l, node.r, &node.val);

                // 左のかけら
                if node.l < l {
                    let left_l = node.l;
                    let left_r = min(node.r, l);
                    if left_l < left_r {
                        let left_val = node.val.clone();
                        self.set.insert(Node {
                            l: left_l,
                            r: left_r,
                            val: left_val.clone(),
                        });
                        // add
                        f(true, left_l, left_r, &left_val);
                    }
                }
                // 右のかけら
                if node.r > r {
                    let right_l = max(node.l, r);
                    let right_r = node.r;
                    if right_l < right_r {
                        let right_val = node.val.clone();
                        self.set.insert(Node {
                            l: right_l,
                            r: right_r,
                            val: right_val.clone(),
                        });
                        // add
                        f(true, right_l, right_r, &right_val);
                    }
                }
            }
        }
    }

    pub fn update_simple(&mut self, l: T, r: T, val: V) {
        self.update(l, r, val, |_, _, _, _| {});
    }

    pub fn insert_simple(&mut self, l: T, r: T) {
        self.update(l, r, V::default(), |_, _, _, _| {});
    }

    pub fn erase_simple(&mut self, l: T, r: T) {
        self.erase(l, r, |_, _, _, _| {});
    }
}

#[cfg(test)]
mod tests {
    use super::IntervalSet;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn expected_nodes(a: &[Option<i32>]) -> Vec<(usize, usize, i32)> {
        let mut res = Vec::new();
        let mut i = 0usize;
        while i < a.len() {
            let Some(v) = a[i] else {
                i += 1;
                continue;
            };
            let l = i;
            while i < a.len() && a[i] == Some(v) {
                i += 1;
            }
            res.push((l, i, v));
        }
        res
    }

    fn find_node(nodes: &[(usize, usize, i32)], p: usize) -> Option<(usize, usize, i32)> {
        nodes.iter().copied().find(|&(l, r, _)| l <= p && p < r)
    }

    fn next_node(nodes: &[(usize, usize, i32)], p: usize) -> Option<(usize, usize, i32)> {
        if let Some(cur) = find_node(nodes, p) {
            return Some(cur);
        }
        nodes.iter().copied().find(|&(l, _, _)| l >= p)
    }

    #[test]
    fn random_matches_naive() {
        let mut rng = StdRng::seed_from_u64(20260421);
        const ID: i32 = -1;

        for _case in 0..120 {
            let n = rng.random_range(0..=80usize);
            let mut st = IntervalSet::<usize, i32>::new(ID);
            let mut a: Vec<Option<i32>> = vec![None; n];

            for _ in 0..220 {
                let op = rng.random_range(0..=2usize);
                let l = rng.random_range(0..=n);
                let r = rng.random_range(l..=n);
                let v = rng.random_range(-3..=3);

                match op {
                    0 => {
                        st.update_simple(l, r, v);
                        for x in a.iter_mut().take(r).skip(l) {
                            *x = Some(v);
                        }
                    }
                    1 => {
                        st.erase_simple(l, r);
                        for x in a.iter_mut().take(r).skip(l) {
                            *x = None;
                        }
                    }
                    _ => {
                        st.update(l, r, v, |_, _, _, _| {});
                        for x in a.iter_mut().take(r).skip(l) {
                            *x = Some(v);
                        }
                    }
                }

                let nodes = expected_nodes(&a);
                let got_nodes: Vec<(usize, usize, i32)> =
                    st.iter().map(|nd| (nd.l, nd.r, nd.val)).collect();
                assert_eq!(got_nodes, nodes, "n={n} op={op} l={l} r={r} v={v}");

                for (p, ap) in a.iter().enumerate().take(n) {
                    let got_cov = st.covered_point(p);
                    let exp_cov = ap.is_some();
                    assert_eq!(got_cov, exp_cov, "covered_point mismatch p={p}");

                    let got_val = *st.get_val(p);
                    let exp_val = ap.unwrap_or(ID);
                    assert_eq!(got_val, exp_val, "get_val mismatch p={p}");

                    let got_mex = st.mex(p);
                    let exp_mex = if let Some((_, rr, _)) = find_node(&nodes, p) {
                        rr
                    } else {
                        p
                    };
                    assert_eq!(got_mex, exp_mex, "mex mismatch p={p}");

                    let got_next = st.next_p(p).map(|nd| (nd.l, nd.r, nd.val));
                    let exp_next = next_node(&nodes, p);
                    assert_eq!(got_next, exp_next, "next_p mismatch p={p}");
                }

                let p = n;
                let got_cov = st.covered_point(p);
                assert!(!got_cov, "covered_point mismatch p={p}");

                let got_val = *st.get_val(p);
                assert_eq!(got_val, ID, "get_val mismatch p={p}");

                let got_mex = st.mex(p);
                let exp_mex = if let Some((_, rr, _)) = find_node(&nodes, p) {
                    rr
                } else {
                    p
                };
                assert_eq!(got_mex, exp_mex, "mex mismatch p={p}");

                let got_next = st.next_p(p).map(|nd| (nd.l, nd.r, nd.val));
                let exp_next = next_node(&nodes, p);
                assert_eq!(got_next, exp_next, "next_p mismatch p={p}");

                for _ in 0..80 {
                    let x = rng.random_range(0..=n);
                    let y = rng.random_range(0..=n);

                    let got_same = st.same(x, y);
                    let exp_same = match (find_node(&nodes, x), find_node(&nodes, y)) {
                        (Some(a1), Some(a2)) => (a1.0, a1.1) == (a2.0, a2.1),
                        _ => false,
                    };
                    assert_eq!(got_same, exp_same, "same mismatch x={x} y={y}");

                    let (lq, rq) = if x <= y { (x, y) } else { (y, x) };
                    let got_cov_range = st.covered_range(lq, rq);
                    let exp_cov_range = if lq == rq {
                        true
                    } else {
                        matches!(find_node(&nodes, lq), Some((_, rr, _)) if rq <= rr)
                    };
                    assert_eq!(
                        got_cov_range, exp_cov_range,
                        "covered_range mismatch l={lq} r={rq}"
                    );

                    let got_cov_range_all = st.covered_range_all(lq, rq);
                    let exp_cov_range_all = if lq == rq {
                        true
                    } else {
                        lq < n && rq <= n && a[lq..rq].iter().all(Option::is_some)
                    };
                    assert_eq!(
                        got_cov_range_all, exp_cov_range_all,
                        "covered_range_all mismatch l={lq} r={rq}"
                    );
                }
            }
        }
    }
}
