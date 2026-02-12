use std::collections::VecDeque;
use itertools::Itertools;

pub struct Rerooting<T, W, F, G, H>
where
    // identityの型。
    T: Clone,
    // コストWの型。
    W: Clone,
    // マージ関数
    F: FnMut(T, T) -> T,
    // 頂点vへの作用関数。
    G: FnMut(T, usize) -> T,
    // コストの振る舞い関数
    H: FnMut(T, W) -> T,
{
    n: usize,
    to: Vec<Vec<usize>>,
    w: Vec<Vec<W>>,
    rev: Vec<Vec<usize>>,
    identity: T,

    merge: F,
    op_node: G,
    op_edge: H,

    parent: Vec<usize>,
    // 中に入ってるのは頂点番号。この順にたどると全ての部分木に対して必ず親 -> 子の順で出現することが保証されている
    // 逆に辿れば子 -> 親が保証されている
    order: Vec<usize>,
    dp: Vec<Vec<T>>,
}

impl<T, W, F, G, H> Rerooting<T, W, F, G, H>
where
    T: Clone,
    W: Clone,
    F: FnMut(T, T) -> T,
    G: FnMut(T, usize) -> T,
    H: FnMut(T, W) -> T,
{
    pub fn new(
        n: usize,
        edges: impl IntoIterator<Item = (usize, usize, W)>,
        identity: T,
        merge: F,
        op_node: G,
        op_edge: H,
    ) -> Self {
        let mut to = vec![vec![]; n];
        let mut w = vec![vec![]; n];
        let mut rev = vec![vec![]; n];

        for (u, v, ww) in edges {
            let iu = to[u].len();
            let iv = to[v].len();

            to[u].push(v);
            w[u].push(ww.to_owned());
            rev[u].push(iv);

            to[v].push(u);
            w[v].push(ww);
            rev[v].push(iu);
        }

        let dp = (0..n)
            .map(|i| vec![identity.to_owned(); to[i].len()])
            .collect_vec();

        Self {
            n,
            to,
            w,
            rev,
            identity,
            merge,
            op_node,
            op_edge,
            parent: vec![0; n],
            order: vec![0; n],
            dp,
        }
    }

    pub fn dp(&self) -> &Vec<Vec<T>> {
        &self.dp
    }

    pub fn run(&mut self, root: usize) -> Vec<T> {
        if self.n == 0 {
            return vec![];
        }
        if self.n == 1 {
            return vec![(self.op_node)(self.identity.clone(), 0)];
        }

        self.build_order(root);
        self.progate_up(root);
        self.progate_down(root);

        let mut res = vec![self.identity.clone(); self.n];
        for v in 0..self.n {
            let mut accum = self.identity.clone();
            for x in self.dp[v].iter().cloned() {
                accum = (self.merge)(accum, x);
            }
            res[v] = (self.op_node)(accum, v);
        }
        res
    }

    fn build_order(&mut self, root: usize) {
        self.parent[root] = usize::MAX;
        let mut stack = VecDeque::new();
        stack.push_back(root);
        let mut cnt = 0usize;

        while !stack.is_empty() {
            let v = stack.pop_front().unwrap();
            self.order[cnt] = v;
            cnt += 1usize;
            for &v2 in self.to[v].iter() {
                if v2 == self.parent[v] {
                    continue;
                }
                self.parent[v2] = v;
                stack.push_back(v2);
            }
        }
    }

    fn progate_up(&mut self, root: usize) {
        for idx in (0..self.n).rev() {
            let v = self.order[idx];
            let p = self.parent[v];

            if v == root {
                continue;
            }

            let mut accum = self.identity.clone();
            let mut p_idx = usize::MAX;

            for k in 0..self.to[v].len() {
                if self.to[v][k] == p {
                    p_idx = k;
                } else {
                    accum = (self.merge)(accum, self.dp[v][k].to_owned());
                }
            }

            let at_v = (self.op_node)(accum, v);
            let msg_to_p = (self.op_edge)(at_v, self.w[v][p_idx].to_owned());

            let p_slot = self.rev[v][p_idx];
            self.dp[p][p_slot] = msg_to_p;
        }
    }

    fn progate_down(&mut self, _root: usize) {
        for &v in self.order.iter() {
            let deg = self.to[v].len();
            let mut suffix = vec![self.identity.to_owned(); deg];

            for i in (1..deg).rev() {
                suffix[i - 1] = (self.merge)(self.dp[v][i].to_owned(), suffix[i].to_owned());
            }

            let mut pref = self.identity.to_owned();

            for i in 0..deg {
                let nx = self.to[v][i];

                let merged = (self.merge)(pref.to_owned(), suffix[i].to_owned());

                let at_v = (self.op_node)(merged, v);

                let msg_to_nx = (self.op_edge)(at_v, self.w[v][i].to_owned());

                let nx_slot = self.rev[v][i];
                self.dp[nx][nx_slot] = msg_to_nx;

                pref = (self.merge)(pref, self.dp[v][i].to_owned());
            }
        }
    }
}
impl<T, F, G> Rerooting<T, (), F, G, fn(T, ()) -> T>
where
    T: Clone,
    F: FnMut(T, T) -> T,
    G: FnMut(T, usize) -> T,
{
    pub fn new_unweighted(
        n: usize,
        edges: impl IntoIterator<Item = (usize, usize)>,
        identity: T,
        merge: F,
        op_node: G,
    ) -> Self {
        let op_edge: fn(T, ()) -> T = |x, _| x;

        Self::new(
            n,
            edges.into_iter().map(|(u, v)| (u, v, ())),
            identity,
            merge,
            op_node,
            op_edge,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Rerooting;
    use rand::Rng;

    fn gen_tree(n: usize, rng: &mut impl Rng) -> Vec<(usize, usize)> {
        // ランダムな木：各 i(1..n-1) を [0..i-1] に繋ぐ
        let mut edges = vec![];
        for i in 1..n {
            let p = rng.random_range(0..i);
            edges.push((i, p));
        }
        edges
    }

    fn brute_sum_dist(n: usize, edges: &[(usize, usize, i64)]) -> Vec<i64> {
        let mut g = vec![vec![]; n];
        for &(u, v, w) in edges {
            g[u].push((v, w));
            g[v].push((u, w));
        }

        let mut res = vec![0i64; n];
        for s in 0..n {
            let mut dist = vec![-1i64; n];
            dist[s] = 0;
            let mut stack = vec![s];
            while let Some(v) = stack.pop() {
                let dv = dist[v];
                for &(nx, w) in &g[v] {
                    if dist[nx] != -1 { continue; }
                    dist[nx] = dv + w;
                    stack.push(nx);
                }
            }
            res[s] = dist.iter().sum();
        }
        res
    }

    #[test]
    fn rerooting_sum_dist_weighted_random() {
        let mut rng = rand::rng();

        for _ in 0..5_000 {
            let n = rng.random_range(1..=25);

            let base = gen_tree(n, &mut rng);
            let edges_w: Vec<(usize, usize, i64)> = base
                .into_iter()
                .map(|(u, v)| {
                    let w = rng.random_range(0i64..=10);
                    (u, v, w)
                })
                .collect();

            let identity = (0i64, 0i64);

            let mut rr = Rerooting::new(
                n,
                edges_w.iter().cloned(),
                identity,
                |a, b| (a.0 + b.0, a.1 + b.1),
                |acc, _v| (acc.0 + 1, acc.1),
                |msg, w| (msg.0, msg.1 + msg.0 * w),
            );

            let got_pair = rr.run(0);
            let got: Vec<i64> = got_pair.into_iter().map(|x| x.1).collect();

            let expected = brute_sum_dist(n, &edges_w);

            assert_eq!(got, expected);
        }
    }

    #[test]
    fn rerooting_sum_dist_unweighted_random() {
        let mut rng = rand::rng();

        for _ in 0..5_000 {
            let n = rng.random_range(1..=30);
            let base = gen_tree(n, &mut rng);

            let edges_w: Vec<(usize, usize, i64)> = base.iter().cloned().map(|(u, v)| (u, v, 1)).collect();
            let expected = brute_sum_dist(n, &edges_w);

            let identity = (0i64, 0i64);
            let mut rr = Rerooting::new_unweighted(
                n,
                base.into_iter(),
                identity,
                |a, b| (a.0 + b.0, a.1 + b.1),
                |acc, _v| (acc.0 + 1, acc.1),
            );

            let mut rr2 = Rerooting::new_unweighted(
                n,
                edges_w.iter().map(|&(u,v,_)| (u,v)),
                0i64,
                |a,b| a+b,
                |acc,_v| acc+1,
            );
            let got = rr2.run(0);
            assert!(got.iter().all(|&x| x == n as i64));
        }
    }
}

