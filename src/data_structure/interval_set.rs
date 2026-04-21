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
            if prev.r == l && prev.val == val {
                if self.set.remove(&prev) {
                    // del
                    f(false, prev.l, prev.r, &prev.val);
                    l = prev.l;
                }
            }
        }

        let right_key = Node {
            l: r,
            r,
            val: V::default(),
        };
        if let Some(next) = self.set.range(right_key..).next().cloned() {
            if next.l == r && next.val == val {
                if self.set.remove(&next) {
                    // del
                    f(false, next.l, next.r, &next.val);
                    r = next.r;
                }
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
