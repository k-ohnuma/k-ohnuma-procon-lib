pub struct HLDecomposition {
    n: usize,
    root: usize,
    g: Vec<Vec<usize>>,
    parent: Vec<usize>,
    depth: Vec<usize>,
    heavy: Vec<usize>,
    // 頂点番号の再割り当ての際に使う変数
    t: usize,
    // 頂点番号を再割り当てした際のindex
    vid: Vec<usize>,
    // vidの逆
    inv: Vec<usize>,
    // dfsに入って来たときのtの値
    t_in: Vec<usize>,
    // dfs戻るときのtの値
    t_out: Vec<usize>,
    // heavy edgeの親
    head: Vec<usize>,
    is_built: bool,
}

impl HLDecomposition {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            root: usize::MAX,
            g: vec![vec![]; n],
            parent: vec![usize::MAX; n],
            depth: vec![usize::MAX; n],
            heavy: vec![usize::MAX; n],
            t: 0,
            vid: vec![usize::MAX; n],
            inv: vec![usize::MAX; n],
            t_in: vec![usize::MAX; n],
            t_out: vec![usize::MAX; n],
            head: (0..n).collect::<Vec<usize>>(),
            is_built: false,
        }
    }
    pub fn add(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.g[v].push(u);
    }

    pub fn build(&mut self, root: usize) {
        assert!(self.n > 0);
        self.root = root;
        self.depth[root] = 0;
        self.dfs(root, usize::MAX);
        self.t = 0;
        self.head[root] = root;
        self.dfs_hld(root);
        self.is_built = true;
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        debug_assert!(self.is_built);
        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                u = self.parent[self.head[u]];
            } else {
                v = self.parent[self.head[v]];
            }
        }
        if self.depth[u] < self.depth[v] {
            u
        } else {
            v
        }
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
        debug_assert!(self.is_built);
        let l = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[l]
    }

    /// from → to のパス上で from から times 個進んだ頂点
    pub fn go(&self, from: usize, to: usize, times: usize) -> Option<usize> {
        debug_assert!(self.is_built);
        let d = self.distance(from, to);
        if times > d {
            return None;
        }
        let lc = self.lca(from, to);
        if lc == to {
            return self.ancestor(from, times);
        }
        if lc == from {
            return self.child(from, to, times);
        }
        let d_to_lca = self.distance(from, lc);
        if times < d_to_lca {
            self.ancestor(from, times)
        } else if times == d_to_lca {
            Some(lc)
        } else {
            let rest = times - d_to_lca;
            self.child(lc, to, rest)
        }
    }

    /// u-v パスを (vid 上の) 区間 [l, r] に分解して f(l, r) を呼ぶ
    pub fn foreach<F>(&self, mut u: usize, mut v: usize, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        debug_assert!(self.is_built);
        while self.head[u] != self.head[v] {
            if self.depth[self.head[u]] > self.depth[self.head[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            let hv = self.head[v];
            let l = self.vid[hv];
            let r = self.vid[v];
            f(l, r);
            v = self.parent[hv];
        }

        if self.vid[u] > self.vid[v] {
            std::mem::swap(&mut u, &mut v);
        }
        f(self.vid[u], self.vid[v]);
    }

    // ── getter ──────────────────────────────────────────────────────────
    pub fn g(&self) -> &Vec<Vec<usize>> {
        &self.g
    }
    pub fn parent(&self) -> &Vec<usize> {
        &self.parent
    }
    pub fn depth(&self) -> &Vec<usize> {
        &self.depth
    }
    pub fn heavy(&self) -> &Vec<usize> {
        &self.heavy
    }
    pub fn vid(&self) -> &Vec<usize> {
        &self.vid
    }
    pub fn inv(&self) -> &Vec<usize> {
        &self.inv
    }
    pub fn t_in(&self) -> &Vec<usize> {
        &self.t_in
    }
    pub fn t_out(&self) -> &Vec<usize> {
        &self.t_out
    }
    pub fn head(&self) -> &Vec<usize> {
        &self.head
    }

    // ── internal ────────────────────────────────────────────────────────
    // rootから見た部分木のコスト・深さ最大値・heavy edgeの計算
    fn dfs(&mut self, v: usize, p: usize) -> usize {
        self.parent[v] = p;
        let mut acc = 1;
        let mut m = 0;
        for &v2 in self.g[v].to_owned().iter() {
            if v2 == p {
                continue;
            }
            self.depth[v2] = self.depth[v] + 1;
            let next = self.dfs(v2, v);
            acc += next;
            if next > m {
                m = next;
                self.heavy[v] = v2;
            }
        }
        acc
    }

    fn dfs_hld(&mut self, v: usize) {
        self.vid[v] = self.t;
        self.inv[self.t] = v;
        self.t_in[v] = self.t;
        self.t += 1usize;

        if self.heavy[v] != usize::MAX {
            let next = self.heavy[v];
            self.head[next] = self.head[v];
            self.dfs_hld(next);
        }

        for &v2 in self.g[v].to_owned().iter() {
            if v2 == self.parent[v] || v2 == self.heavy[v] {
                continue;
            }
            self.head[v2] = v2;
            self.dfs_hld(v2);
        }
        self.t_out[v] = self.t;
    }

    // vのk個上の親頂点を返す
    fn ancestor(&self, mut v: usize, mut k: usize) -> Option<usize> {
        while k > 0 {
            let h = self.head[v];
            let d_on_chain = self.depth[v] - self.depth[h];

            if k <= d_on_chain {
                return Some(self.inv[self.vid[v] - k]);
            }

            k -= d_on_chain + 1;
            if self.parent[h] == usize::MAX {
                return None;
            }
            v = self.parent[h];
        }
        Some(v)
    }

    // parent -> childのパスでparentからtimes個だけ進んだ頂点
    fn child(&self, parent: usize, child: usize, times: usize) -> Option<usize> {
        assert!(self.depth[parent] < self.depth[child]);
        let d = self.distance(parent, child);
        if times > d {
            return None;
        }
        let up = d - times;
        self.ancestor(child, up)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn build_random_tree(n: usize, rng: &mut StdRng) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for i in 1..n {
            let p = rng.random_range(0..i);
            edges.push((p, i));
        }
        edges
    }

    fn build_children(parent: &[usize]) -> Vec<Vec<usize>> {
        let n = parent.len();
        let mut ch = vec![vec![]; n];
        for v in 0..n {
            let p = parent[v];
            if p != usize::MAX {
                ch[p].push(v);
            }
        }
        ch
    }

    fn naive_lca(u: usize, v: usize, parent: &[usize], depth: &[usize]) -> usize {
        let mut a = u;
        let mut b = v;
        while depth[a] > depth[b] {
            a = parent[a];
        }
        while depth[b] > depth[a] {
            b = parent[b];
        }
        while a != b {
            a = parent[a];
            b = parent[b];
        }
        a
    }

    fn naive_path(u: usize, v: usize, parent: &[usize], depth: &[usize]) -> Vec<usize> {
        let l = naive_lca(u, v, parent, depth);
        let mut left = vec![];
        let mut x = u;
        while x != l {
            left.push(x);
            x = parent[x];
        }
        left.push(l);

        let mut right = vec![];
        let mut y = v;
        while y != l {
            right.push(y);
            y = parent[y];
        }
        right.reverse();

        left.extend(right);
        left
    }

    fn is_ancestor(a: usize, b: usize, parent: &[usize]) -> bool {
        let mut x = b;
        while x != usize::MAX {
            if x == a {
                return true;
            }
            x = parent[x];
        }
        false
    }

    #[test]
    fn random_compare_with_naive() {
        let mut rng = StdRng::seed_from_u64(20251220);

        for _case in 0..200 {
            let n = rng.random_range(2..=200);
            let edges = build_random_tree(n, &mut rng);
            let root = rng.random_range(0..n);

            let mut hld = HLDecomposition::new(n);
            for (u, v) in edges {
                hld.add(u, v);
            }
            hld.build(root);

            for v in 0..n {
                assert_ne!(hld.depth[v], usize::MAX);
                assert_ne!(hld.vid[v], usize::MAX);
                assert_ne!(hld.t_in[v], usize::MAX);
                assert_ne!(hld.t_out[v], usize::MAX);
            }

            let parent = hld.parent.clone();
            let depth = hld.depth.clone();

            for _ in 0..1000 {
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);

                // lca
                let l1 = hld.lca(u, v);
                let l2 = naive_lca(u, v, &parent, &depth);
                assert_eq!(l1, l2, "lca mismatch u={u} v={v}");

                // distance
                let d1 = hld.distance(u, v);
                let path = naive_path(u, v, &parent, &depth);
                let d2 = path.len() - 1;
                assert_eq!(d1, d2, "dist mismatch u={u} v={v}");

                // go
                let dist = d1;
                let t = rng.random_range(0..=dist);
                let g1 = hld.go(u, v, t).unwrap();
                let g2 = path[t];
                assert_eq!(g1, g2, "go mismatch u={u} v={v} t={t}");

                // go over-range
                assert!(hld.go(u, v, dist + 1).is_none());
            }
        }
    }

    #[test]
    fn foreach_covers_path_exactly() {
        let mut rng = StdRng::seed_from_u64(7);

        for _case in 0..100 {
            let n = rng.random_range(2..=250);
            let edges = build_random_tree(n, &mut rng);
            let root = 0;

            let mut hld = HLDecomposition::new(n);
            for (u, v) in edges {
                hld.add(u, v);
            }
            hld.build(root);

            let parent = hld.parent.clone();
            let depth = hld.depth.clone();

            for _ in 0..500 {
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);

                let path = naive_path(u, v, &parent, &depth);
                let mut on_path = vec![false; n];
                for &x in &path {
                    on_path[x] = true;
                }

                let mut covered = vec![false; n];
                hld.foreach(u, v, |l, r| {
                    assert!(l <= r);
                    for idx in l..=r {
                        let node = hld.inv[idx];
                        assert!(on_path[node], "foreach produced node not on path");
                        assert!(!covered[node], "node duplicated in segments");
                        covered[node] = true;
                    }
                });

                for &x in &path {
                    assert!(covered[x], "path node not covered");
                }
            }
        }
    }

    #[test]
    fn subtree_interval_property() {
        let mut rng = StdRng::seed_from_u64(123);

        for _case in 0..100 {
            let n = rng.random_range(2..=300);
            let edges = build_random_tree(n, &mut rng);
            let root = rng.random_range(0..n);

            let mut hld = HLDecomposition::new(n);
            for (u, v) in edges {
                hld.add(u, v);
            }
            hld.build(root);

            let parent = hld.parent.clone();
            let children = build_children(&parent);

            for v in 0..n {
                let mut stack = vec![v];
                let mut subtree = vec![false; n];
                subtree[v] = true;
                while let Some(x) = stack.pop() {
                    for &c in &children[x] {
                        subtree[c] = true;
                        stack.push(c);
                    }
                }

                let tin = hld.t_in[v];
                let tout = hld.t_out[v];
                assert!(tin < tout);

                for idx in tin..tout {
                    let node = hld.inv[idx];
                    assert!(subtree[node], "interval contains non-subtree node");
                }
                for node in 0..n {
                    if subtree[node] {
                        let id = hld.vid[node];
                        assert!(tin <= id && id < tout, "subtree node out of interval");
                    }
                }
            }

            for _ in 0..1000 {
                let a = rng.random_range(0..n);
                let b = rng.random_range(0..n);
                if is_ancestor(a, b, &parent) {
                    assert!(hld.t_in[a] <= hld.t_in[b] && hld.t_in[b] < hld.t_out[a]);
                }
            }
        }
    }
}
