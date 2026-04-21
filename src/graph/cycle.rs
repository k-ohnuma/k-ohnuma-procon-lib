pub struct Graph {
    pub n: usize,
    // (to, edge_id)
    pub adj: Vec<Vec<(usize, usize)>>,
    pub edges: Vec<Edge>,
    directed: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
}

impl Edge {
    pub fn new(u: usize, v: usize) -> Self {
        Self { u, v }
    }
}

#[derive(Debug, Clone)]
pub struct Cycle {
    pub vertices: Vec<usize>,
    pub edge_ids: Vec<usize>,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
            edges: vec![],
            directed,
        }
    }
    pub fn node_count(&self) -> usize {
        self.n
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.n && v < self.n);
        let id = self.edges.len();
        self.edges.push(Edge { u, v });
        self.adj[u].push((v, id));
        if !self.directed {
            self.adj[v].push((u, id));
        }
        id
    }

    pub fn find_cycle(&self) -> Option<Cycle> {
        let n = self.n;

        let mut state = vec![0u8; n];

        let mut parent_v: Vec<Option<usize>> = vec![None; n];
        let mut parent_e: Vec<Option<usize>> = vec![None; n];

        let mut it_idx = vec![0usize; n];

        for start in 0..n {
            if state[start] != 0 {
                continue;
            }

            state[start] = 1;
            parent_v[start] = None;
            parent_e[start] = None;

            let mut stack: Vec<usize> = vec![start];

            while let Some(&v) = stack.last() {
                if it_idx[v] >= self.adj[v].len() {
                    state[v] = 2;
                    stack.pop();
                    continue;
                }

                let (to, eid) = self.adj[v][it_idx[v]];
                it_idx[v] += 1;

                if parent_e[v].is_some() && parent_e[v].unwrap() == eid {
                    continue;
                }

                if state[to] == 0 {
                    state[to] = 1;
                    parent_v[to] = Some(v);
                    parent_e[to] = Some(eid);
                    stack.push(to);
                    continue;
                }

                if state[to] == 1 {
                    return Some(self.reconstruct_cycle(v, to, eid, &parent_v, &parent_e));
                }
            }
        }

        None
    }

    fn reconstruct_cycle(
        &self,
        from: usize,
        to: usize,
        back_eid: usize,
        parent_v: &[Option<usize>],
        parent_e: &[Option<usize>],
    ) -> Cycle {
        let mut chain_vertices: Vec<usize> = vec![from];
        let mut chain_edges: Vec<usize> = Vec::new();

        let mut cur = from;
        while cur != to {
            let p = parent_v[cur].expect("to must be an ancestor in stack");
            let pe = parent_e[cur].expect("edge to parent must exist");
            chain_edges.push(pe);
            chain_vertices.push(p);
            cur = p;
        }
        chain_vertices.reverse();
        chain_edges.reverse();

        let vertices = chain_vertices;
        let mut edge_ids = chain_edges;
        edge_ids.push(back_eid);

        assert_eq!(edge_ids.len(), vertices.len());
        Cycle { vertices, edge_ids }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cycle, Graph};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::VecDeque;

    fn is_valid_cycle_directed(g: &Graph, c: &Cycle) -> bool {
        let k = c.vertices.len();
        if k == 0 || c.edge_ids.len() != k {
            return false;
        }
        for i in 0..k {
            let eid = c.edge_ids[i];
            if eid >= g.edges.len() {
                return false;
            }
            let e = g.edges[eid];
            let u = c.vertices[i];
            let v = c.vertices[(i + 1) % k];
            if e.u != u || e.v != v {
                return false;
            }
        }
        true
    }

    fn is_valid_cycle_undirected(g: &Graph, c: &Cycle) -> bool {
        let k = c.vertices.len();
        if k == 0 || c.edge_ids.len() != k {
            return false;
        }
        for i in 0..k {
            let eid = c.edge_ids[i];
            if eid >= g.edges.len() {
                return false;
            }
            let e = g.edges[eid];
            let u = c.vertices[i];
            let v = c.vertices[(i + 1) % k];
            if !((e.u == u && e.v == v) || (e.u == v && e.v == u)) {
                return false;
            }
        }
        true
    }

    fn has_cycle_directed_naive(n: usize, edges: &[(usize, usize)]) -> bool {
        let mut adj = vec![Vec::new(); n];
        for &(u, v) in edges {
            adj[u].push(v);
        }
        for &(u, v) in edges {
            if u == v {
                return true;
            }
            let mut seen = vec![false; n];
            let mut q = VecDeque::new();
            seen[v] = true;
            q.push_back(v);
            while let Some(x) = q.pop_front() {
                for &nx in &adj[x] {
                    if !seen[nx] {
                        seen[nx] = true;
                        q.push_back(nx);
                    }
                }
            }
            if seen[u] {
                return true;
            }
        }
        false
    }

    fn has_cycle_undirected_naive(n: usize, edges: &[(usize, usize)]) -> bool {
        let mut uf: Vec<usize> = (0..n).collect();

        fn find(uf: &mut [usize], x: usize) -> usize {
            if uf[x] == x {
                x
            } else {
                let r = find(uf, uf[x]);
                uf[x] = r;
                r
            }
        }

        fn unite(uf: &mut [usize], a: usize, b: usize) {
            let ra = find(uf, a);
            let rb = find(uf, b);
            if ra != rb {
                uf[rb] = ra;
            }
        }

        for &(u, v) in edges {
            if u == v {
                return true;
            }
            let ru = find(&mut uf, u);
            let rv = find(&mut uf, v);
            if ru == rv {
                return true;
            }
            unite(&mut uf, u, v);
        }
        false
    }

    #[test]
    fn random_directed_compare_naive_and_validate_cycle() {
        let mut rng = StdRng::seed_from_u64(20260421);

        for _ in 0..2000 {
            let n = rng.random_range(1..=20usize);
            let m = rng.random_range(0..=80usize);

            let mut g = Graph::new(n, true);
            let mut edges = Vec::with_capacity(m);
            for _ in 0..m {
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);
                g.add_edge(u, v);
                edges.push((u, v));
            }

            let got = g.find_cycle();
            let exp = has_cycle_directed_naive(n, &edges);

            assert_eq!(got.is_some(), exp);
            if let Some(c) = got {
                assert!(is_valid_cycle_directed(&g, &c));
            }
        }
    }

    #[test]
    fn random_undirected_compare_naive_and_validate_cycle() {
        let mut rng = StdRng::seed_from_u64(7);

        for _ in 0..2000 {
            let n = rng.random_range(1..=20usize);
            let m = rng.random_range(0..=80usize);

            let mut g = Graph::new(n, false);
            let mut edges = Vec::with_capacity(m);
            for _ in 0..m {
                let u = rng.random_range(0..n);
                let v = rng.random_range(0..n);
                g.add_edge(u, v);
                edges.push((u, v));
            }

            let got = g.find_cycle();
            let exp = has_cycle_undirected_naive(n, &edges);

            assert_eq!(got.is_some(), exp);
            if let Some(c) = got {
                assert!(is_valid_cycle_undirected(&g, &c));
            }
        }
    }
}
