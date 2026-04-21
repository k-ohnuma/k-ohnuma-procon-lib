
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
        parent_v: &Vec<Option<usize>>,
        parent_e: &Vec<Option<usize>>,
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
