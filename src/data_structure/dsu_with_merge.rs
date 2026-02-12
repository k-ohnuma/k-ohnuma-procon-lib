use ac_library::Dsu;

pub trait Merge {
    /// small-to-large の比較に使う「重さ」
    fn weight(&self) -> usize;

    /// small を self(big) に吸収する。small は空になってOK。
    fn absorb(&mut self, small: &mut Self);
}

/// DSUを small-to-large で管理
pub struct DsuWithMerge<B: Merge> {
    uf: Dsu,
    bag: Vec<B>,
    bag_of: Vec<usize>,
}

impl<B: Merge> DsuWithMerge<B> {
    pub fn new(n: usize, mut init: impl FnMut(usize) -> B) -> Self {
        let mut bag = Vec::with_capacity(n);
        for i in 0..n {
            bag.push(init(i));
        }
        Self {
            uf: ac_library::Dsu::new(n),
            bag,
            bag_of: (0..n).collect(),
        }
    }

    #[inline]
    pub fn leader(&mut self, v: usize) -> usize {
        self.uf.leader(v)
    }

    #[inline]
    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.uf.same(u, v)
    }

    /// unite して bag も small-to-large で統合
    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        if self.uf.same(u, v) {
            return self.uf.leader(u);
        }
        let lu = self.uf.leader(u);
        let lv = self.uf.leader(v);

        let newl = self.uf.merge(lu, lv);

        let bu = self.bag_of[lu];
        let bv = self.bag_of[lv];

        let (big, small) = if self.bag[bu].weight() >= self.bag[bv].weight() {
            (bu, bv)
        } else {
            (bv, bu)
        };

        if big != small {
            let (big_ref, small_ref) = if big < small {
                let (l, r) = self.bag.split_at_mut(small);
                (&mut l[big], &mut r[0])
            } else {
                let (l, r) = self.bag.split_at_mut(big);
                (&mut r[0], &mut l[small])
            };
            big_ref.absorb(small_ref);
        }

        // 新leader は big の箱を参照
        self.bag_of[newl] = big;
        newl
    }

    /// v の属する成分の bag を参照
    #[inline]
    pub fn bag(&mut self, v: usize) -> &B {
        let l = self.uf.leader(v);
        let id = self.bag_of[l];
        &self.bag[id]
    }

    /// v の属する成分の bag をミュータブルに参照（クエリ更新したい時用）
    #[inline]
    pub fn bag_mut(&mut self, v: usize) -> &mut B {
        let l = self.uf.leader(v);
        let id = self.bag_of[l];
        &mut self.bag[id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[derive(Clone, Debug)]
    struct Bag {
        v: Vec<usize>,
    }
    impl Merge for Bag {
        fn weight(&self) -> usize {
            self.v.len()
        }
        fn absorb(&mut self, small: &mut Self) {
            self.v.append(&mut small.v);
        }
    }

    #[derive(Clone, Debug)]
    struct Naive {
        parent: Vec<usize>,
    }
    impl Naive {
        fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
            }
        }
        fn leader(&self, x: usize) -> usize {
            let mut x = x;
            while self.parent[x] != x {
                x = self.parent[x];
            }
            x
        }
        fn same(&self, a: usize, b: usize) -> bool {
            self.leader(a) == self.leader(b)
        }
        fn merge(&mut self, a: usize, b: usize) {
            let ra = self.leader(a);
            let rb = self.leader(b);
            if ra == rb {
                return;
            }
            self.parent[rb] = ra;
        }
        fn component(&self, x: usize) -> Vec<usize> {
            let rx = self.leader(x);
            let mut res = vec![];
            for i in 0..self.parent.len() {
                if self.leader(i) == rx {
                    res.push(i);
                }
            }
            res.sort();
            res
        }
    }

    fn sorted(mut a: Vec<usize>) -> Vec<usize> {
        a.sort();
        a
    }

    #[test]
    fn dsu_with_merge_random_components_match() {
        let mut rng = rand::rng();

        for _case in 0..2000 {
            let n = rng.random_range(1..=60);

            let mut dsu = DsuWithMerge::new(n, |i| Bag { v: vec![i] });
            let mut naive = Naive::new(n);

            let q = rng.random_range(1..=300);
            for _ in 0..q {
                let t = rng.random_range(0..3);
                let a = rng.random_range(0..n);
                let b = rng.random_range(0..n);

                match t {
                    0 | 1 => {
                        dsu.merge(a, b);
                        naive.merge(a, b);
                    }
                    _ => {
                        assert_eq!(dsu.same(a, b), naive.same(a, b));
                    }
                }

                for _ in 0..5 {
                    let v = rng.random_range(0..n);
                    let got = {
                        let mut x = dsu.bag(v).v.clone();
                        x.sort();
                        x
                    };
                    let exp = naive.component(v);
                    assert_eq!(got, exp);
                }
            }
        }
    }

    #[test]
    fn merge_idempotent() {
        let n = 10;
        let mut dsu = DsuWithMerge::new(n, |i| Bag { v: vec![i] });

        let l1 = dsu.merge(3, 7);
        let l2 = dsu.merge(3, 7);
        assert_eq!(dsu.leader(3), l1);
        assert_eq!(dsu.leader(7), l1);
        assert_eq!(l2, l1);

        let got = sorted(dsu.bag(3).v.clone());
        assert_eq!(got, vec![3, 7]);
    }

    #[test]
    fn bag_mut_updates_are_visible() {
        #[derive(Clone, Debug)]
        struct SumBag {
            sum: i64,
            cnt: usize,
        }
        impl Merge for SumBag {
            fn weight(&self) -> usize {
                self.cnt
            }
            fn absorb(&mut self, small: &mut Self) {
                self.sum += small.sum;
                self.cnt += small.cnt;
                small.sum = 0;
                small.cnt = 0;
            }
        }

        let mut dsu = DsuWithMerge::new(5, |i| SumBag {
            sum: i as i64,
            cnt: 1,
        });

        dsu.merge(0, 1);
        dsu.merge(3, 4);

        dsu.bag_mut(1).sum += 10;

        dsu.merge(0, 2);
        dsu.merge(0, 3);
        let total = dsu.bag(0).sum;
        assert_eq!(total, 20);
    }
}
